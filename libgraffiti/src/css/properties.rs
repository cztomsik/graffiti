// supported CSS props (longhand)

use std::fmt::{Display, Error, Formatter};

use super::values::*;
use crate::util::Atom;

// type shorthand
type V<T> = CssValue<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum StyleProp {
    // size
    Width(V<CssDimension>),
    Height(V<CssDimension>),
    MinWidth(V<CssDimension>),
    MinHeight(V<CssDimension>),
    MaxWidth(V<CssDimension>),
    MaxHeight(V<CssDimension>),

    // padding
    PaddingTop(V<CssDimension>),
    PaddingRight(V<CssDimension>),
    PaddingBottom(V<CssDimension>),
    PaddingLeft(V<CssDimension>),

    // margin
    MarginTop(V<CssDimension>),
    MarginRight(V<CssDimension>),
    MarginBottom(V<CssDimension>),
    MarginLeft(V<CssDimension>),

    // background
    BackgroundColor(V<CssColor>),

    // border-radius
    BorderTopLeftRadius(V<CssDimension>),
    BorderTopRightRadius(V<CssDimension>),
    BorderBottomRightRadius(V<CssDimension>),
    BorderBottomLeftRadius(V<CssDimension>),

    // border
    BorderTopWidth(V<CssDimension>),
    BorderTopStyle(V<CssBorderStyle>),
    BorderTopColor(V<CssColor>),
    BorderRightWidth(V<CssDimension>),
    BorderRightStyle(V<CssBorderStyle>),
    BorderRightColor(V<CssColor>),
    BorderBottomWidth(V<CssDimension>),
    BorderBottomStyle(V<CssBorderStyle>),
    BorderBottomColor(V<CssColor>),
    BorderLeftWidth(V<CssDimension>),
    BorderLeftStyle(V<CssBorderStyle>),
    BorderLeftColor(V<CssColor>),

    // shadow
    BoxShadow(V<CssBoxShadow>),

    // flex
    FlexBasis(V<CssDimension>),
    FlexGrow(V<f32>),
    FlexShrink(V<f32>),
    FlexDirection(V<CssFlexDirection>),
    FlexWrap(V<CssFlexWrap>),
    AlignContent(V<CssAlign>),
    AlignItems(V<CssAlign>),
    AlignSelf(V<CssAlign>),
    JustifyContent(V<CssAlign>),

    // text
    FontFamily(V<Atom<String>>),
    FontSize(V<CssDimension>),
    LineHeight(V<CssDimension>),
    TextAlign(V<CssTextAlign>),
    Color(V<CssColor>),

    // outline
    OutlineColor(V<CssColor>),
    OutlineStyle(V<CssBorderStyle>),
    OutlineWidth(V<CssDimension>),

    // overflow
    OverflowX(V<CssOverflow>),
    OverflowY(V<CssOverflow>),

    // position
    Position(V<CssPosition>),
    Top(V<CssDimension>),
    Right(V<CssDimension>),
    Bottom(V<CssDimension>),
    Left(V<CssDimension>),

    // other
    Display(V<CssDisplay>),
    Opacity(V<f32>),
    Visibility(V<CssVisibility>),
}

impl StyleProp {
    pub fn name(&self) -> &'static str {
        use StyleProp::*;

