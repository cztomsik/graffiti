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

    pub fn expect(self: *Self, comptime tag: @Type(.EnumLiteral)) !std.meta.fieldInfo(Token, tag).field_type {
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

        if (@typeInfo(T) == .Enum) {
            return std.meta.stringToEnum(T, try self.expect(.ident)) orelse return error.invalid;
        }

        return switch (T) {
            f32 => self.expect(.number),
            else => @compileError("unknown value type"),
        };
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

test "parse enums" {
    const Display = enum { block, @"table-row" };

    try expectParse(Display, "block", .block);
    try expectParse(Display, "table-row", .@"table-row");

    try expectParse(Display, "err", error.invalid);
}
