// CSS value types

use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CssValue<T> {
    Specified(T),
    Inherit,
    Initial,
    // inherited from parent if the property is inherited by default, initial otherwise
    Unset,
}

impl<T: Display> Display for CssValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Specified(v) => write!(f, "{}", v),
            Self::Inherit => write!(f, "inherit"),
            Self::Initial => write!(f, "initial"),
            Self::Unset => write!(f, "unset"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssAlign {
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

impl Display for CssAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Start => write!(f, "start"),
            Self::Center => write!(f, "center"),
            Self::End => write!(f, "end"),
            Self::Stretch => write!(f, "stretch"),
            Self::Baseline => write!(f, "baseline"),
            Self::SpaceBetween => write!(f, "space-between"),
            Self::SpaceAround => write!(f, "space-around"),
            Self::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CssColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(unused)]
impl CssColor {
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

impl Display for CssColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let Self { r, g, b, a } = self;
        write!(f, "rgba({}, {}, {}, {})", r, g, b, a)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssDimension {
    Auto,
    Px(f32),
    Percent(f32),
    //Vw(f32)
    //Vh(f32)
}

impl Display for CssDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Px(v) => write!(f, "{}px", v),
            Self::Percent(v) => write!(f, "{}%", v),
        }
    }
}

// TODO: Border (border-spacing/collapse is shared for all sides so it can't be array)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssBorderStyle {
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

impl Display for CssBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::None => write!(f, "none"),
            Self::Hidden => write!(f, "hidden"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Solid => write!(f, "solid"),
            Self::Double => write!(f, "double"),
            Self::Groove => write!(f, "groove"),
            Self::Ridge => write!(f, "ridge"),
            Self::Inset => write!(f, "inset"),
            Self::Outset => write!(f, "outset"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssBoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: CssColor,
}

impl Display for CssBoxShadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}px {}px {}px {}px {}",
            self.offset.0, self.offset.1, self.blur, self.spread, self.color
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssDisplay {
    None,
    Block,
    Inline,
    Flex,
    // Grid, Table, TableRow, TableCell, ...
}

impl Display for CssDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::None => write!(f, "none"),
            Self::Block => write!(f, "block"),
            Self::Inline => write!(f, "inline"),
            Self::Flex => write!(f, "flex"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl Display for CssFlexDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Column => write!(f, "column"),
            Self::ColumnReverse => write!(f, "row-reverse"),
            Self::Row => write!(f, "row"),
            Self::RowReverse => write!(f, "row-reverse"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Display for CssFlexWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::NoWrap => write!(f, "nowrap"),
            Self::Wrap => write!(f, "wrap"),
            Self::WrapReverse => write!(f, "wrap-reverse"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssOverflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Display for CssOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
            Self::Scroll => write!(f, "scroll"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssPosition {
    Static,
    Relative,
    Absolute,
    Sticky,
}

impl Display for CssPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Static => write!(f, "static"),
            Self::Relative => write!(f, "relative"),
            Self::Absolute => write!(f, "absolute"),
            Self::Sticky => write!(f, "sticky"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssTextAlign {
    Left,
    Right,
    Center,
    Justify,
}

impl Display for CssTextAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::Justify => write!(f, "justify"),
        }
    }
}

// TODO, enum?
//pub struct CssTransform { ? }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssVisibility {
    Visible,
    Hidden,
    Collapse,
}

impl Display for CssVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
            Self::Collapse => write!(f, "collapse"),
        }
    }
}
