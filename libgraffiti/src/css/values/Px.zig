// some props are currently limited to pixels

const std = @import("std");
const Parser = @import("../parser.zig").Parser;

pub const Px = struct {
    px: f32,

    const Self = @This();

    pub fn parse(parser: *Parser) !Self {
        const tok = try parser.tokenizer.next();

        switch (tok) {
            .number => |n| if (n == 0) return Px{ .px = 0 },
            .dimension => |d| {
                if (std.mem.eql(u8, "px", d.unit)) return Px{ .px = d.value };
            },
            else => {},
        }

        return error.invalid;
    }
};

fn expectPx(input: []const u8, expected: Px) !void {
    try std.testing.expectEqual(expected, try Parser.init(std.testing.allocator, input).parse(Px));
}

test "Px.parse()" {
    try expectPx("0", Px{ .px = 0 });
    try expectPx("10px", Px{ .px = 10 });
}
