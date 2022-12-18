const std = @import("std");
const css = @import("../css.zig");
const Tokenizer = @import("tokenizer.zig").Tokenizer;
const Token = @import("tokenizer.zig").Token;

// comptime snake_case/camelCase to css-case
pub fn cssName(comptime name: []const u8) []const u8 {
    comptime {
        var len = name.len;
        for (name) |ch| {
            if (std.ascii.isUpper(ch)) len += 1;
        }
        var res: [len]u8 = undefined;
        var i = 0;
        for (name) |ch| {
            if (std.ascii.isUpper(ch)) {
                res[i] = '-';
                i += 1;
            }

            res[i] = if (ch == '_') '-' else std.ascii.toLower(ch);
            i += 1;
        }
        return &res;
    }
}

test "cssName" {
    try std.testing.expectEqualSlices(u8, "background-color", cssName("background-color"));
    try std.testing.expectEqualSlices(u8, "background-color", cssName("background_color"));
    try std.testing.expectEqualSlices(u8, "background-color", cssName("backgroundColor"));
    try std.testing.expectEqualSlices(u8, "border-top-left-radius", cssName("borderTopLeftRadius"));
}

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
            .Int => self.parseInt(T),
            .Float => self.parseFloat(T),
            .Enum => self.parseEnum(T),
            .Union => self.parseUnion(T),
            .Optional => self.parseOptional(std.meta.Child(T)),
            .Struct => {
                if (comptime isColor(T)) {
                    return parseColor(self, T);
                }

                if (comptime isRect(T)) {
                    return parseRect(self, T);
                }

                return self.parseStruct(T);
            },
            else => @compileError("type " ++ @typeName(T) ++ " cannot be parsed"),
        };
    }

    pub fn parseInt(self: *Self, comptime T: type) !T {
        const num = try self.expect(.number);
        return std.math.cast(T, @floatToInt(u32, num)) orelse error.InvalidInt;
    }

    pub fn parseFloat(self: *Self, comptime T: type) !T {
        return @floatCast(T, try self.expect(.number));
    }

    pub fn parseEnum(self: *Self, comptime T: type) !T {
        const ident = try self.expect(.ident);

        inline for (std.meta.fields(T)) |f| {
            if (std.mem.eql(u8, cssName(f.name), ident)) {
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

    pub fn parseOptional(self: *Self, comptime T: type) ?T {
        const prev = self.tokenizer;
        return self.parse(T) catch {
            self.tokenizer = prev;
            return null;
        };
    }

    pub fn parseStruct(self: *Self, comptime T: type) !T {
        var res: T = undefined;

        inline for (std.meta.fields(T)) |f| {
            if (f.default_value) |ptr| {
                const v = @ptrCast(*const f.field_type, @alignCast(f.alignment, ptr)).*;
                @field(res, f.name) = self.parseOptional(f.field_type) orelse v;
            } else {
                @field(res, f.name) = try self.parse(f.field_type);
            }
        }

        return res;
    }

    pub fn parseColor(self: *Self, comptime T: type) !T {
        const tok = try self.tokenizer.next();

        switch (tok) {
            //.ident => if (named(tok.ident)) |c| return c,
            .function => {
                if (std.mem.eql(u8, tok.function, "rgb")) {
                    const args = try self.parseArgs(std.meta.Tuple(&.{ u8, u8, u8 }));
                    return T.rgba(args.@"0", args.@"1", args.@"2", 0xFF);
                }

                if (std.mem.eql(u8, tok.function, "rgba")) {
                    const args = try self.parseArgs(std.meta.Tuple(&.{ u8, u8, u8, u8 }));
                    return T.rgba(args.@"0", args.@"1", args.@"2", args.@"3");
                }
            },
            .hash => |s| {
                switch (s.len) {
                    8 => return T.rgba(hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), hex(s[6..8])),
                    6 => return T.rgba(hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), 0xFF),
                    4 => return T.rgba((hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, (hex(s[3..4])) * 17),
                    3 => return T.rgba((hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, 0xFF),
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

    pub fn parseArgs(self: *Self, comptime T: type) !T {
        var res: T = undefined;

        inline for (std.meta.fields(T)) |f, i| {
            if (i > 0) try self.expect(.comma);
            @field(res, f.name) = try self.parse(f.field_type);
        }

        return res;
    }

    fn hex(s: []const u8) u8 {
        return std.fmt.parseInt(u8, s, 16) catch 0;
    }
};

fn isColor(comptime T: type) bool {
    return @hasDecl(T, "rgba");
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

test "Parser.parse(num)" {
    try expectParse(u32, "123", 123);
    try expectParse(u16, "123", 123);
    try expectParse(u8, "123", 123);

    try expectParse(f32, "1.23", 1.23);
    try expectParse(f16, "1.23", 1.23);
    try expectParse(f32, "1", 1);

    try expectParse(u8, "255", 255);
    try expectParse(u8, "256", error.InvalidInt);
}

test "Parser.parse(Enum)" {
    const Display = enum { block, table_row };

    try expectParse(Display, "block", .block);
    try expectParse(Display, "table-row", .table_row);

    try expectParse(Display, "err", error.InvalidValue);
}

test "Parser.parse(Union)" {
    const Dimension = union(enum) { auto, px: f32, percent: f32 };

    try expectParse(Dimension, "auto", .auto);
    // TODO
    // try expectParse(Dimension, "0", .{ .px = 0 });
    try expectParse(Dimension, "10px", .{ .px = 10 });
    try expectParse(Dimension, "100%", .{ .percent = 100 });

    try expectParse(Dimension, "xxx", error.InvalidValue);
}

test "Parser.parse(Color)" {
    const Color = struct {
        r: u8,
        g: u8,
        b: u8,
        a: u8,

        pub fn rgba(r: u8, g: u8, b: u8, a: u8) @This() {
            return .{ .r = r, .g = g, .b = b, .a = a };
        }
    };

    try expectParse(Color, "#000000", .{ .r = 0, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#ff0000", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#00ff00", .{ .r = 0, .g = 0xFF, .b = 0, .a = 0xFF });
    try expectParse(Color, "#0000ff", .{ .r = 0, .g = 0, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#000", .{ .r = 0, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#f00", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "#fff", .{ .r = 0xFF, .g = 0xFF, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#0000", .{ .r = 0, .g = 0, .b = 0, .a = 0 });
    try expectParse(Color, "#f00f", .{ .r = 0xFF, .g = 0, .b = 0, .a = 0xFF });

    try expectParse(Color, "rgb(0, 0, 0)", .{ .r = 0, .g = 0, .b = 0, .a = 0xFF });
    try expectParse(Color, "rgba(0, 0, 0, 0)", .{ .r = 0, .g = 0, .b = 0, .a = 0 });
    try expectParse(Color, "rgba(255, 128, 0, 255)", .{ .r = 0xFF, .g = 0x80, .b = 0, .a = 0xFF });

    // TODO
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

test "Parser.parse(Struct)" {
    const Color = enum { black, blue };
    const Outline = struct { width: f32 = 3, style: enum { none, solid }, color: Color = .black };
    const Shadow = struct { x: f32, y: f32, blur: f32 = 0, spread: f32 = 0, color: Color = .black };

    try expectParse(Outline, "solid", .{ .style = .solid });
    try expectParse(Outline, "1 solid", .{ .width = 1, .style = .solid });
    try expectParse(Outline, "1 solid blue", .{ .width = 1, .style = .solid, .color = .blue });

    try expectParse(Shadow, "1 1", .{ .x = 1, .y = 1 });
    try expectParse(Shadow, "1 1 blue", .{ .x = 1, .y = 1, .color = .blue });
    try expectParse(Shadow, "1 1 1 blue", .{ .x = 1, .y = 1, .blur = 1, .color = .blue });

    try expectParse(Outline, "xxx", error.InvalidValue);
    try expectParse(Shadow, "xxx", error.UnexpectedToken);
}

test "Parser.parse(Optional)" {
    try expectParse(?f32, "", null);
    try expectParse(?f32, "foo", null);
    try expectParse(?f32, "1", 1);
}
