const nvg = @import("nanovg");

pub const Display = enum { none, block, @"inline", flex };

pub const Dimension = union(enum) {
    auto,
    px: f32,
    // percent: f32,
    // ...
};

pub const Color = nvg.Color;

pub const TRANSPARENT = nvg.rgba(0, 0, 0, 0);

pub const Style = struct {
    // TODO: should be inline
    display: Display = Display.block,

    width: Dimension = .auto,
    height: Dimension = .auto,
    // min_width: Dimension,
    // min_height: Dimension,
    // max_width: Dimension,
    // max_height: Dimension,

    padding_top: Dimension = .{ .px = 0 },
    padding_right: Dimension = .{ .px = 0 },
    padding_bottom: Dimension = .{ .px = 0 },
    padding_left: Dimension = .{ .px = 0 },

    // margin_top: Dimension,
    // margin_right: Dimension,
    // margin_bottom: Dimension,
    // margin_left: Dimension,

    // border_top_width: Dimension,
    // border_right_width: Dimension,
    // border_bottom_width: Dimension,
    // border_left_width: Dimension,

    // position: Position,
    // top: Dimension,
    // right: Dimension,
    // bottom: Dimension,
    // left: Dimension,

    // TODO: flex

    opacity: f32 = 1,
    // visibility: Visibility,
    // transform: Transform,

    border_radius: [4]f32 = .{ 0, 0, 0, 0 },
    // border_top_left_radius: Px,
    // border_top_right_radius: Px,
    // border_bottom_right_radius: Px,
    // border_bottom_left_radius: Px,

    background_color: Color = TRANSPARENT,
};
