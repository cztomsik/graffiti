// generic CSS parser

const std = @import("std");
const Tokenizer = @import("tokenizer.zig").Tokenizer;
const Token = @import("tokenizer.zig").Token;

// comptime snake_case to css-case
pub fn cssName(comptime name: []const u8) []const u8 {
    comptime {
        var buf: [name.len]u8 = undefined;
        _ = std.mem.replace(u8, name, "_", "-", &buf);
        return &buf;
    }
}

test "cssName" {
    try std.testing.expectEqualSlices(u8, "background-color", cssName("background_color"));
    try std.testing.expectEqualSlices(u8, "background-color", cssName("background-color"));
}

pub const Parser = struct {
    allocator: std.mem.Allocator,
    tokenizer: Tokenizer,

    /// Initialize a new parser.
    pub fn init(allocator: std.mem.Allocator, input: []const u8) Parser {
        return .{
            .allocator = allocator,
            .tokenizer = Tokenizer{ .input = input },
        };
    }

    /// Expect a token of a specific type and get its payload.
    pub fn expect(self: *Parser, comptime tag: std.meta.FieldEnum(Token)) !std.meta.fieldInfo(Token, tag).type {
        return switch (try self.tokenizer.next()) {
            tag => |t| t,
            else => error.UnexpectedToken,
        };
    }

    pub fn parse(self: *Parser, comptime T: anytype) !T {
        if (comptime std.meta.trait.hasFn("parse")(T)) {
            return T.parse(self);
        }

        return switch (@typeInfo(T)) {
            .Int => self.parseInt(T),
            .Float => self.parseFloat(T),
            .Enum => self.parseEnum(T),
            .Optional => self.parseOptional(std.meta.Child(T)),
            .Struct => return self.parseStruct(T),
            else => @compileError("type " ++ @typeName(T) ++ " cannot be parsed"),
        };
    }

    pub fn parseInt(self: *Parser, comptime T: type) !T {
        const num = try self.expect(.number);
        return std.math.cast(T, @floatToInt(u32, num)) orelse error.InvalidInt;
    }

    pub fn parseFloat(self: *Parser, comptime T: type) !T {
        return @floatCast(T, try self.expect(.number));
    }

    pub fn parseEnum(self: *Parser, comptime T: type) !T {
        const ident = try self.expect(.ident);

        inline for (std.meta.fields(T)) |f| {
            if (std.mem.eql(u8, cssName(f.name), ident)) {
                return @intToEnum(T, f.value);
            }
        }

        return error.InvalidValue;
    }

    pub fn parseOptional(self: *Parser, comptime T: type) ?T {
        const prev = self.tokenizer;
        return self.parse(T) catch {
            self.tokenizer = prev;
            return null;
        };
    }

    /// Parse struct by reading subsequent values for each field. Spaces are
    /// ignored. If a field has a default value, it will be used if the value is
    /// missing.
    pub fn parseStruct(self: *Parser, comptime T: type) !T {
        var res: T = undefined;

        inline for (std.meta.fields(T)) |f| {
            if (f.default_value) |ptr| {
                const v = @ptrCast(*const f.type, @alignCast(f.alignment, ptr)).*;
                @field(res, f.name) = self.parseOptional(f.type) orelse v;
            } else {
                @field(res, f.name) = try self.parse(f.type);
            }
        }

        return res;
    }

    /// Call a function with arguments parsed from the input. Arguments are
    /// separated by commas and spaces are ignored.
    pub fn parseFnCall(self: *Parser, fun: anytype) !@typeInfo(@TypeOf(fun)).Fn.return_type.? {
        const Args = std.meta.ArgsTuple(@TypeOf(fun));
        var args: Args = undefined;
        inline for (std.meta.fields(Args), 0..) |f, i| {
            if (i > 0) try self.expect(.comma);
            @field(args, f.name) = try self.parse(f.type);
        }

        return @call(.auto, fun, args);
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
        return std.testing.expectEqualDeep(exp, try result);
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

test "Parser.parse(Struct)" {
    const Color = enum { black, blue };
    const Outline = struct { width: f32 = 3, style: enum { none, solid }, color: Color = .black };
    const Shadow = struct { x: f32, y: f32, blur: f32 = 0, spread: f32 = 0, color: Color = .black };
    const Packed = packed struct { x: u1, y: u1 };

    try expectParse(Outline, "solid", .{ .style = .solid });
    try expectParse(Outline, "1 solid", .{ .width = 1, .style = .solid });
    try expectParse(Outline, "1 solid blue", .{ .width = 1, .style = .solid, .color = .blue });

    try expectParse(Shadow, "1 1", .{ .x = 1, .y = 1 });
    try expectParse(Shadow, "1 1 blue", .{ .x = 1, .y = 1, .color = .blue });
    try expectParse(Shadow, "1 1 1 blue", .{ .x = 1, .y = 1, .blur = 1, .color = .blue });

    try expectParse(Outline, "xxx", error.InvalidValue);
    try expectParse(Shadow, "xxx", error.UnexpectedToken);

    try expectParse(Packed, "1 1", .{ .x = 1, .y = 1 });
}

test "Parser.parse(Optional)" {
    try expectParse(?f32, "", null);
    try expectParse(?f32, "foo", null);
    try expectParse(?f32, "1", 1);
}

test "Parser.parseFnCall(fn)" {
    const Helper = struct {
        fn sum(a: f32, b: f32) f32 {
            return a + b;
        }
    };

    var parser = Parser.init(std.testing.allocator, "1, 2");
    const res = try parser.parseFnCall(Helper.sum);
    try std.testing.expectEqual(res, 3);
}
