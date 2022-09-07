const std = @import("std");
const Tokenizer = @import("tokenizer.zig").Tokenizer;
const Token = @import("tokenizer.zig").Token;

pub const Parser = struct {
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

        return error.invalid;
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

fn testParser(input: []const u8) Parser {
    return Parser.init(std.testing.allocator, input);
}

test "parse enums" {
    const Display = enum { block, @"table-row" };

    try std.testing.expectEqual(Display.block, try testParser("block").parse(Display));
    try std.testing.expectEqual(Display.@"table-row", try testParser("table-row").parse(Display));
    try std.testing.expectError(error.invalid, testParser("err").parse(Display));
}
