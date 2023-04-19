// supported CSS properties
//
// note that we are using the types from the style module
// this is because we want to avoid any unnecessary conversions
// during layout and rendering (so we parse as much as we can)

// TODO: pub usingnamespace Rect(@This()) ???
//       which can define both parse() and format() together?

const std = @import("std");
const Parser = @import("parser.zig").Parser;
const style = @import("../style.zig");

// TODO: decide if we want to keep this separate or generate it from the Style
//       struct. But maybe this is easier to read and it's easier to add custom
//       parsing for some properties. But we could also use custom type in the
//       Style struct, so I'm not sure.
//
// longhand props, these are stored
pub const Property = union(std.meta.FieldEnum(style.Style)) {
    display: style.Display,

    width: style.Dimension,
    height: style.Dimension,
    min_width: style.Dimension,
    min_height: style.Dimension,
    max_width: style.Dimension,
    max_height: style.Dimension,

    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: style.Dimension,
    flex_direction: style.FlexDirection,
    flex_wrap: style.FlexWrap,

    align_content: style.AlignContent,
    align_items: style.AlignItems,
    align_self: style.AlignSelf,
    justify_content: style.JustifyContent,

    padding_top: style.Dimension,
    padding_right: style.Dimension,
    padding_bottom: style.Dimension,
    padding_left: style.Dimension,

    margin_top: style.Dimension,
    margin_right: style.Dimension,
    margin_bottom: style.Dimension,
    margin_left: style.Dimension,

    border_top_width: style.Dimension,
    border_top_style: style.BorderStyle,
    border_top_color: style.Color,

    border_right_width: style.Dimension,
    border_right_style: style.BorderStyle,
    border_right_color: style.Color,

    border_bottom_width: style.Dimension,
    border_bottom_style: style.BorderStyle,
    border_bottom_color: style.Color,

    border_left_width: style.Dimension,
    border_left_style: style.BorderStyle,
    border_left_color: style.Color,

    border_top_left_radius: style.Dimension,
    border_top_right_radius: style.Dimension,
    border_bottom_right_radius: style.Dimension,
    border_bottom_left_radius: style.Dimension,

    visibility: style.Visibility,
    opacity: f32,
    overflow_x: style.Overflow,
    overflow_y: style.Overflow,
    box_shadow: ?style.BoxShadow, // TODO: []const style.BoxShadow,
    background_color: style.Color,
    // TODO: background_image: []const style.BackgroundImage,

    outline_width: style.Dimension,
    outline_style: style.OutlineStyle,
    outline_color: style.Color,
};

// shorthand props, we parse & print them but we always store expanded longhands
// every shorthand is just a struct of longhands which makes it easy to
// expand them into longhands with a simple inline for loop.
//
// we could concat prefix and field name, but there are some edge-cases like `border-radius`
// where the full name is in mixed order (`border-top-left-radius`)
//
// parsing & printing usually works fine thanks to defaults and optionals
// but it's possible to define `parse()` and `format()` for anything custom
pub const Shorthand = union(enum) {
    flex: struct {
        flex_grow: f32 = 0,
        flex_shrink: f32 = 1,
        flex_basis: style.Dimension = .auto, // .{ .percent = 0 }
    },

    padding: struct {
        padding_top: style.Dimension = .{ .px = 0 },
        padding_right: style.Dimension = .{ .px = 0 },
        padding_bottom: style.Dimension = .{ .px = 0 },
        padding_left: style.Dimension = .{ .px = 0 },
    },

    margin: struct {
        margin_top: style.Dimension = .{ .px = 0 },
        margin_right: style.Dimension = .{ .px = 0 },
        margin_bottom: style.Dimension = .{ .px = 0 },
        margin_left: style.Dimension = .{ .px = 0 },
    },

    // TODO: this is simplified but the full syntax is crazy, so
    //       we will likely support just some reasonable subset
    background: struct {
        // TODO: background_image: []const style.BackgroundImage,
        background_color: style.Color = style.TRANSPARENT,
    },

    // TODO: this should expand to 12 longhands but it should only
    //       accept/print 3 values
    // border: struct {}

    border_width: struct {
        border_top_width: style.Dimension = .{ .px = 3 },
        border_right_width: style.Dimension = .{ .px = 3 },
        border_bottom_width: style.Dimension = .{ .px = 3 },
        border_left_width: style.Dimension = .{ .px = 3 },
    },

    border_style: struct {
        border_top_style: style.BorderStyle = .none,
        border_right_style: style.BorderStyle = .none,
        border_bottom_style: style.BorderStyle = .none,
        border_left_style: style.BorderStyle = .none,
    },

    border_color: struct {
        border_top_color: style.Color = style.TRANSPARENT,
        border_right_color: style.Color = style.TRANSPARENT,
        border_bottom_color: style.Color = style.TRANSPARENT,
        border_left_color: style.Color = style.TRANSPARENT,
    },

    border_top: struct {
        border_top_width: style.Dimension = .{ .px = 3 },
        border_top_style: style.BorderStyle = .none,
        border_top_color: style.Color = style.TRANSPARENT,
    },

    border_right: struct {
        border_right_width: style.Dimension = .{ .px = 3 },
        border_right_style: style.BorderStyle = .none,
        border_right_color: style.Color = style.TRANSPARENT,
    },

    border_bottom: struct {
        border_bottom_width: style.Dimension = .{ .px = 3 },
        border_bottom_style: style.BorderStyle = .none,
        border_bottom_color: style.Color = style.TRANSPARENT,
    },

    border_left: struct {
        border_left_width: style.Dimension = .{ .px = 3 },
        border_left_style: style.BorderStyle = .none,
        border_left_color: style.Color = style.TRANSPARENT,
    },

    border_radius: struct {
        border_top_left_radius: style.Dimension = .{ .px = 0 },
        border_top_right_radius: style.Dimension = .{ .px = 0 },
        border_bottom_right_radius: style.Dimension = .{ .px = 0 },
        border_bottom_left_radius: style.Dimension = .{ .px = 0 },
    },

    overflow: struct {
        overflow_x: style.Overflow = .visible,
        overflow_y: style.Overflow = .visible,
    },

    outline: struct {
        outline_width: style.Dimension = .{ .px = 3 },
        outline_style: style.OutlineStyle,
        outline_color: style.Color = style.TRANSPARENT,
    },
};

// fn parseRect(comptime T: type) !T {
//     const P = struct {
//         fn parse(self: *Parser) !T {
//             const fields = std.meta.fields(T);
//             const res: T = undefined;
//             const vals: fields[0].type[4] = undefined;
//             vals[0] = try self.parse(V);
//             vals[1] = try self.parse(?V) orelse vals[0];
//             vals[2] = try self.parse(?V) orelse vals[0];
//             vals[3] = try self.parse(?V) orelse vals[1];

//             inline for (fields, vals) |f, v| {
//                 @field(res, f.name) = v;
//             }

//             return res;
//         }
//     };
//     return P.parse;
// }
