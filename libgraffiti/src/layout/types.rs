#[derive(Debug, Clone, Copy)]
pub struct LayoutStyle {
    pub display: Display,

    pub width: Dimension,
    pub height: Dimension,

    pub min_width: Dimension,
    pub min_height: Dimension,

    pub max_width: Dimension,
    pub max_height: Dimension,

    pub padding: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub border: Rect<Dimension>,

    // flex & grid (not supported ATM)
    pub align_self: Align,
    pub align_content: Align,
    pub align_items: Align,
    pub justify_content: Justify,

    // flex
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
}

impl LayoutStyle {
    pub(crate) fn size(&self) -> Size<Dimension> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            display: Display::Inline,

            width: Dimension::Auto,
            height: Dimension::Auto,

            min_width: Dimension::Auto,
            min_height: Dimension::Auto,

            max_width: Dimension::Auto,
            max_height: Dimension::Auto,

            padding: Rect::ZERO,
            margin: Rect::ZERO,
            border: Rect::ZERO,

            // TODO: position
            // TODO: overflow
            align_self: Align::Auto,
            align_items: Align::Stretch,
            align_content: Align::Stretch,
            justify_content: Justify::FlexStart,

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.,
            flex_shrink: 1.,
            flex_basis: Dimension::Auto,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
    Table,
    TableRow,
    TableCell,
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Auto,
    Px(f32),
    /*Fraction*/ Percent(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Size<T: Copy> {
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect<T: Copy> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Rect<Dimension> {
    pub const ZERO: Self = Self {
        top: Dimension::Px(0.),
        right: Dimension::Px(0.),
        bottom: Dimension::Px(0.),
        left: Dimension::Px(0.),
    };
}
