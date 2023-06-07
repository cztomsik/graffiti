const std = @import("std");
const layout = @import("emlay");
const css = @import("css/mod.zig");

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
pub const Color = @import("css/values/color.zig").Color;
pub const Dimension = @import("css/values/dimension.zig").Dimension;

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

pub const StyleProp = blk: {
    var fields: [std.meta.fields(Style).len]std.builtin.Type.UnionField = undefined;
    for (std.meta.fields(Style), 0..) |f, i| {
        fields[i] = .{
            .name = f.name,
            .type = f.type,
            .alignment = @alignOf(f.type),
        };
    }
    break :blk @Type(.{ .Union = .{
        .layout = .Auto,
        .tag_type = std.meta.FieldEnum(Style),
        .fields = &fields,
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
        flex_basis: Dimension = .auto, // .{ .percent = 0 }
    },

    padding: struct {
        padding_top: Dimension = .{ .px = 0 },
        padding_right: Dimension = .{ .px = 0 },
        padding_bottom: Dimension = .{ .px = 0 },
        padding_left: Dimension = .{ .px = 0 },

        pub usingnamespace RectMixin(@This());
    },

    margin: struct {
        margin_top: Dimension = .{ .px = 0 },
        margin_right: Dimension = .{ .px = 0 },
        margin_bottom: Dimension = .{ .px = 0 },
        margin_left: Dimension = .{ .px = 0 },

        pub usingnamespace RectMixin(@This());
    },

    // TODO: this is simplified but the full syntax is crazy, so
    //       we will likely support just some reasonable subset
    background: struct {
        // TODO: background_image: []const BackgroundImage,
        background_color: Color = TRANSPARENT,
    },

    // TODO: this should expand to 12 longhands but it should only
    //       accept/print 3 values
    // border: struct {}

    border_width: struct {
        border_top_width: Dimension = .{ .px = 3 },
        border_right_width: Dimension = .{ .px = 3 },
        border_bottom_width: Dimension = .{ .px = 3 },
        border_left_width: Dimension = .{ .px = 3 },
    },

    border_style: struct {
        border_top_style: BorderStyle = .none,
        border_right_style: BorderStyle = .none,
        border_bottom_style: BorderStyle = .none,
        border_left_style: BorderStyle = .none,
    },

    border_color: struct {
        border_top_color: Color = TRANSPARENT,
        border_right_color: Color = TRANSPARENT,
        border_bottom_color: Color = TRANSPARENT,
        border_left_color: Color = TRANSPARENT,
    },

    border_top: struct {
        border_top_width: Dimension = .{ .px = 3 },
        border_top_style: BorderStyle = .none,
        border_top_color: Color = TRANSPARENT,
    },

    border_right: struct {
        border_right_width: Dimension = .{ .px = 3 },
        border_right_style: BorderStyle = .none,
        border_right_color: Color = TRANSPARENT,
    },

    border_bottom: struct {
        border_bottom_width: Dimension = .{ .px = 3 },
        border_bottom_style: BorderStyle = .none,
        border_bottom_color: Color = TRANSPARENT,
    },

    border_left: struct {
        border_left_width: Dimension = .{ .px = 3 },
        border_left_style: BorderStyle = .none,
        border_left_color: Color = TRANSPARENT,
    },

    border_radius: struct {
        border_top_left_radius: Dimension = .{ .px = 0 },
        border_top_right_radius: Dimension = .{ .px = 0 },
        border_bottom_right_radius: Dimension = .{ .px = 0 },
        border_bottom_left_radius: Dimension = .{ .px = 0 },
    },

    overflow: struct {
        overflow_x: Overflow = .visible,
        overflow_y: Overflow = .visible,
    },

    outline: struct {
        outline_width: Dimension = .{ .px = 3 },
        outline_style: OutlineStyle,
        outline_color: Color = TRANSPARENT,
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
