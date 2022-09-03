const std = @import("std");
const Parser = @import("../parser.zig").Parser;
const Px = @import("./Px.zig").Px;
const Color = @import("./Color.zig").Color;

pub const BoxShadow = struct {
    offset_x: Px,
    offset_y: Px,
    blur: Px,
    spread: Px,
    color: Color,

    const Self = @This();

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

fn expectBoxShadow(input: []const u8, expected: BoxShadow) !void {
    try std.testing.expectEqual(expected, try Parser.init(std.testing.allocator, input).parse(BoxShadow));
}

test "Shadow.parse()" {
    try expectBoxShadow(
        "1px 1px 1px 1px #000",
        BoxShadow{
            .offset_x = .{ .px = 1 },
            .offset_y = .{ .px = 1 },
            .blur = .{ .px = 1 },
            .spread = .{ .px = 1 },
            .color = Color.BLACK,
        },
    );

    // try expectBoxShadow(
    //     "0 0 10px #000",
    //     BoxShadow{
    //         .offset_x = .{ .px = 0 },
    //         .offset_y = .{ .px = 0 },
    //         .blur = .{ .px = 10 },
    //         .spread = .{ .px = 0 },
    //         .color = Color.BLACK,
    //     },
    // );
}
