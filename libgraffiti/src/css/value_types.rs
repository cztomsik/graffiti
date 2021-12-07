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
        InlineBlock = "inline-block",
        Flex = "flex",
        Table = "table",
        TableHeaderGroup = "table-header-group",
        TableRowGroup = "table-row-group",
        TableRow = "table-row",
        TableCell = "table-cell",
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
    // just a few for easier testing
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);

    pub(super) const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub(super) const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub(super) static NAMED_COLORS: Lazy<HashMap<&'static str, CssColor>> = Lazy::new(|| {
    HashMap::from_iter(IntoIter::new([
        ("transparent", CssColor::rgba(0, 0, 0, 0)),
        // https://drafts.csswg.org/css-color/#named-colors
        ("aliceblue", CssColor::rgb(240, 248, 255)),
        ("antiquewhite", CssColor::rgb(250, 235, 215)),
        ("aqua", CssColor::rgb(0, 255, 255)),
        ("aquamarine", CssColor::rgb(127, 255, 212)),
        ("azure", CssColor::rgb(240, 255, 255)),
        ("beige", CssColor::rgb(245, 245, 220)),
        ("bisque", CssColor::rgb(255, 228, 196)),
        ("black", CssColor::rgb(0, 0, 0)),
        ("blanchedalmond", CssColor::rgb(255, 235, 205)),
        ("blue", CssColor::rgb(0, 0, 255)),
        ("blueviolet", CssColor::rgb(138, 43, 226)),
        ("brown", CssColor::rgb(165, 42, 42)),
        ("burlywood", CssColor::rgb(222, 184, 135)),
        ("cadetblue", CssColor::rgb(95, 158, 160)),
        ("chartreuse", CssColor::rgb(127, 255, 0)),
        ("chocolate", CssColor::rgb(210, 105, 30)),
        ("coral", CssColor::rgb(255, 127, 80)),
        ("cornflowerblue", CssColor::rgb(100, 149, 237)),
        ("cornsilk", CssColor::rgb(255, 248, 220)),
        ("crimson", CssColor::rgb(220, 20, 60)),
        ("cyan", CssColor::rgb(0, 255, 255)),
        ("darkblue", CssColor::rgb(0, 0, 139)),
        ("darkcyan", CssColor::rgb(0, 139, 139)),
        ("darkgoldenrod", CssColor::rgb(184, 134, 11)),
        ("darkgray", CssColor::rgb(169, 169, 169)),
        ("darkgreen", CssColor::rgb(0, 100, 0)),
        ("darkgrey", CssColor::rgb(169, 169, 169)),
        ("darkkhaki", CssColor::rgb(189, 183, 107)),
        ("darkmagenta", CssColor::rgb(139, 0, 139)),
        ("darkolivegreen", CssColor::rgb(85, 107, 47)),
        ("darkorange", CssColor::rgb(255, 140, 0)),
        ("darkorchid", CssColor::rgb(153, 50, 204)),
        ("darkred", CssColor::rgb(139, 0, 0)),
        ("darksalmon", CssColor::rgb(233, 150, 122)),
        ("darkseagreen", CssColor::rgb(143, 188, 143)),
        ("darkslateblue", CssColor::rgb(72, 61, 139)),
        ("darkslategray", CssColor::rgb(47, 79, 79)),
        ("darkslategrey", CssColor::rgb(47, 79, 79)),
        ("darkturquoise", CssColor::rgb(0, 206, 209)),
        ("darkviolet", CssColor::rgb(148, 0, 211)),
        ("deeppink", CssColor::rgb(255, 20, 147)),
        ("deepskyblue", CssColor::rgb(0, 191, 255)),
        ("dimgray", CssColor::rgb(105, 105, 105)),
        ("dimgrey", CssColor::rgb(105, 105, 105)),
        ("dodgerblue", CssColor::rgb(30, 144, 255)),
        ("firebrick", CssColor::rgb(178, 34, 34)),
        ("floralwhite", CssColor::rgb(255, 250, 240)),
        ("forestgreen", CssColor::rgb(34, 139, 34)),
        ("fuchsia", CssColor::rgb(255, 0, 255)),
        ("gainsboro", CssColor::rgb(220, 220, 220)),
        ("ghostwhite", CssColor::rgb(248, 248, 255)),
        ("gold", CssColor::rgb(255, 215, 0)),
        ("goldenrod", CssColor::rgb(218, 165, 32)),
        ("gray", CssColor::rgb(128, 128, 128)),
        ("green", CssColor::rgb(0, 128, 0)),
        ("greenyellow", CssColor::rgb(173, 255, 47)),
        ("grey", CssColor::rgb(128, 128, 128)),
        ("honeydew", CssColor::rgb(240, 255, 240)),
        ("hotpink", CssColor::rgb(255, 105, 180)),
        ("indianred", CssColor::rgb(205, 92, 92)),
        ("indigo", CssColor::rgb(75, 0, 130)),
        ("ivory", CssColor::rgb(255, 255, 240)),
        ("khaki", CssColor::rgb(240, 230, 140)),
        ("lavender", CssColor::rgb(230, 230, 250)),
        ("lavenderblush", CssColor::rgb(255, 240, 245)),
        ("lawngreen", CssColor::rgb(124, 252, 0)),
        ("lemonchiffon", CssColor::rgb(255, 250, 205)),
        ("lightblue", CssColor::rgb(173, 216, 230)),
        ("lightcoral", CssColor::rgb(240, 128, 128)),
        ("lightcyan", CssColor::rgb(224, 255, 255)),
        ("lightgoldenrodyellow", CssColor::rgb(250, 250, 210)),
        ("lightgray", CssColor::rgb(211, 211, 211)),
        ("lightgreen", CssColor::rgb(144, 238, 144)),
        ("lightgrey", CssColor::rgb(211, 211, 211)),
        ("lightpink", CssColor::rgb(255, 182, 193)),
        ("lightsalmon", CssColor::rgb(255, 160, 122)),
        ("lightseagreen", CssColor::rgb(32, 178, 170)),
        ("lightskyblue", CssColor::rgb(135, 206, 250)),
        ("lightslategray", CssColor::rgb(119, 136, 153)),
        ("lightslategrey", CssColor::rgb(119, 136, 153)),
        ("lightsteelblue", CssColor::rgb(176, 196, 222)),
        ("lightyellow", CssColor::rgb(255, 255, 224)),
        ("lime", CssColor::rgb(0, 255, 0)),
        ("limegreen", CssColor::rgb(50, 205, 50)),
        ("linen", CssColor::rgb(250, 240, 230)),
        ("magenta", CssColor::rgb(255, 0, 255)),
        ("maroon", CssColor::rgb(128, 0, 0)),
        ("mediumaquamarine", CssColor::rgb(102, 205, 170)),
        ("mediumblue", CssColor::rgb(0, 0, 205)),
        ("mediumorchid", CssColor::rgb(186, 85, 211)),
        ("mediumpurple", CssColor::rgb(147, 112, 219)),
        ("mediumseagreen", CssColor::rgb(60, 179, 113)),
        ("mediumslateblue", CssColor::rgb(123, 104, 238)),
        ("mediumspringgreen", CssColor::rgb(0, 250, 154)),
        ("mediumturquoise", CssColor::rgb(72, 209, 204)),
        ("mediumvioletred", CssColor::rgb(199, 21, 133)),
        ("midnightblue", CssColor::rgb(25, 25, 112)),
        ("mintcream", CssColor::rgb(245, 255, 250)),
        ("mistyrose", CssColor::rgb(255, 228, 225)),
        ("moccasin", CssColor::rgb(255, 228, 181)),
        ("navajowhite", CssColor::rgb(255, 222, 173)),
        ("navy", CssColor::rgb(0, 0, 128)),
        ("oldlace", CssColor::rgb(253, 245, 230)),
        ("olive", CssColor::rgb(128, 128, 0)),
        ("olivedrab", CssColor::rgb(107, 142, 35)),
        ("orange", CssColor::rgb(255, 165, 0)),
        ("orangered", CssColor::rgb(255, 69, 0)),
        ("orchid", CssColor::rgb(218, 112, 214)),
        ("palegoldenrod", CssColor::rgb(238, 232, 170)),
        ("palegreen", CssColor::rgb(152, 251, 152)),
        ("paleturquoise", CssColor::rgb(175, 238, 238)),
        ("palevioletred", CssColor::rgb(219, 112, 147)),
        ("papayawhip", CssColor::rgb(255, 239, 213)),
        ("peachpuff", CssColor::rgb(255, 218, 185)),
        ("peru", CssColor::rgb(205, 133, 63)),
        ("pink", CssColor::rgb(255, 192, 203)),
        ("plum", CssColor::rgb(221, 160, 221)),
        ("powderblue", CssColor::rgb(176, 224, 230)),
        ("purple", CssColor::rgb(128, 0, 128)),
        ("rebeccapurple", CssColor::rgb(102, 51, 153)),
        ("red", CssColor::rgb(255, 0, 0)),
        ("rosybrown", CssColor::rgb(188, 143, 143)),
        ("royalblue", CssColor::rgb(65, 105, 225)),
        ("saddlebrown", CssColor::rgb(139, 69, 19)),
        ("salmon", CssColor::rgb(250, 128, 114)),
        ("sandybrown", CssColor::rgb(244, 164, 96)),
        ("seagreen", CssColor::rgb(46, 139, 87)),
        ("seashell", CssColor::rgb(255, 245, 238)),
        ("sienna", CssColor::rgb(160, 82, 45)),
        ("silver", CssColor::rgb(192, 192, 192)),
        ("skyblue", CssColor::rgb(135, 206, 235)),
        ("slateblue", CssColor::rgb(106, 90, 205)),
        ("slategray", CssColor::rgb(112, 128, 144)),
        ("slategrey", CssColor::rgb(112, 128, 144)),
        ("snow", CssColor::rgb(255, 250, 250)),
        ("springgreen", CssColor::rgb(0, 255, 127)),
        ("steelblue", CssColor::rgb(70, 130, 180)),
        ("tan", CssColor::rgb(210, 180, 140)),
        ("teal", CssColor::rgb(0, 128, 128)),
        ("thistle", CssColor::rgb(216, 191, 216)),
        ("tomato", CssColor::rgb(255, 99, 71)),
        ("turquoise", CssColor::rgb(64, 224, 208)),
        ("violet", CssColor::rgb(238, 130, 238)),
        ("wheat", CssColor::rgb(245, 222, 179)),
        ("white", CssColor::rgb(255, 255, 255)),
        ("whitesmoke", CssColor::rgb(245, 245, 245)),
        ("yellow", CssColor::rgb(255, 255, 0)),
        ("yellowgreen", CssColor::rgb(154, 205, 50)),
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
    Em(f32),
    Rem(f32),
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
            Self::Em(v) => write!(f, "{}em", v),
            Self::Rem(v) => write!(f, "{}rem", v),
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
