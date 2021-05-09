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

impl CssColor {
    // just a few for easier testing
    pub const TRANSPARENT: Self = Self::from_rgba8(0, 0, 0, 0);
    pub const BLACK: Self = Self::from_rgb8(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb8(255, 255, 255);
    pub const RED: Self = Self::from_rgb8(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb8(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb8(0, 0, 255);

    pub(super) const NAMED_COLORS: Lazy<HashMap<&'static str, Self>> = Lazy::new(|| {
        HashMap::from_iter(IntoIter::new([
            ("transparent", Self::from_rgba8(0, 0, 0, 0)),
            // https://drafts.csswg.org/css-color/#named-colors
            ("aliceblue", Self::from_rgb8(240, 248, 255)),
            ("antiquewhite", Self::from_rgb8(250, 235, 215)),
            ("aqua", Self::from_rgb8(0, 255, 255)),
            ("aquamarine", Self::from_rgb8(127, 255, 212)),
            ("azure", Self::from_rgb8(240, 255, 255)),
            ("beige", Self::from_rgb8(245, 245, 220)),
            ("bisque", Self::from_rgb8(255, 228, 196)),
            ("black", Self::from_rgb8(0, 0, 0)),
            ("blanchedalmond", Self::from_rgb8(255, 235, 205)),
            ("blue", Self::from_rgb8(0, 0, 255)),
            ("blueviolet", Self::from_rgb8(138, 43, 226)),
            ("brown", Self::from_rgb8(165, 42, 42)),
            ("burlywood", Self::from_rgb8(222, 184, 135)),
            ("cadetblue", Self::from_rgb8(95, 158, 160)),
            ("chartreuse", Self::from_rgb8(127, 255, 0)),
            ("chocolate", Self::from_rgb8(210, 105, 30)),
            ("coral", Self::from_rgb8(255, 127, 80)),
            ("cornflowerblue", Self::from_rgb8(100, 149, 237)),
            ("cornsilk", Self::from_rgb8(255, 248, 220)),
            ("crimson", Self::from_rgb8(220, 20, 60)),
            ("cyan", Self::from_rgb8(0, 255, 255)),
            ("darkblue", Self::from_rgb8(0, 0, 139)),
            ("darkcyan", Self::from_rgb8(0, 139, 139)),
            ("darkgoldenrod", Self::from_rgb8(184, 134, 11)),
            ("darkgray", Self::from_rgb8(169, 169, 169)),
            ("darkgreen", Self::from_rgb8(0, 100, 0)),
            ("darkgrey", Self::from_rgb8(169, 169, 169)),
            ("darkkhaki", Self::from_rgb8(189, 183, 107)),
            ("darkmagenta", Self::from_rgb8(139, 0, 139)),
            ("darkolivegreen", Self::from_rgb8(85, 107, 47)),
            ("darkorange", Self::from_rgb8(255, 140, 0)),
            ("darkorchid", Self::from_rgb8(153, 50, 204)),
            ("darkred", Self::from_rgb8(139, 0, 0)),
            ("darksalmon", Self::from_rgb8(233, 150, 122)),
            ("darkseagreen", Self::from_rgb8(143, 188, 143)),
            ("darkslateblue", Self::from_rgb8(72, 61, 139)),
            ("darkslategray", Self::from_rgb8(47, 79, 79)),
            ("darkslategrey", Self::from_rgb8(47, 79, 79)),
            ("darkturquoise", Self::from_rgb8(0, 206, 209)),
            ("darkviolet", Self::from_rgb8(148, 0, 211)),
            ("deeppink", Self::from_rgb8(255, 20, 147)),
            ("deepskyblue", Self::from_rgb8(0, 191, 255)),
            ("dimgray", Self::from_rgb8(105, 105, 105)),
            ("dimgrey", Self::from_rgb8(105, 105, 105)),
            ("dodgerblue", Self::from_rgb8(30, 144, 255)),
            ("firebrick", Self::from_rgb8(178, 34, 34)),
            ("floralwhite", Self::from_rgb8(255, 250, 240)),
            ("forestgreen", Self::from_rgb8(34, 139, 34)),
            ("fuchsia", Self::from_rgb8(255, 0, 255)),
            ("gainsboro", Self::from_rgb8(220, 220, 220)),
            ("ghostwhite", Self::from_rgb8(248, 248, 255)),
            ("gold", Self::from_rgb8(255, 215, 0)),
            ("goldenrod", Self::from_rgb8(218, 165, 32)),
            ("gray", Self::from_rgb8(128, 128, 128)),
            ("green", Self::from_rgb8(0, 128, 0)),
            ("greenyellow", Self::from_rgb8(173, 255, 47)),
            ("grey", Self::from_rgb8(128, 128, 128)),
            ("honeydew", Self::from_rgb8(240, 255, 240)),
            ("hotpink", Self::from_rgb8(255, 105, 180)),
            ("indianred", Self::from_rgb8(205, 92, 92)),
            ("indigo", Self::from_rgb8(75, 0, 130)),
            ("ivory", Self::from_rgb8(255, 255, 240)),
            ("khaki", Self::from_rgb8(240, 230, 140)),
            ("lavender", Self::from_rgb8(230, 230, 250)),
            ("lavenderblush", Self::from_rgb8(255, 240, 245)),
            ("lawngreen", Self::from_rgb8(124, 252, 0)),
            ("lemonchiffon", Self::from_rgb8(255, 250, 205)),
            ("lightblue", Self::from_rgb8(173, 216, 230)),
            ("lightcoral", Self::from_rgb8(240, 128, 128)),
            ("lightcyan", Self::from_rgb8(224, 255, 255)),
            ("lightgoldenrodyellow", Self::from_rgb8(250, 250, 210)),
            ("lightgray", Self::from_rgb8(211, 211, 211)),
            ("lightgreen", Self::from_rgb8(144, 238, 144)),
            ("lightgrey", Self::from_rgb8(211, 211, 211)),
            ("lightpink", Self::from_rgb8(255, 182, 193)),
            ("lightsalmon", Self::from_rgb8(255, 160, 122)),
            ("lightseagreen", Self::from_rgb8(32, 178, 170)),
            ("lightskyblue", Self::from_rgb8(135, 206, 250)),
            ("lightslategray", Self::from_rgb8(119, 136, 153)),
            ("lightslategrey", Self::from_rgb8(119, 136, 153)),
            ("lightsteelblue", Self::from_rgb8(176, 196, 222)),
            ("lightyellow", Self::from_rgb8(255, 255, 224)),
            ("lime", Self::from_rgb8(0, 255, 0)),
            ("limegreen", Self::from_rgb8(50, 205, 50)),
            ("linen", Self::from_rgb8(250, 240, 230)),
            ("magenta", Self::from_rgb8(255, 0, 255)),
            ("maroon", Self::from_rgb8(128, 0, 0)),
            ("mediumaquamarine", Self::from_rgb8(102, 205, 170)),
            ("mediumblue", Self::from_rgb8(0, 0, 205)),
            ("mediumorchid", Self::from_rgb8(186, 85, 211)),
            ("mediumpurple", Self::from_rgb8(147, 112, 219)),
            ("mediumseagreen", Self::from_rgb8(60, 179, 113)),
            ("mediumslateblue", Self::from_rgb8(123, 104, 238)),
            ("mediumspringgreen", Self::from_rgb8(0, 250, 154)),
            ("mediumturquoise", Self::from_rgb8(72, 209, 204)),
            ("mediumvioletred", Self::from_rgb8(199, 21, 133)),
            ("midnightblue", Self::from_rgb8(25, 25, 112)),
            ("mintcream", Self::from_rgb8(245, 255, 250)),
            ("mistyrose", Self::from_rgb8(255, 228, 225)),
            ("moccasin", Self::from_rgb8(255, 228, 181)),
            ("navajowhite", Self::from_rgb8(255, 222, 173)),
            ("navy", Self::from_rgb8(0, 0, 128)),
            ("oldlace", Self::from_rgb8(253, 245, 230)),
            ("olive", Self::from_rgb8(128, 128, 0)),
            ("olivedrab", Self::from_rgb8(107, 142, 35)),
            ("orange", Self::from_rgb8(255, 165, 0)),
            ("orangered", Self::from_rgb8(255, 69, 0)),
            ("orchid", Self::from_rgb8(218, 112, 214)),
            ("palegoldenrod", Self::from_rgb8(238, 232, 170)),
            ("palegreen", Self::from_rgb8(152, 251, 152)),
            ("paleturquoise", Self::from_rgb8(175, 238, 238)),
            ("palevioletred", Self::from_rgb8(219, 112, 147)),
            ("papayawhip", Self::from_rgb8(255, 239, 213)),
            ("peachpuff", Self::from_rgb8(255, 218, 185)),
            ("peru", Self::from_rgb8(205, 133, 63)),
            ("pink", Self::from_rgb8(255, 192, 203)),
            ("plum", Self::from_rgb8(221, 160, 221)),
            ("powderblue", Self::from_rgb8(176, 224, 230)),
            ("purple", Self::from_rgb8(128, 0, 128)),
            ("rebeccapurple", Self::from_rgb8(102, 51, 153)),
            ("red", Self::from_rgb8(255, 0, 0)),
            ("rosybrown", Self::from_rgb8(188, 143, 143)),
            ("royalblue", Self::from_rgb8(65, 105, 225)),
            ("saddlebrown", Self::from_rgb8(139, 69, 19)),
            ("salmon", Self::from_rgb8(250, 128, 114)),
            ("sandybrown", Self::from_rgb8(244, 164, 96)),
            ("seagreen", Self::from_rgb8(46, 139, 87)),
            ("seashell", Self::from_rgb8(255, 245, 238)),
            ("sienna", Self::from_rgb8(160, 82, 45)),
            ("silver", Self::from_rgb8(192, 192, 192)),
            ("skyblue", Self::from_rgb8(135, 206, 235)),
            ("slateblue", Self::from_rgb8(106, 90, 205)),
            ("slategray", Self::from_rgb8(112, 128, 144)),
            ("slategrey", Self::from_rgb8(112, 128, 144)),
            ("snow", Self::from_rgb8(255, 250, 250)),
            ("springgreen", Self::from_rgb8(0, 255, 127)),
            ("steelblue", Self::from_rgb8(70, 130, 180)),
            ("tan", Self::from_rgb8(210, 180, 140)),
            ("teal", Self::from_rgb8(0, 128, 128)),
            ("thistle", Self::from_rgb8(216, 191, 216)),
            ("tomato", Self::from_rgb8(255, 99, 71)),
            ("turquoise", Self::from_rgb8(64, 224, 208)),
            ("violet", Self::from_rgb8(238, 130, 238)),
            ("wheat", Self::from_rgb8(245, 222, 179)),
            ("white", Self::from_rgb8(255, 255, 255)),
            ("whitesmoke", Self::from_rgb8(245, 245, 245)),
            ("yellow", Self::from_rgb8(255, 255, 0)),
            ("yellowgreen", Self::from_rgb8(154, 205, 50)),
        ]))
    });

    pub(super) const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }

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

impl CssDimension {
    pub const ZERO: Self = Self::Px(0.);
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
