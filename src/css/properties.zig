const layout = @import("emlay");

// longhand props, these are stored
pub const Property = union(enum) {
    display: layout.Display,

    // width: layout.Dimension,
    // height: layout.Dimension,
    // min_width: layout.Dimension,
    // min_height: layout.Dimension,
    // max_width: layout.Dimension,
    // max_height: layout.Dimension,

    flex_grow: f32,
    flex_shrink: f32,
    // flex_basis: layout.Dimension,
    // flex_direction: layout.FlexDirection,
    // flex_wrap: layout.FlexWrap,

    // align_content: layout.AlignContent,
    // align_items: layout.AlignItems,
    // align_self: layout.AlignSelf,
    // justify_content: layout.JustifyContent,

    // padding_top: Dimension,
    // padding_right: Dimension,
    // padding_bottom: Dimension,
    // padding_left: Dimension,

    // margin_top: Dimension,
    // margin_right: Dimension,
    // margin_bottom: Dimension,
    // margin_left: Dimension,

    // border_top_width: Dimension,
    // border_top_style: BorderStyle,
    // border_top_color: Color,

    // border_right_width: Dimension,
    // border_right_style: BorderStyle,
    // border_right_color: Color,

    // border_bottom_width: Dimension,
    // border_bottom_style: BorderStyle,
    // border_bottom_color: Color,

    // border_left_style: BorderStyle,
    // border_left_width: Dimension,
    // border_left_color: Color,

    // border_top_left_radius: Dimension,
    // border_top_right_radius: Dimension,
    // border_bottom_right_radius: Dimension,
    // border_bottom_left_radius: Dimension,

    // outline_width: Dimension,
    // outline_style: BorderStyle,
    // outline_color: Color,
    // outline_offset: Dimension,

};

// shorthand props, these are only parsed and printed (TODO)
// every shorthand is just a struct of longhands which makes it easy to
// expand them into longhands with a simple inline for loop.
//
// we could concat prefix and field name, but there are some edge-cases like `border-radius`
// where the full name is in mixed order (`border-top-left-radius`)
//
// parsing usually works fine thanks to defaults and optionals
// but it's possible to define .parse() for anything custom
//
// TODO: printing shorthands in getPropertyValue() could work by
// first checking if we have all the longhands and
// "filling the struct as we go", and then we could just call the
// generic formatter
pub const Shorthand = union(enum) {
    flex: struct {
        flex_grow: f32 = 0,
        flex_shrink: f32 = 0,
        // flex_basis: layout.Dimension = .auto, // .{ .percent = 0 }
    },

    padding: struct {
        // padding_top: Dimension = .{ .px = 0 },
        // padding_right: Dimension = .{ .px = 0 },
        // padding_bottom: Dimension = .{ .px = 0 },
        // padding_left: Dimension = .{ .px = 0 },
    },

    margin: struct {
        // margin_top: Dimension = .{ .px = 0 },
        // margin_right: Dimension = .{ .px = 0 },
        // margin_bottom: Dimension = .{ .px = 0 },
        // margin_left: Dimension = .{ .px = 0 },
    },

    background: struct {
        // background_color: Color = Color.TRANSPARENT,
        // background_image: []const u8 = "",
        // background_repeat: BackgroundRepeat = .repeat,
        // background_attachment: BackgroundAttachment = .scroll,
        // background_position_x: Dimension = .{ .px = 0 },
        // background_position_y: Dimension = .{ .px = 0 },
    },

    // border: struct {

    border_width: struct {
        // border_top_width: Dimension = .{ .px = 3 },
        // border_right_width: Dimension = .{ .px = 3 },
        // border_bottom_width: Dimension = .{ .px = 3 },
        // border_left_width: Dimension = .{ .px = 3 },
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
        // border_top_width: Dimension = .{ .px = 3 },
        // border_top_style: BorderStyle = .none,
        // border_top_color: Color = Color.TRANSPARENT,
    },

    border_right: struct {
        // border_right_width: Dimension = .{ .px = 3 },
        // border_right_style: BorderStyle = .none,
        // border_right_color: Color = Color.TRANSPARENT,
    },

    border_bottom: struct {
        // border_bottom_width: Dimension = .{ .px = 3 },
        // border_bottom_style: BorderStyle = .none,
        // border_bottom_color: Color = Color.TRANSPARENT,
    },

    border_left: struct {
        // border_left_width: Dimension = .{ .px = 3 },
        // border_left_style: BorderStyle = .none,
        // border_left_color: Color = Color.TRANSPARENT,
    },

    border_radius: struct {
        // border_top_left_radius: Dimension = .{ .px = 0 },
        // border_top_right_radius: Dimension = .{ .px = 0 },
        // border_bottom_right_radius: Dimension = .{ .px = 0 },
        // border_bottom_left_radius: Dimension = .{ .px = 0 },
    },

    overflow: struct {
        // overflow_x: Overflow = .visible,
        // overflow_y: Overflow = .visible,
    },

    outline: struct {
        outline_width: f32 = 3,
        outline_style: enum { none, solid },
        // outline_color: Color = Color.TRANSPARENT,
    },

    // font: struct {
    //     font_style: FontStyle = .normal,
    //     font_variant: FontVariant = .normal,
    //     font_weight: FontWeight = .normal,
    //     font_size: Dimension = .{ .px = 16 },
    //     line_height: Dimension = .{ .px = 16 },
    //     font_family: []const u8 = "sans-serif",
    // },

};
