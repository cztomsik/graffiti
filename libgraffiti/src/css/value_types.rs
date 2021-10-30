// CSS value types

use once_cell::sync::Lazy;
use std::array::IntoIter;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::iter::FromIterator;

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
    // https://drafts.csswg.org/css-flexbox-1/#align-items-property
    // + yoga also has space-between & space-around
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssAlign {
        Auto = "auto",
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
        Stretch = "stretch",
        Baseline = "baseline",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
    }

    // https://drafts.csswg.org/css-flexbox-1/#justify-content-property
    // + yoga also has evenly
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CssJustify {
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
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

impl CssColor {
    // just a few for easier testing
    pub const TRANSPARENT: Self = Self::from_rgba8(0, 0, 0, 0);
    pub const BLACK: Self = Self::from_rgb8(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb8(255, 255, 255);
    pub const RED: Self = Self::from_rgb8(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb8(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb8(0, 0, 255);

    pub(super) const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

    pub(super) const fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub(super) static NAMED_COLORS: Lazy<HashMap<&'static str, CssColor>> = Lazy::new(|| {
    HashMap::from_iter(IntoIter::new([
        ("transparent", CssColor::from_rgba8(0, 0, 0, 0)),
        // https://drafts.csswg.org/css-color/#named-colors
        ("aliceblue", CssColor::from_rgb8(240, 248, 255)),
        ("antiquewhite", CssColor::from_rgb8(250, 235, 215)),
        ("aqua", CssColor::from_rgb8(0, 255, 255)),
        ("aquamarine", CssColor::from_rgb8(127, 255, 212)),
        ("azure", CssColor::from_rgb8(240, 255, 255)),
        ("beige", CssColor::from_rgb8(245, 245, 220)),
        ("bisque", CssColor::from_rgb8(255, 228, 196)),
        ("black", CssColor::from_rgb8(0, 0, 0)),
        ("blanchedalmond", CssColor::from_rgb8(255, 235, 205)),
        ("blue", CssColor::from_rgb8(0, 0, 255)),
        ("blueviolet", CssColor::from_rgb8(138, 43, 226)),
        ("brown", CssColor::from_rgb8(165, 42, 42)),
        ("burlywood", CssColor::from_rgb8(222, 184, 135)),
        ("cadetblue", CssColor::from_rgb8(95, 158, 160)),
        ("chartreuse", CssColor::from_rgb8(127, 255, 0)),
        ("chocolate", CssColor::from_rgb8(210, 105, 30)),
        ("coral", CssColor::from_rgb8(255, 127, 80)),
        ("cornflowerblue", CssColor::from_rgb8(100, 149, 237)),
        ("cornsilk", CssColor::from_rgb8(255, 248, 220)),
        ("crimson", CssColor::from_rgb8(220, 20, 60)),
        ("cyan", CssColor::from_rgb8(0, 255, 255)),
        ("darkblue", CssColor::from_rgb8(0, 0, 139)),
        ("darkcyan", CssColor::from_rgb8(0, 139, 139)),
        ("darkgoldenrod", CssColor::from_rgb8(184, 134, 11)),
        ("darkgray", CssColor::from_rgb8(169, 169, 169)),
        ("darkgreen", CssColor::from_rgb8(0, 100, 0)),
        ("darkgrey", CssColor::from_rgb8(169, 169, 169)),
        ("darkkhaki", CssColor::from_rgb8(189, 183, 107)),
        ("darkmagenta", CssColor::from_rgb8(139, 0, 139)),
        ("darkolivegreen", CssColor::from_rgb8(85, 107, 47)),
        ("darkorange", CssColor::from_rgb8(255, 140, 0)),
        ("darkorchid", CssColor::from_rgb8(153, 50, 204)),
        ("darkred", CssColor::from_rgb8(139, 0, 0)),
        ("darksalmon", CssColor::from_rgb8(233, 150, 122)),
        ("darkseagreen", CssColor::from_rgb8(143, 188, 143)),
        ("darkslateblue", CssColor::from_rgb8(72, 61, 139)),
        ("darkslategray", CssColor::from_rgb8(47, 79, 79)),
        ("darkslategrey", CssColor::from_rgb8(47, 79, 79)),
        ("darkturquoise", CssColor::from_rgb8(0, 206, 209)),
        ("darkviolet", CssColor::from_rgb8(148, 0, 211)),
        ("deeppink", CssColor::from_rgb8(255, 20, 147)),
        ("deepskyblue", CssColor::from_rgb8(0, 191, 255)),
        ("dimgray", CssColor::from_rgb8(105, 105, 105)),
        ("dimgrey", CssColor::from_rgb8(105, 105, 105)),
        ("dodgerblue", CssColor::from_rgb8(30, 144, 255)),
        ("firebrick", CssColor::from_rgb8(178, 34, 34)),
        ("floralwhite", CssColor::from_rgb8(255, 250, 240)),
        ("forestgreen", CssColor::from_rgb8(34, 139, 34)),
        ("fuchsia", CssColor::from_rgb8(255, 0, 255)),
        ("gainsboro", CssColor::from_rgb8(220, 220, 220)),
        ("ghostwhite", CssColor::from_rgb8(248, 248, 255)),
        ("gold", CssColor::from_rgb8(255, 215, 0)),
        ("goldenrod", CssColor::from_rgb8(218, 165, 32)),
        ("gray", CssColor::from_rgb8(128, 128, 128)),
        ("green", CssColor::from_rgb8(0, 128, 0)),
        ("greenyellow", CssColor::from_rgb8(173, 255, 47)),
        ("grey", CssColor::from_rgb8(128, 128, 128)),
        ("honeydew", CssColor::from_rgb8(240, 255, 240)),
        ("hotpink", CssColor::from_rgb8(255, 105, 180)),
        ("indianred", CssColor::from_rgb8(205, 92, 92)),
        ("indigo", CssColor::from_rgb8(75, 0, 130)),
        ("ivory", CssColor::from_rgb8(255, 255, 240)),
        ("khaki", CssColor::from_rgb8(240, 230, 140)),
        ("lavender", CssColor::from_rgb8(230, 230, 250)),
        ("lavenderblush", CssColor::from_rgb8(255, 240, 245)),
        ("lawngreen", CssColor::from_rgb8(124, 252, 0)),
        ("lemonchiffon", CssColor::from_rgb8(255, 250, 205)),
        ("lightblue", CssColor::from_rgb8(173, 216, 230)),
        ("lightcoral", CssColor::from_rgb8(240, 128, 128)),
        ("lightcyan", CssColor::from_rgb8(224, 255, 255)),
        ("lightgoldenrodyellow", CssColor::from_rgb8(250, 250, 210)),
        ("lightgray", CssColor::from_rgb8(211, 211, 211)),
        ("lightgreen", CssColor::from_rgb8(144, 238, 144)),
        ("lightgrey", CssColor::from_rgb8(211, 211, 211)),
        ("lightpink", CssColor::from_rgb8(255, 182, 193)),
        ("lightsalmon", CssColor::from_rgb8(255, 160, 122)),
        ("lightseagreen", CssColor::from_rgb8(32, 178, 170)),
        ("lightskyblue", CssColor::from_rgb8(135, 206, 250)),
        ("lightslategray", CssColor::from_rgb8(119, 136, 153)),
        ("lightslategrey", CssColor::from_rgb8(119, 136, 153)),
        ("lightsteelblue", CssColor::from_rgb8(176, 196, 222)),
        ("lightyellow", CssColor::from_rgb8(255, 255, 224)),
        ("lime", CssColor::from_rgb8(0, 255, 0)),
        ("limegreen", CssColor::from_rgb8(50, 205, 50)),
        ("linen", CssColor::from_rgb8(250, 240, 230)),
        ("magenta", CssColor::from_rgb8(255, 0, 255)),
        ("maroon", CssColor::from_rgb8(128, 0, 0)),
        ("mediumaquamarine", CssColor::from_rgb8(102, 205, 170)),
        ("mediumblue", CssColor::from_rgb8(0, 0, 205)),
        ("mediumorchid", CssColor::from_rgb8(186, 85, 211)),
        ("mediumpurple", CssColor::from_rgb8(147, 112, 219)),
        ("mediumseagreen", CssColor::from_rgb8(60, 179, 113)),
        ("mediumslateblue", CssColor::from_rgb8(123, 104, 238)),
        ("mediumspringgreen", CssColor::from_rgb8(0, 250, 154)),
        ("mediumturquoise", CssColor::from_rgb8(72, 209, 204)),
        ("mediumvioletred", CssColor::from_rgb8(199, 21, 133)),
        ("midnightblue", CssColor::from_rgb8(25, 25, 112)),
        ("mintcream", CssColor::from_rgb8(245, 255, 250)),
        ("mistyrose", CssColor::from_rgb8(255, 228, 225)),
        ("moccasin", CssColor::from_rgb8(255, 228, 181)),
        ("navajowhite", CssColor::from_rgb8(255, 222, 173)),
        ("navy", CssColor::from_rgb8(0, 0, 128)),
        ("oldlace", CssColor::from_rgb8(253, 245, 230)),
        ("olive", CssColor::from_rgb8(128, 128, 0)),
        ("olivedrab", CssColor::from_rgb8(107, 142, 35)),
        ("orange", CssColor::from_rgb8(255, 165, 0)),
        ("orangered", CssColor::from_rgb8(255, 69, 0)),
        ("orchid", CssColor::from_rgb8(218, 112, 214)),
        ("palegoldenrod", CssColor::from_rgb8(238, 232, 170)),
        ("palegreen", CssColor::from_rgb8(152, 251, 152)),
        ("paleturquoise", CssColor::from_rgb8(175, 238, 238)),
        ("palevioletred", CssColor::from_rgb8(219, 112, 147)),
        ("papayawhip", CssColor::from_rgb8(255, 239, 213)),
        ("peachpuff", CssColor::from_rgb8(255, 218, 185)),
        ("peru", CssColor::from_rgb8(205, 133, 63)),
        ("pink", CssColor::from_rgb8(255, 192, 203)),
        ("plum", CssColor::from_rgb8(221, 160, 221)),
        ("powderblue", CssColor::from_rgb8(176, 224, 230)),
        ("purple", CssColor::from_rgb8(128, 0, 128)),
        ("rebeccapurple", CssColor::from_rgb8(102, 51, 153)),
        ("red", CssColor::from_rgb8(255, 0, 0)),
        ("rosybrown", CssColor::from_rgb8(188, 143, 143)),
        ("royalblue", CssColor::from_rgb8(65, 105, 225)),
        ("saddlebrown", CssColor::from_rgb8(139, 69, 19)),
        ("salmon", CssColor::from_rgb8(250, 128, 114)),
        ("sandybrown", CssColor::from_rgb8(244, 164, 96)),
        ("seagreen", CssColor::from_rgb8(46, 139, 87)),
        ("seashell", CssColor::from_rgb8(255, 245, 238)),
        ("sienna", CssColor::from_rgb8(160, 82, 45)),
        ("silver", CssColor::from_rgb8(192, 192, 192)),
        ("skyblue", CssColor::from_rgb8(135, 206, 235)),
        ("slateblue", CssColor::from_rgb8(106, 90, 205)),
        ("slategray", CssColor::from_rgb8(112, 128, 144)),
        ("slategrey", CssColor::from_rgb8(112, 128, 144)),
        ("snow", CssColor::from_rgb8(255, 250, 250)),
        ("springgreen", CssColor::from_rgb8(0, 255, 127)),
        ("steelblue", CssColor::from_rgb8(70, 130, 180)),
        ("tan", CssColor::from_rgb8(210, 180, 140)),
        ("teal", CssColor::from_rgb8(0, 128, 128)),
        ("thistle", CssColor::from_rgb8(216, 191, 216)),
        ("tomato", CssColor::from_rgb8(255, 99, 71)),
        ("turquoise", CssColor::from_rgb8(64, 224, 208)),
        ("violet", CssColor::from_rgb8(238, 130, 238)),
        ("wheat", CssColor::from_rgb8(245, 222, 179)),
        ("white", CssColor::from_rgb8(255, 255, 255)),
        ("whitesmoke", CssColor::from_rgb8(245, 245, 245)),
        ("yellow", CssColor::from_rgb8(255, 255, 0)),
        ("yellowgreen", CssColor::from_rgb8(154, 205, 50)),
    ]))
});

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
    Vw(f32),
    Vh(f32),
    Vmin,
    Vmax,
}

impl CssDimension {
    pub const ZERO: Self = Self::Px(0.);
}

impl Display for CssDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Px(v) => write!(f, "{}px", v),
            Self::Percent(v) => write!(f, "{}%", v),
            Self::Vw(v) => write!(f, "{}vw", v),
            Self::Vh(v) => write!(f, "{}vh", v),
            Self::Vmin => write!(f, "vmin"),
            Self::Vmax => write!(f, "vmax"),
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
