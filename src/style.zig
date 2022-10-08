const layout = @import("layout.zig");
const nvg = @import("nanovg");

pub const Style = struct {
    display: layout.Display = .block,
    background_color: nvg.Color = nvg.rgba(0, 0, 0, 0),
    opacity: f32 = 1,

    width: layout.Dimension = .auto,
    height: layout.Dimension = .auto,
    // min_width: layout.Dimension = .auto,
    // min_height: layout.Dimension = .auto,
    // max_width: layout.Dimension = .auto,
    // max_height: layout.Dimension = .auto,

    padding_top: layout.Dimension = .{ .px = 0 },
    padding_right: layout.Dimension = .{ .px = 0 },
    padding_bottom: layout.Dimension = .{ .px = 0 },
    padding_left: layout.Dimension = .{ .px = 0 },

    // margin_top: layout.Dimension = .{ .px = 0 },
    // margin_right: layout.Dimension = .{ .px = 0 },
    // margin_bottom: layout.Dimension = .{ .px = 0 },
    // margin_left: layout.Dimension = .{ .px = 0 },

    // border_top: layout.Dimension = .{ .px = 0 },
    // border_right: layout.Dimension = .{ .px = 0 },
    // border_bottom: layout.Dimension = .{ .px = 0 },
    // border_left: layout.Dimension = .{ .px = 0 },

    // TODO: position, top, right, left

    // flex_grow: f32 = 0,
    // flex_shrink: f32 = 1,
    // flex_basis: layout.Dimension = .auto,

    // flex_direction: layout.FlexDirection = .row,
    // flex_wrap: layout.FlexWrap = .no_wrap,

    // align_content: layout.AlignContent = .stretch,
    // align_items: layout.AlignItems = .stretch,
    // align_self: layout.AlignSelf = .auto,
    // justify_content: layout.JustifyContent = .flex_start,

    pub const INLINE = Style{ .display = .@"inline" };
};
