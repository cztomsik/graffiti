const std = @import("std");
const layout = @import("layout.zig");
const nvg = @import("nanovg");

pub const Style = struct {
    display: layout.Display = .flex,
    background_color: nvg.Color = nvg.rgba(0, 0, 0, 0),
    border_radius: f32 = 0,
    outline: ?Outline = null,
    box_shadow: ?BoxShadow = null,
    opacity: f32 = 1,

    width: layout.Dimension = .auto,
    height: layout.Dimension = .auto,
    // min_width: layout.Dimension = .auto,
    // min_height: layout.Dimension = .auto,
    // max_width: layout.Dimension = .auto,
    // max_height: layout.Dimension = .auto,

    padding: Rect(layout.Dimension, .{ .px = 0 }) = .{},
    margin: Rect(layout.Dimension, .{ .px = 0 }) = .{},
    // border: ? = ?,

    // position: layout.Position = .static,
    // top: layout.Dimension = .auto
    // right: layout.Dimension = .auto
    // left: layout.Dimension = .auto
    // bottom: layout.Dimension = .auto

    flex: Flex = .{},
    flex_direction: layout.FlexDirection = .row,
    // flex_wrap: layout.FlexWrap = .no_wrap,

    // align_content: layout.AlignContent = .stretch,
    // align_items: layout.AlignItems = .stretch,
    // align_self: layout.AlignSelf = .auto,
    // justify_content: layout.JustifyContent = .flex_start,

    pub const INLINE = Style{ .display = .@"inline" };

    pub const Flex = struct {
        grow: f32 = 0,
        shrink: f32 = 0,
        basis: layout.Dimension = .auto, // .{ .percent = 0 }
    };

    pub const Outline = struct {
        width: f32 = 3,
        style: enum { none, solid },
        color: nvg.Color,
    };

    pub const BoxShadow = struct {
        x: f32,
        y: f32,
        blur: f32 = 0,
        spread: f32 = 0,
        color: nvg.Color,
    };

    pub fn Rect(comptime T: type, comptime default: T) type {
        return struct {
            top: T = default,
            right: T = default,
            bottom: T = default,
            left: T = default,
        };
    }
};
