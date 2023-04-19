const std = @import("std");
const layout = @import("emlay");
const Parser = @import("css/parser.zig").Parser;

// layout enums
pub const Display = layout.Display;
pub const FlexDirection = layout.FlexDirection;
pub const FlexWrap = layout.FlexWrap;
pub const AlignContent = layout.AlignContent;
pub const AlignItems = layout.AlignItems;
pub const AlignSelf = layout.AlignSelf;
pub const JustifyContent = layout.JustifyContent;
pub const Position = layout.Position;

// other enums
pub const Visibility = enum { visible, hidden };
pub const Overflow = enum { visible, hidden, scroll, auto };
pub const OutlineStyle = enum { none, solid };
pub const BorderStyle = enum { none, solid };

// TODO: figure out where to put these
pub const Color = @import("css/color.zig").Color;
pub const Dimension = @import("css/dimension.zig").Dimension;

// TODO: figure out where to put this
pub const BoxShadow = struct {
    x: Dimension,
    y: Dimension,
    blur: Dimension,
    spread: Dimension,
    color: Color,

    pub fn format(self: BoxShadow, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        try writer.print("{}, {}, {}, {}, {}", .{ self.x, self.y, self.blur, self.spread, self.color });
    }
};

// consts
pub const TRANSPARENT: Color = .{ .r = 0, .g = 0, .b = 0, .a = 0 };
pub const CURRENT_COLOR: Color = .{ .r = 0, .g = 0, .b = 0, .a = 255 }; // TODO
pub const ZERO: Dimension = .{ .px = 0 };

pub const Style = struct {
    display: Display = .block,

    // size
    width: Dimension = .auto,
    height: Dimension = .auto,
    min_width: Dimension = .auto,
    min_height: Dimension = .auto,
    max_width: Dimension = .auto,
    max_height: Dimension = .auto,

    // flex
    flex_grow: f32 = 0,
    flex_shrink: f32 = 1,
    flex_basis: Dimension = .auto, // .percent = 0
    flex_direction: FlexDirection = .row,
    flex_wrap: FlexWrap = .no_wrap,

    // align
    align_content: AlignContent = .stretch,
    align_items: AlignItems = .stretch,
    align_self: AlignSelf = .auto,
    justify_content: JustifyContent = .flex_start,

    // padding
    padding_top: Dimension = ZERO,
    padding_right: Dimension = ZERO,
    padding_bottom: Dimension = ZERO,
    padding_left: Dimension = ZERO,

    // margin
    margin_top: Dimension = ZERO,
    margin_right: Dimension = ZERO,
    margin_bottom: Dimension = ZERO,
    margin_left: Dimension = ZERO,

    // position: Position = .relative, // TODO: should be .static
    // top: Dimension = .auto,
    // right: Dimension = .auto,
    // bottom: Dimension = .auto,
    // left: Dimension = .auto,

    visibility: Visibility = .visible,
    opacity: f32 = 1,

    // transform: []const Transform = &.{},

    // overflow
    overflow_x: Overflow = .visible,
    overflow_y: Overflow = .visible,

    // border-radius
    border_top_left_radius: Dimension = ZERO,
    border_top_right_radius: Dimension = ZERO,
    border_bottom_right_radius: Dimension = ZERO,
    border_bottom_left_radius: Dimension = ZERO,

    // box-shadow
    box_shadow: ?BoxShadow = null, // TODO: []const BoxShadow = &.{},

    // outline
    outline_width: Dimension = .{ .px = 3 },
    outline_style: OutlineStyle = .none,
    outline_color: Color = CURRENT_COLOR,

    // background
    background_color: Color = TRANSPARENT,
    // TODO: background_image: []const BackgroundImage = &.{},

    // border
    border_top_width: Dimension = .{ .px = 3 },
    border_top_style: BorderStyle = .none,
    border_top_color: Color = CURRENT_COLOR,
    border_right_width: Dimension = .{ .px = 3 },
    border_right_style: BorderStyle = .none,
    border_right_color: Color = CURRENT_COLOR,
    border_bottom_width: Dimension = .{ .px = 3 },
    border_bottom_style: BorderStyle = .none,
    border_bottom_color: Color = CURRENT_COLOR,
    border_left_width: Dimension = .{ .px = 3 },
    border_left_style: BorderStyle = .none,
    border_left_color: Color = CURRENT_COLOR,

    // text
    // font_family: []const u8 = "sans-serif",
    // font_size: Dimension = .{ .px = 16 },
    // ...
};
