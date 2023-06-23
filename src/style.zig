const std = @import("std");
const emlay = @import("emlay");
const css = @import("css/mod.zig");

// enums
pub const Visibility = enum { visible, hidden };
pub const Overflow = enum { visible, hidden, scroll, auto };
pub const OutlineStyle = enum { none, solid };
pub const BorderStyle = enum { none, solid };

// TODO: figure out where to put these
pub const Color = @import("css/values/color.zig").Color;

// consts
pub const TRANSPARENT: css.Color = .{ .r = 0, .g = 0, .b = 0, .a = 0 };
pub const CURRENT_COLOR: css.Color = .{ .r = 0, .g = 0, .b = 0, .a = 255 }; // TODO: css.Color.current_color (and convert it in element.applyStyle())

pub const LayerStyle = struct {
    visibility: enum { visible, hidden } = .visible,
    opacity: f32 = 1,

    // transform: []const Transform = &.{},

    // overflow
    overflow_x: Overflow = .visible,
    overflow_y: Overflow = .visible,

    // border-radius
    border_top_left_radius: emlay.Dimension = .{ .px = 0 },
    border_top_right_radius: emlay.Dimension = .{ .px = 0 },
    border_bottom_right_radius: emlay.Dimension = .{ .px = 0 },
    border_bottom_left_radius: emlay.Dimension = .{ .px = 0 },

    // outline
    outline_width: emlay.Dimension = .{ .px = 3 },
    outline_style: BorderStyle = .none,
    outline_color: css.Color = CURRENT_COLOR,

    // background
    background_color: css.Color = TRANSPARENT,
    // TODO: background_image: []const BackgroundImage = &.{},

    // border (widths are part of the LayoutStyle)
    border_top_style: BorderStyle = .none,
    border_top_color: css.Color = CURRENT_COLOR,
    border_right_style: BorderStyle = .none,
    border_right_color: css.Color = CURRENT_COLOR,
    border_bottom_style: BorderStyle = .none,
    border_bottom_color: css.Color = CURRENT_COLOR,
    border_left_style: BorderStyle = .none,
    border_left_color: css.Color = CURRENT_COLOR,
};

// supported CSS props
// - all emlay.Style props (but parse css.Dimension which is converted to emlay.Dimension during applyStyle())
// - all LayerStyle props
// - all TextAttr variants (TODO)
pub const StyleProp = blk: {
    var enum_fields: []const std.builtin.Type.EnumField = &.{};
    var union_fields: []const std.builtin.Type.UnionField = &.{};

    for (.{ emlay.Style, LayerStyle }) |T| {
        for (std.meta.fields(T)) |f| {
            enum_fields = enum_fields ++ .{.{
                .name = f.name,
                .value = enum_fields.len,
            }};

            union_fields = union_fields ++ .{.{
                .name = f.name,
                .type = if (f.type == emlay.Dimension) css.Dimension else f.type,
                .alignment = @alignOf(f.type),
            }};
        }
    }

    break :blk @Type(.{ .Union = .{
        .layout = .Auto,
        .tag_type = @Type(.{ .Enum = .{
            .tag_type = std.math.IntFittingRange(0, enum_fields.len - 1),
            .fields = enum_fields,
            .decls = &.{},
            .is_exhaustive = true,
        } }),
        .fields = union_fields,
        .decls = &.{},
    } });
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
        flex_basis: css.Dimension = .auto, // .{ .percent = 0 }
    },

    padding: struct {
        padding_top: css.Dimension = .{ .px = 0 },
        padding_right: css.Dimension = .{ .px = 0 },
        padding_bottom: css.Dimension = .{ .px = 0 },
        padding_left: css.Dimension = .{ .px = 0 },

        pub usingnamespace RectMixin(@This());
    },

    margin: struct {
        margin_top: css.Dimension = .{ .px = 0 },
        margin_right: css.Dimension = .{ .px = 0 },
        margin_bottom: css.Dimension = .{ .px = 0 },
        margin_left: css.Dimension = .{ .px = 0 },

        pub usingnamespace RectMixin(@This());
    },

    // TODO: this is simplified but the full syntax is crazy, so
    //       we will likely support just some reasonable subset
    background: struct {
        // TODO: background_image: []const BackgroundImage,
        background_color: css.Color = TRANSPARENT,
    },

    // // TODO: this should expand to 12 longhands but it should only
    // //       accept/print 3 values
    // // border: struct {}

    border_width: struct {
        border_top_width: css.Dimension = .{ .px = 3 },
        border_right_width: css.Dimension = .{ .px = 3 },
        border_bottom_width: css.Dimension = .{ .px = 3 },
        border_left_width: css.Dimension = .{ .px = 3 },
    },

    border_style: struct {
        border_top_style: BorderStyle = .none,
        border_right_style: BorderStyle = .none,
        border_bottom_style: BorderStyle = .none,
        border_left_style: BorderStyle = .none,
    },

    border_color: struct {
        border_top_color: css.Color = TRANSPARENT,
        border_right_color: css.Color = TRANSPARENT,
        border_bottom_color: css.Color = TRANSPARENT,
        border_left_color: css.Color = TRANSPARENT,
    },

    border_top: struct {
        border_top_width: css.Dimension = .{ .px = 3 },
        border_top_style: BorderStyle = .none,
        border_top_color: css.Color = TRANSPARENT,
    },

    border_right: struct {
        border_right_width: css.Dimension = .{ .px = 3 },
        border_right_style: BorderStyle = .none,
        border_right_color: css.Color = TRANSPARENT,
    },

    border_bottom: struct {
        border_bottom_width: css.Dimension = .{ .px = 3 },
        border_bottom_style: BorderStyle = .none,
        border_bottom_color: css.Color = TRANSPARENT,
    },

    border_left: struct {
        border_left_width: css.Dimension = .{ .px = 3 },
        border_left_style: BorderStyle = .none,
        border_left_color: css.Color = TRANSPARENT,
    },

    border_radius: struct {
        border_top_left_radius: css.Dimension = .{ .px = 0 },
        border_top_right_radius: css.Dimension = .{ .px = 0 },
        border_bottom_right_radius: css.Dimension = .{ .px = 0 },
        border_bottom_left_radius: css.Dimension = .{ .px = 0 },
    },

    overflow: struct {
        overflow_x: Overflow = .visible,
        overflow_y: Overflow = .visible,
    },

    outline: struct {
        outline_width: css.Dimension = .{ .px = 3 },
        outline_style: BorderStyle,
        outline_color: css.Color = TRANSPARENT,
    },
};

fn RectMixin(comptime T: type) type {
    const fields = std.meta.fields(T);
    const V = fields[0].type;

    return struct {
        pub fn parseWith(parser: *css.Parser) !T {
            var res: T = undefined;
            var vals: [4]V = undefined;
            vals[0] = try parser.parse(V);
            vals[1] = try parser.parse(?V) orelse vals[0];
            vals[2] = try parser.parse(?V) orelse vals[0];
            vals[3] = try parser.parse(?V) orelse vals[1];

            inline for (fields, vals) |f, v| {
                @field(res, f.name) = v;
            }

            return res;
        }
    };
}

pub const StyleDeclaration = css.StyleDeclaration(StyleProp, Shorthand);

pub const StyleSheet = css.StyleSheet(css.StyleRule(StyleDeclaration));
