const std = @import("std");
const Tokenizer = @import("tokenizer.zig").Tokenizer;
const Token = @import("tokenizer.zig").Token;

pub const Parser = struct {
    // TODO: parsed things are likely to out-live the parser itself
    // and should be in arena or .deinit() should be called explicitly
    allocator: std.mem.Allocator,
    tokenizer: Tokenizer,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, input: []const u8) Self {
        return .{
            .allocator = allocator,
            .tokenizer = Tokenizer{ .input = input },
        };
    }

    pub fn expect(self: *Self, comptime tag: std.meta.FieldEnum(Token)) !std.meta.fieldInfo(Token, tag).field_type {
        const tok = try self.tokenizer.next();

        if (tok == tag) {
            return @field(tok, @tagName(tag));
        }

        return error.UnexpectedToken;
    }

    pub fn parse(self: *Self, comptime T: type) !T {
        if (comptime std.meta.trait.hasFn("parse")(T)) {
            return T.parse(self);
        }

        return switch (@typeInfo(T)) {
            .Enum => self.parseEnum(T),
            .Union => self.parseUnion(T),
            .Float => @floatCast(T, try self.expect(.number)),
            else => @compileError("unknown value type"),
        };
    }

    pub fn parseEnum(self: *Self, comptime T: type) !T {
        const ident = try self.expect(.ident);

        inline for (std.meta.fields(T)) |f| {
            if (std.mem.eql(u8, ident, cssName(f.name))) {
                return @intToEnum(T, f.value);
            }
        }

        return error.InvalidValue;
    }

    pub fn parseUnion(self: *Self, comptime T: type) !T {
        const tok = try self.tokenizer.next();

        switch (tok) {
            .dimension => |d| inline for (std.meta.fields(T)) |f| {
                if (comptime @typeInfo(f.field_type) == .Float) {
                    if (std.mem.eql(u8, d.unit, f.name)) {
                        return @unionInit(T, f.name, d.value);
                    }
                }
            },
            .ident => |k| inline for (std.meta.fields(T)) |f| {
                if (comptime f.field_type == void) {
                    if (std.mem.eql(u8, k, f.name)) {
                        return @field(T, f.name);
                    }
                }
            },
            else => {},
        }

        return error.InvalidValue;
    }

    pub fn cssName(comptime name: []const u8) []const u8 {
        comptime {
            var buf: [name.len:0]u8 = undefined;
            _ = std.mem.replace(u8, name, "_", "-", &buf);
            buf[buf.len] = 0;
            return &buf;
        }
    }
};

pub fn expectParse(comptime T: type, input: []const u8, expected: anyerror!T) anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();

    var parser = Parser.init(arena.allocator(), input);
    const result = parser.parse(T);

    errdefer std.debug.print(
        "\n== EXPECTED ====\n{any}\n== FOUND =======\n{any}\n== INPUT LEFT ==\n{s}\n================\n",
        .{ expected, result, parser.tokenizer.rest() },
    );

    if (expected) |exp| {
        const parsed = try result;

        if (comptime std.meta.trait.hasFn("eql")(T)) {
            return std.testing.expect(exp.eql(parsed));
        } else {
            return std.testing.expectEqual(exp, parsed);
        }
    } else |err| return std.testing.expectError(err, result);
}

test "Parser.parse(Enum)" {
    const Display = enum { block, @"table-row" };

    try expectParse(Display, "block", .block);
    try expectParse(Display, "table-row", .@"table-row");

    try expectParse(Display, "err", error.InvalidValue);
}

test "Parser.parse(Union)" {
    const Dimension = union(enum) { auto, px: f32, percent: f32 };

    try expectParse(Dimension, "auto", .auto);
    // try expectParse(Dimension, "0", .{ .px = 0 });
    try expectParse(Dimension, "10px", .{ .px = 10 });
    try expectParse(Dimension, "100%", .{ .percent = 100 });

    try expectParse(Dimension, "xxx", error.InvalidValue);
}
