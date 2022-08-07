// basic types
// TODO: percent: f32, em: f32, rem: f32, vw: f32, vh: f32, vmin, vmax
pub const Dimension = union(enum) { auto, px: f32 };
pub const Px = struct { f32 };
pub const Color = struct { r: u8, g: u8, b: u8, a: u8 };
pub const BoxShadow = struct { offset: [2]Px, blur: Px, spread: Px, color: Color };
pub const Transform = union(enum) { translate: [2]f32, scale: [2]f32, rotate: f32 };

// enums
pub const Align = enum { auto, flex_start, center, flex_end, stretch, baseline, space_between, space_around };
pub const Justify = enum { flex_start, center, flex_end, space_between, space_around, space_evenly };
pub const BorderStyle = enum { none, hidden, dotted, dashed, solid, double, groove, ridge, inset, outset };
pub const Display = enum { none, block, @"inline", inline_block, flex, table, table_header_group, table_row_group, table_row, table_cell };
pub const FlexDirection = enum { column, column_reverse, row, row_reverse };
pub const FlexWrap = enum { no_wrap, wrap, wrap_reverse };
pub const Overflow = enum { visible, hidden, scroll, auto };
pub const Position = enum { static, relative, absolute, sticky };
pub const TextAlign = enum { left, right, center, justify };
pub const Visibility = enum { visible, hidden, collapse };

pub const Style = struct {
    // TODO: encode as u8 + var-len value (most declarations are prop+enum)
    //       but
    props: []StyleProp,
};

// longhand props
pub const StyleProp = union(enum) {
    display: Display,

    // // size
    // width: Dimension,
    // height: Dimension,
    // // min_width: Dimension,
    // // min_height: Dimension,
    // // max_width: Dimension,
    // // max_height: Dimension,

    // // padding
    // padding_top: Dimension,
    // padding_right: Dimension,
    // padding_bottom: Dimension,
    // padding_left: Dimension,

    // // margin
    // // margin_top: Dimension,
    // // margin_right: Dimension,
    // // margin_bottom: Dimension,
    // // margin_left: Dimension,

    // // background
    // background_color: Color,

    // // border-radius
    // // border_top_left_radius: Px,
    // // border_top_right_radius: Px,
    // // border_bottom_right_radius: Px,
    // // border_bottom_left_radius: Px,

    // // border
    // // border_top_width: Px,
    // // border_top_style: BorderStyle,
    // // border_top_color: Color,
    // // border_right_width: Px,
    // // border_right_style: BorderStyle,
    // // border_right_color: Color,
    // // border_bottom_width: Px,
    // // border_bottom_style: BorderStyle,
    // // border_bottom_color: Color,
    // // border_left_width: Px,
    // // border_left_style: BorderStyle,
    // // border_left_color: Color,

    // // shadow
    // // box_shadow: BoxShadow,

    // // flex
    // // flex_grow: f32,
    // // flex_shrink: f32,
    // // flex_basis: Dimension,
    // // flex_direction: FlexDirection,
    // // flex_wrap: FlexWrap,
    // // align_content: Align,
    // // align_items: Align,
    // // align_self: Align,
    // // justify_content: Justify,

    // // text
    // // font_family: String,
    // // font_size: Dimension,
    // // line_height: Dimension,
    // // text_align: TextAlign,
    // // color: Color,

    // // outline
    // // outline_color: Color,
    // // outline_style: BorderStyle,
    // // outline_width: Px,

    // // overflow
    // // overflow_x: Overflow,
    // // overflow_y: Overflow,

    // // position
    // // position: Position,
    // // top: Dimension,
    // // right: Dimension,
    // // bottom: Dimension,
    // // left: Dimension,

    // // other
    // // opacity: f32,
    // // visibility: Visibility,
    // // transform: Transform,
};
