const std = @import("std");
const Parser = @import("../parser.zig").Parser;
const expectParse = @import("../parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;
const Px = @import("./Px.zig").Px;
const Color = @import("./Color.zig").Color;

pub const BoxShadow = struct {
    offset_x: Px,
    offset_y: Px,
    blur: Px,
    spread: Px,
    color: Color,

    const Self = @This();

    pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("{} {} {} {} {}", .{ self.offset_x, self.offset_y, self.blur, self.spread, self.color });
    }

    pub fn parse(parser: *Parser) !Self {
        return Self{
            .offset_x = try parser.parse(Px),
            .offset_y = try parser.parse(Px),
            .blur = try parser.parse(Px),
            // TODO: opt
            .spread = try parser.parse(Px),
            .color = try parser.parse(Color),
        };
    }
};

test "BoxShadow.format()" {
    try expectFmt("10px 10px 20px 5px rgba(255, 0, 0, 255)", "{}", .{BoxShadow{
        .offset_x = .{ .px = 10 },
        .offset_y = .{ .px = 10 },
        .blur = .{ .px = 20 },
        .spread = .{ .px = 5 },
        .color = Color.RED,
    }});
}

test "BoxShadow.parse()" {
    try expectParse(
        BoxShadow,
        "1px 1px 1px 1px #000",
        .{
            .offset_x = .{ .px = 1 },
            .offset_y = .{ .px = 1 },
            .blur = .{ .px = 1 },
            .spread = .{ .px = 1 },
            .color = Color.BLACK,
        },
    );

    // try expectParse(
    //     BoxShadow,
    //     "0 0 10px #000",
    //     .{
    //         .offset_x = .{ .px = 0 },
    //         .offset_y = .{ .px = 0 },
    //         .blur = .{ .px = 10 },
    //         .spread = .{ .px = 0 },
    //         .color = Color.BLACK,
    //     },
    // );

    try expectParse(BoxShadow, "xxx", error.invalid);
}
