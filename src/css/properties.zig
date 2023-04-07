// TODO: pub usingnamespace Rect(@This()) ???
//       which can define both parse() and format() together?

const std = @import("std");
const layout = @import("emlay");
const Parser = @import("parser.zig").Parser;

// longhand props, these are stored
pub const Property = union(enum) {
    display: layout.Display,

    width: layout.Dimension,
    height: layout.Dimension,
    min_width: layout.Dimension,
    min_height: layout.Dimension,
    max_width: layout.Dimension,
    max_height: layout.Dimension,

    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: layout.Dimension,
    flex_direction: layout.FlexDirection,
    flex_wrap: layout.FlexWrap,

    align_content: layout.AlignContent,
    align_items: layout.AlignItems,
    align_self: layout.AlignSelf,
    justify_content: layout.JustifyContent,

    padding_top: layout.Dimension,
    padding_right: layout.Dimension,
    padding_bottom: layout.Dimension,
    padding_left: layout.Dimension,

    margin_top: layout.Dimension,
    margin_right: layout.Dimension,
    margin_bottom: layout.Dimension,
    margin_left: layout.Dimension,

    border_top_width: layout.Dimension,
    // border_top_style: BorderStyle,
    // border_top_color: Color,

    border_right_width: layout.Dimension,
    // border_right_style: BorderStyle,
    // border_right_color: Color,

    border_bottom_width: layout.Dimension,
    // border_bottom_style: BorderStyle,
    // border_bottom_color: Color,

    border_left_width: layout.Dimension,
    // border_left_style: BorderStyle,
    // border_left_color: Color,

    border_top_left_radius: layout.Dimension,
    border_top_right_radius: layout.Dimension,
    border_bottom_right_radius: layout.Dimension,
    border_bottom_left_radius: layout.Dimension,

    outline_width: layout.Dimension,
    // outline_style: BorderStyle,
    // outline_color: Color,
};

// shorthand props, these are only parsed and printed
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
        flex_basis: layout.Dimension = .auto, // .{ .percent = 0 }
    },

    padding: struct {
        padding_top: layout.Dimension = .{ .px = 0 },
        padding_right: layout.Dimension = .{ .px = 0 },
        padding_bottom: layout.Dimension = .{ .px = 0 },
        padding_left: layout.Dimension = .{ .px = 0 },
    },

    margin: struct {
        margin_top: layout.Dimension = .{ .px = 0 },
        margin_right: layout.Dimension = .{ .px = 0 },
        margin_bottom: layout.Dimension = .{ .px = 0 },
        margin_left: layout.Dimension = .{ .px = 0 },
    },

    // inset: struct {
    //     top: layout.Dimension = .auto,
    //     right: layout.Dimension = .auto,
    //     bottom: layout.Dimension = .auto,
    //     left: layout.Dimension = .auto,
    // },

    background: struct {
        // background_color: Color = Color.TRANSPARENT,
    },

    // TODO: this should expand to 12 longhands but it should only
    //       accept/print 3 values
    // border: struct {}

    border_width: struct {
        border_top_width: layout.Dimension = .{ .px = 3 },
        border_right_width: layout.Dimension = .{ .px = 3 },
        border_bottom_width: layout.Dimension = .{ .px = 3 },
        border_left_width: layout.Dimension = .{ .px = 3 },
    },

    border_style: struct {
        // border_top_style: BorderStyle = .none,
        // border_right_style: BorderStyle = .none,
        // border_bottom_style: BorderStyle = .none,
        // border_left_style: BorderStyle = .none,
    },

    border_color: struct {
        // border_top_color: Color = Color.TRANSPARENT,
        // border_right_color: Color = Color.TRANSPARENT,
        // border_bottom_color: Color = Color.TRANSPARENT,
        // border_left_color: Color = Color.TRANSPARENT,
    },

    border_top: struct {
        // border_top_width: layout.Dimension = .{ .px = 3 },
        // border_top_style: BorderStyle = .none,
        // border_top_color: Color = Color.TRANSPARENT,
    },

    border_right: struct {
        // border_right_width: layout.Dimension = .{ .px = 3 },
        // border_right_style: BorderStyle = .none,
        // border_right_color: Color = Color.TRANSPARENT,
    },

    border_bottom: struct {
        // border_bottom_width: layout.Dimension = .{ .px = 3 },
        // border_bottom_style: BorderStyle = .none,
        // border_bottom_color: Color = Color.TRANSPARENT,
    },

    border_left: struct {
        // border_left_width: layout.Dimension = .{ .px = 3 },
        // border_left_style: BorderStyle = .none,
        // border_left_color: Color = Color.TRANSPARENT,
    },

    border_radius: struct {
        border_top_left_radius: layout.Dimension = .{ .px = 0 },
        border_top_right_radius: layout.Dimension = .{ .px = 0 },
        border_bottom_right_radius: layout.Dimension = .{ .px = 0 },
        border_bottom_left_radius: layout.Dimension = .{ .px = 0 },
    },

    overflow: struct {
        // overflow_x: Overflow = .visible,
        // overflow_y: Overflow = .visible,
    },

    outline: struct {
        outline_width: layout.Dimension = .{ .px = 3 },
        // outline_style: BorderStyle,
        // outline_color: Color = Color.TRANSPARENT,
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
