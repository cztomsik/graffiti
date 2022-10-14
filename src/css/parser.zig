const std = @import("std");
const css = @import("../css.zig");
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
        return switch (try self.tokenizer.next()) {
            tag => |t| t,
            else => error.UnexpectedToken,
        };
    }

    pub fn parse(self: *Self, comptime T: type) !T {
        if (comptime std.meta.trait.hasFn("parse")(T)) {
            return T.parse(self);
        }

        return switch (@typeInfo(T)) {
            .Enum => self.parseEnum(T),
            .Float => @floatCast(T, try self.expect(.number)),
            .Union => self.parseUnion(T),
            else => {
                if (comptime isColor(T)) {
                    return parseColor(self, T);
                }

                if (comptime isRect(T)) {
                    return parseRect(self, T);
                }

                @compileError("unknown value type " ++ @typeName(T));
            },
        };
    }

    pub fn parseEnum(self: *Self, comptime T: type) !T {
        const ident = try self.expect(.ident);

        inline for (std.meta.fields(T)) |f| {
            if (css.propNameEql(f.name, ident)) {
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

    pub fn parseColor(self: *Self, comptime T: type) !T {
        const tok = try self.tokenizer.next();

        switch (tok) {
            //.ident => if (named(tok.ident)) |c| return c,
            .function => {
                if (std.mem.eql(u8, tok.function, "rgb")) {
                    @panic("TODO: rgb()");
                }

                if (std.mem.eql(u8, tok.function, "rgba")) {
                    @panic("TODO: rgba()");
                }
            },
            .hash => |s| {
                switch (s.len) {
                    8 => return rgba(T, hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), hex(s[6..8])),
                    6 => return rgba(T, hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), 0xFF),
                    4 => return rgba(T, (hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, (hex(s[3..4])) * 17),
                    3 => return rgba(T, (hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, 0xFF),
                    else => {},
                }
            },
            else => {},
        }

        return error.InvalidColor;
    }

    pub fn parseRect(self: *Self, comptime T: type) !T {
        const V = std.meta.fieldInfo(T, .top).field_type;
        const top = try self.parse(V);
        const right = try self.parse(?V) orelse top;
        const bottom = try self.parse(?V) orelse top;
        const left = try self.parse(?V) orelse right;

        return T{ .top = top, .right = right, .bottom = bottom, .left = left };
    }

    fn hex(s: []const u8) u8 {
        return std.fmt.parseInt(u8, s, 16) catch 0;
    }

    fn rgba(comptime T: type, r: u8, g: u8, b: u8, a: u8) T {
        if (comptime std.meta.fieldInfo(T, .r).field_type == f32) {
            return T{
                .r = @intToFloat(f32, r) / 255.0,
                .g = @intToFloat(f32, g) / 255.0,
                .b = @intToFloat(f32, b) / 255.0,
                .a = @intToFloat(f32, a) / 255.0,
            };
        }

        return T{ .r = r, .g = g, .b = b, .a = a };
    }
};

fn isColor(comptime T: type) bool {
    return std.meta.trait.hasFields(T, .{ "r", "g", "b", "a" });
}

fn isRect(comptime T: type) bool {
    return std.meta.trait.hasFields(T, .{ "top", "right", "left", "bottom" });
}

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

test "Parser.parse(f32)" {
    try expectParse(f32, "1", 1);
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

test "Parser.parse(Color)" {
    const Color = struct { r: u8, g: u8, b: u8, a: u8 };

    try expectParse(Color, "#000000", .{ .r = 0, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#ff0000", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#00ff00", .{ .r = 0, .g = 0xFF, .b = 0, .a = 0xFF });
    try expectParse(Color, "#0000ff", .{ .r = 0, .g = 0, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#000", .{ .r = 0, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#f00", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#fff", .{ .r = 0xFF, .g = 0xFF, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#0000", .{ .r = 0, .g = 0, .b = 0, .a = 0 });
    try expectParse(Color, "#f00f", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });

    // try expectParse(Color, "rgb(0, 0, 0)", Color.BLACK);
    // try expectParse(Color, "rgba(0, 0, 0, 0)", Color.TRANSPARENT);

    // try expectParse(Color, "transparent", Color.TRANSPARENT);
    // try expectParse(Color, "black", Color.BLACK);

    try expectParse(Color, "xxx", error.InvalidColor);
}

test "Parser.parse(Rect)" {
    const Rect = struct { top: f32, right: f32, bottom: f32, left: f32 };

    try expectParse(Rect, "1", .{ .top = 1, .right = 1, .bottom = 1, .left = 1 });
    try expectParse(Rect, "1 2", .{ .top = 1, .right = 2, .bottom = 1, .left = 2 });
    try expectParse(Rect, "1 2 3", .{ .top = 1, .right = 2, .bottom = 3, .left = 2 });
    try expectParse(Rect, "1 2 3 4", .{ .top = 1, .right = 2, .bottom = 3, .left = 4 });

    try expectParse(Rect, "xxx", error.UnexpectedToken);
}