        match self {
            Width(_) => "width",
            Height(_) => "height",
            MinWidth(_) => "min-width",
            MinHeight(_) => "min-height",
            MaxWidth(_) => "max-width",
            MaxHeight(_) => "max-height",

            // padding
            PaddingTop(_) => "padding-top",
            PaddingRight(_) => "padding-right",
            PaddingBottom(_) => "padding-bottom",
            PaddingLeft(_) => "padding-left",

            // margin
            MarginTop(_) => "margin-top",
            MarginRight(_) => "margin-right",
            MarginBottom(_) => "margin-bottom",
            MarginLeft(_) => "margin-left",

            // background
            BackgroundColor(_) => "background-color",

            // border-radius
            BorderTopLeftRadius(_) => "border-top-left-radius",
            BorderTopRightRadius(_) => "border-top-right-radius",
            BorderBottomRightRadius(_) => "border-bottom-right-radius",
            BorderBottomLeftRadius(_) => "border-bottom-left-radius",

            // border
            BorderTopWidth(_) => "border-top-width",
            BorderTopStyle(_) => "border-top-style",
            BorderTopColor(_) => "border-top-color",
            BorderRightWidth(_) => "border-right-width",
            BorderRightStyle(_) => "border-right-style",
            BorderRightColor(_) => "border-right-color",
            BorderBottomWidth(_) => "border-bottom-width",
            BorderBottomStyle(_) => "border-bottom-style",
            BorderBottomColor(_) => "border-bottom-color",
            BorderLeftWidth(_) => "border-left-width",
            BorderLeftStyle(_) => "border-left-style",
            BorderLeftColor(_) => "border-left-color",

            // shadow
            BoxShadow(_) => "box-shadow",

            // flex
            FlexBasis(_) => "flex-basis",
            FlexGrow(_) => "flex-grow",
            FlexShrink(_) => "flex-shrink",
            FlexDirection(_) => "flex-direction",
            FlexWrap(_) => "flex-wrap",
            AlignContent(_) => "align-content",
            AlignItems(_) => "align-items",
            AlignSelf(_) => "align-self",
            JustifyContent(_) => "justify-content",

            // text
            FontFamily(_) => "font-family",
            FontSize(_) => "font-size",
            LineHeight(_) => "line-height",
            TextAlign(_) => "text-align",
            Color(_) => "color",

            // outline
            OutlineColor(_) => "outline-color",
            OutlineStyle(_) => "outline-style",
            OutlineWidth(_) => "outline-width",

            // overflow
            OverflowX(_) => "overflow-x",
            OverflowY(_) => "overflow-y",

            // position
            Position(_) => "position",
            Top(_) => "top",
            Right(_) => "right",
            Bottom(_) => "bottom",
            Left(_) => "left",

            // other
            Display(_) => "display",
            Opacity(_) => "opacity",
            Visibility(_) => "visibility",
        }
    }

    pub fn value(&self) -> String {
        use StyleProp::*;

        match self {
            // Dimension
            Width(v)
            | Height(v)
            | MinWidth(v)
            | MinHeight(v)
            | MaxWidth(v)
            | MaxHeight(v)
            | PaddingTop(v)
            | PaddingRight(v)
            | PaddingBottom(v)
            | PaddingLeft(v)
            | MarginTop(v)
            | FontSize(v)
            | LineHeight(v)
            | FlexBasis(v)
            | MarginRight(v)
            | MarginBottom(v)
            | MarginLeft(v)
            | BorderTopLeftRadius(v)
            | BorderTopRightRadius(v)
            | BorderBottomRightRadius(v)
            | BorderBottomLeftRadius(v)
            | BorderTopWidth(v)
            | BorderRightWidth(v)
            | BorderBottomWidth(v)
            | BorderLeftWidth(v)
            | OutlineWidth(v)
            | Top(v)
            | Right(v)
            | Bottom(v)
            | Left(v) => format!("{}", v),

            // Color
            BackgroundColor(v) | BorderTopColor(v) | BorderRightColor(v) | BorderBottomColor(v)
            | BorderLeftColor(v) | Color(v) | OutlineColor(v) => format!("{}", v),

            // BorderStyle
            OutlineStyle(v) | BorderTopStyle(v) | BorderRightStyle(v) | BorderBottomStyle(v) | BorderLeftStyle(v) => {
                format!("{}", v)
            }

            // f32
            Opacity(v) | FlexGrow(v) | FlexShrink(v) => format!("{}", v),

            // Align
            AlignContent(v) | AlignItems(v) | AlignSelf(v) | JustifyContent(v) => format!("{}", v),

            // Others
            TextAlign(v) => format!("{}", v),
            FlexDirection(v) => format!("{}", v),
            FlexWrap(v) => format!("{}", v),
            Position(v) => format!("{}", v),
            Visibility(v) => format!("{}", v),
            Display(v) => format!("{}", v),
            BoxShadow(v) => format!("{}", v),
            FontFamily(v) => format!("{}", v),
            OverflowX(v) | OverflowY(v) => format!("{}", v),
        }        
    }
}

impl Display for StyleProp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {};", self.name(), self.value())
    }
}
