// some props are currently limited to pixels

const std = @import("std");
const Parser = @import("../parser.zig").Parser;
const expectParse = @import("../parser.zig").expectParse;
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
    try expectParse(Px, "0", .{ .px = 0 });
    try expectParse(Px, "10px", .{ .px = 10 });
    try expectParse(Px, "xxx", error.invalid);
}
