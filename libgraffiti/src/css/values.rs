// CSS value types

use std::convert::TryFrom;
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

macro_rules! css_enums {
    ($(
        #[$($meta:meta),*]
        $pub:vis enum $name:ident {
            $($variant:ident = $value:literal,)*
        }
    )*) => {
        $(
            #[$($meta),*]
            $pub enum $name {
                $($variant),*
            }

            impl Display for $name {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                    match self {
                        $(Self::$variant => write!(f, $value),)*
                    }
                }
            }

            impl <'a> TryFrom<&'a str> for $name {
                type Error = &'static str;

                fn try_from(v: &str) -> Result<Self, Self::Error> {
                    Ok(match v {
                        $($value => Self::$variant,)*
                        _ => return Err("invalid input")
                    })
                }
            }
        )*
    };
}

css_enums! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssAlign {
        Auto = "auto",
        Start = "start",
        FlexStart = "flex-start",
        Center = "center",
        End = "end",
        FlexEnd = "flex-end",
        Stretch = "stretch",
        Baseline = "baseline",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
        SpaceEvenly = "space-evenly",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssBorderStyle {
        None = "none",
        Hidden = "hidden",
        Dotted = "dotted",
        Dashed = "dashed",
        Solid = "solid",
        Double = "double",
        Groove = "groove",
        Ridge = "ridge",
        Inset = "inset",
        Outset = "outset",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssDisplay {
        None = "none",
        Block = "block",
        Inline = "inline",
        Flex = "flex",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssFlexDirection {
        Column = "column",
        ColumnReverse = "column-reverse",
        Row = "row",
        RowReverse = "row-reverse",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssFlexWrap {
        NoWrap = "nowrap",
        Wrap = "wrap",
        WrapReverse = "wrap-reverse",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssOverflow {
        Visible = "visible",
        Hidden = "hidden",
        Scroll = "scroll",
        Auto = "auto",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssPosition {
        Static = "static",
        Relative = "relative",
        Absolute = "absolute",
        Sticky = "sticky",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssTextAlign {
        Left = "left",
        Right = "right",
        Center = "center",
        Justify = "justify",
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssVisibility {
        Visible = "visible",
        Hidden = "hidden",
        Collapse = "collapse",
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
