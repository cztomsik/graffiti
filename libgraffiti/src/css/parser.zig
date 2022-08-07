const std = @import("std");
const Tokenizer = @import("tokenizer.zig").Tokenizer;

const Display = enum { block, table_row };

const Style = struct {
    display: Display,
};

pub const Parser = struct {
    allocator: std.mem.Allocator,
    tokenizer: Tokenizer,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, input: []const u8) Self {
        return .{
            .allocator = allocator,
            .tokenizer = Tokenizer.init(input),
        };
    }

    // pub fn parseStyleSheet(self: *Self) !StyleSheet {}
    // pub fn parseStyleRule(self: *Self) !StyleRule {}
    // pub fn parseSelector(self: *Self) !Selector {}
    // pub fn parseStyle(self: *Self) !Style {}

    fn parseValue(comptime T: type) T {
        switch (@typeInfo(T)) {
            .Enum => parseEnum(T),
            // TODO .Struct + inline for
            else => @compileError("unknown value type"),
        }
    }

    fn parseEnum(self: *Self, comptime T: type) T {
        const tok = self.tokenizer.next();

        if (tok.tag == .ident) {
            const tok_str = self.tokenizer.input[tok.start..tok.end];

            inline for (@typeInfo(T).Enum.fields) |f| {
                if (std.mem.eql(u8, cssName(f.name), tok_str)) {
                    return @intToEnum(T, f.value);
                }
            }
        }

        @panic("TODO: err");
    }
};

fn cssName(comptime name: []const u8) []const u8 {
    comptime {
        var buf: [name.len:0]u8 = undefined;
        _ = std.mem.replace(u8, name, "_", "-", &buf);
        buf[buf.len] = 0;
        return &buf;
    }
}

fn testParser(input: []const u8) Parser {
    return Parser.init(std.testing.allocator, input);
}

test "enums" {
    try std.testing.expectEqual(testParser("block").parseEnum(Display), .block);
    try std.testing.expectEqual(testParser("table-row").parseEnum(Display), .table_row);
    // try t.expectError(Error.Err, parseEnum(Display, "err"));

}
