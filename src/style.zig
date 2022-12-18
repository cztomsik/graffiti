const std = @import("std");
const layout = @import("layout.zig");
const css = @import("css.zig");
const nvg = @import("nanovg");

pub const Style = struct {
    flags: packed struct {
        display: layout.Display = .flex,
    } = .{},
    w: layout.Dimension = .auto,

    const Self = @This();

    pub const cssMapping = .{
        .properties = .{
            .{ "display", display, setDisplay },
            .{ "width", width, setWidth },
        },
    };

    pub fn display(self: *const Self) layout.Display {
        return self.flags.display;
    }

    pub fn setDisplay(self: *Self, value: layout.Display) void {
        self.flags.display = value;
    }

    pub fn width(self: *const Self) layout.Dimension {
        return self.w;
    }

    pub fn setWidth(self: *Self, value: layout.Dimension) void {
        self.w = value;
    }
};

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

// pub const Style = struct {
//     flags: packed struct {
//         display: layout.Display = .flex,
//         // position: layout.Position = .static,
//         // flex_direction: layout.FlexDirection = .row,
//         // flex_wrap: layout.FlexWrap = .no_wrap,
//         // align_content: layout.AlignContent = .stretch,
//         // align_items: layout.AlignItems = .stretch,
//         // align_self: layout.AlignSelf = .auto,
//         // justify_content: layout.JustifyContent = .flex_start,
//         // visibility: Visibility = .visible
//         // overflow: [2]Overflow = .visible,
//     } = .{},

//     // outline
//     // box_shadow
//     // text - family, size, line-height, text-align, color

//     // TODO: min/max
//     // TODO: split tag and f32 value
//     // TODO: technically, size can never be <0 (margin & position is the only exception)
//     // width: layout.Dimension = .auto,
//     // height: layout.Dimension = .auto,
//     // min_width: layout.Dimension = .auto,
//     // min_height: layout.Dimension = .auto,
//     // max_width: layout.Dimension = .auto,
//     // max_height: layout.Dimension = .auto,

//     // padding: Rect(layout.Dimension, .{ .px = 0 }) = .{},
//     // margin: Rect(layout.Dimension, .{ .px = 0 }) = .{},

//     // top: layout.Dimension = .auto
//     // right: layout.Dimension = .auto
//     // left: layout.Dimension = .auto
//     // bottom: layout.Dimension = .auto

//     // flex: Flex = .{},

//     // border_width: [4]u8 = .{ 0, 0, 0, 0 },

//     // border_radius: [4]u8 = .{ 0, 0, 0, 0 },
//     // border_color: [4]Color = undefined,

//     // opacity: f32 = 1, // or just packed 1/1000 precision? we mostly just check for 1 anyway

//     // background_color: Color = Color.TRANSPARENT,
//     // outline: Outline = .{ .style = .none, .color = Color.TRANSPARENT },
//     // box_shadow: ?BoxShadow = null,

//     const Self = @This();

//     pub fn getDisplay(self: *Self) layout.Display {
//         return self.flags.display;
//     }

//     pub fn setDisplay(self: *Self, value: layout.Display) void {
//         self.flags.display = value;
//     }

//     // fn getWidth(self: *Self) layout.Dimension {
//     //     return self.width;
//     // }

//     // fn setWidth(self: *Self, value: layout.Dimension) void {
//     //     self.width = value;
//     // }

//     // fn getHeight(self: *Self) layout.Dimension {
//     //     return self.height;
//     // }

//     // fn setHeight(self: *Self, value: layout.Dimension) void {
//     //     self.height = value;
//     // }

//     // fn getFlexGrow(self: *Self) f32 {
//     //     return self.flex_grow;
//     // }

//     // fn setFlexGrow(self: *Self, value: f32) void {
//     //     self.flex_grow = value;
//     // }

//     // fn getFlexShrink(self: *Self) f32 {
//     //     return self.flex_shrink;
//     // }

//     // fn setFlexShrink(self: *Self, value: f32) void {
//     //     self.flex_shrink = value;
//     // }

//     // fn getBorder(self: *Self) Border {
//     //     return .{ .width = self.border_widths[0], .style = self.border_style[0], .color = self.border_color[0] };
//     // }

//     // fn setBorder(self: *Self, border: Border) void {
//     //     self.setBorderWidth(border.width);
//     //     self.setBorderStyle(border.style);
//     //     self.setBorderColor(border.color);
//     // }

//     // // fn getBorderWidth(self: *Self) void {}
//     // // fn setBorderWidth(self: *Self) void {}

//     // // fn getBorderRightWidth(self: *Self) f32 {}
//     // // fn setBorderRightWidth(self: *Self) void {}

//     // // fn getBorderRadius(self: *Self) void {}
//     // // fn setBorderRadius(self: *Self) void {}
//     // pub const Flex = struct {
//     //     grow: f32 = 0,
//     //     shrink: f32 = 0,
//     //     basis: layout.Dimension = .auto, // .{ .percent = 0 }
//     // };

//     // pub const Outline = struct {
//     //     width: f32 = 3,
//     //     style: enum { none, solid },
//     //     color: Color = Color.TRANSPARENT,
//     // };

//     // pub const BoxShadow = struct {
//     //     x: f32,
//     //     y: f32,
//     //     blur: f32 = 0,
//     //     spread: f32 = 0,
//     //     color: Color = Color.TRANSPARENT,
//     // };

//     // pub fn Rect(comptime T: type, comptime default: T) type {
//     //     return struct {
//     //         top: T = default,
//     //         right: T = default,
//     //         bottom: T = default,
//     //         left: T = default,
//     //     };
//     // }
// };

// // const Color = struct {
// //     r: u8 = 0,
// //     g: u8 = 0,
// //     b: u8 = 0,
// //     a: u8 = 0,

// //     const Self = @This();

// //     const TRANSPARENT: Self = .{};

// //     pub fn rgba(r: u8, g: u8, b: u8, a: u8) Color {
// //         return .{ .r = r, .g = g, .b = b, .a = a };
// //     }
// // };

// // pub const Border = struct { width: layout.Dimension, style: BorderStyle, color: Color };

// // pub const BorderStyle = enum { none, solid };
