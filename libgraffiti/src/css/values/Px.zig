// some props are currently limited to pixels

const std = @import("std");
const Parser = @import("../parser.zig").Parser;
const expectFmt = std.testing.expectFmt;

pub const Px = struct {
    px: f32,

    const Self = @This();

    pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("{d}px", .{self.px});
    }

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

test "Px.format()" {
    try expectFmt("0px", "{}", .{Px{ .px = 0 }});
}

test "Px.parse()" {
    try expectPx("0", Px{ .px = 0 });
    try expectPx("10px", Px{ .px = 10 });
}
