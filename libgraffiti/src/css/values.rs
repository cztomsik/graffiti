// CSS value types

#[derive(Debug, Clone, PartialEq)]
pub enum Value<T> {
    Specified(T),
    Inherit,
    Initial,
    // inherited from parent if the property is inherited by default, initial otherwise
    Unset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self::from_rgba8(0, 0, 0, 0);
    pub const BLACK: Self = Self::from_rgba8(0, 0, 0, 255);
    pub const WHITE: Self = Self::from_rgba8(255, 255, 255, 255);
    pub const RED: Self = Self::from_rgba8(255, 0, 0, 255);
    pub const GREEN: Self = Self::from_rgba8(0, 255, 0, 255);
    pub const BLUE: Self = Self::from_rgba8(0, 0, 255, 255);

    pub(super) const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
    //Vw(f32)
    //Vh(f32)
}

impl Dimension {
    pub const ZERO: Self = Self::Px(0.);
}

// TODO: Border (border-spacing/collapse is shared for all sides so it can't be array)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    None,
    Block,
    Inline,
    Flex,
    // Grid, Table, TableRow, TableCell, ...
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

// TODO, enum?
//pub struct Transform { ? }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}
