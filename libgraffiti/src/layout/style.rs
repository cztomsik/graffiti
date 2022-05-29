#[derive(Debug, Clone)]
pub struct LayoutStyle {
    pub display: Display,

    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,

    pub padding: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub border: Rect<Dimension>,

    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,

    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,

    pub align_self: Align,
    pub align_content: Align,
    pub align_items: Align,
    pub justify_content: Justify,

    pub position_type: Position,
    pub position: Rect<Dimension>,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            display: Display::Inline,

            size: Size::AUTO,
            min_size: Size::AUTO,
            max_size: Size::AUTO,

            padding: Rect::ZERO,
            margin: Rect::ZERO,
            border: Rect::ZERO,

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,

            flex_grow: 0.,
            flex_shrink: 1.,
            flex_basis: Dimension::Auto,

            align_self: Align::Auto,
            align_items: Align::Stretch,
            align_content: Align::Stretch,
            justify_content: Justify::FlexStart,

            position_type: Position::Static,
            position: Rect::ZERO,
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
    Percent(f32),
    Em(f32),
    Rem(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size<T: Copy> {
    pub width: T,
    pub height: T,
}

impl<T: Copy> Size<T> {
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl<T: Copy + PartialOrd> Size<T> {
    pub fn min(self) -> T {
        if self.width < self.height {
            self.width
        } else {
            self.height
        }
    }

    pub fn max(self) -> T {
        if self.width > self.height {
            self.width
        } else {
            self.height
        }
    }
}

impl Size<Dimension> {
    pub const AUTO: Self = Self::new(Dimension::Auto, Dimension::Auto);
    pub const ZERO: Self = Self::new(Dimension::Px(0.), Dimension::Px(0.));
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect<T: Copy> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl Rect<Dimension> {
    pub const AUTO: Self = Self {
        top: Dimension::Auto,
        right: Dimension::Auto,
        bottom: Dimension::Auto,
        left: Dimension::Auto,
    };

    pub const ZERO: Self = Self {
        top: Dimension::Px(0.),
        right: Dimension::Px(0.),
        bottom: Dimension::Px(0.),
        left: Dimension::Px(0.),
    };
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
pub enum Position {
    Static,
    Relative,
    Absolute,
}
