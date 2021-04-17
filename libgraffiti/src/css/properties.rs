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

impl Display for StyleProp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Width(v) => write!(f, "width: {};", v),
            Self::Height(v) => write!(f, "height: {};", v),
            Self::MinWidth(v) => write!(f, "min-width: {};", v),
            Self::MinHeight(v) => write!(f, "min-height: {};", v),
            Self::MaxWidth(v) => write!(f, "max-width: {};", v),
            Self::MaxHeight(v) => write!(f, "max-height: {};", v),
        
            // padding
            Self::PaddingTop(v) => write!(f, "padding-top: {};", v),
            Self::PaddingRight(v) => write!(f, "padding-right: {};", v),
            Self::PaddingBottom(v) => write!(f, "padding-bottom: {};", v),
            Self::PaddingLeft(v) => write!(f, "padding-left: {};", v),
        
            // margin
            Self::MarginTop(v) => write!(f, "margin-top: {};", v),
            Self::MarginRight(v) => write!(f, "margin-right: {};", v),
            Self::MarginBottom(v) => write!(f, "margin-bottom: {};", v),
            Self::MarginLeft(v) => write!(f, "margin-left: {};", v),
        
            // background
            Self::BackgroundColor(v) => write!(f, "background-color: {};", v),
        
            // border-radius
            Self::BorderTopLeftRadius(v) => write!(f, "border-top-left-radius: {};", v),
            Self::BorderTopRightRadius(v) => write!(f, "border-top-right-radius: {};", v),
            Self::BorderBottomRightRadius(v) => write!(f, "border-bottom-right-radius: {};", v),
            Self::BorderBottomLeftRadius(v) => write!(f, "border-bottom-left-radius: {};", v),
        
            // border
            Self::BorderTopWidth(v) => write!(f, "border-top-width: {};", v),
            Self::BorderTopStyle(v) => write!(f, "border-top-style: {};", v),
            Self::BorderTopColor(v) => write!(f, "border-top-color: {};", v),
            Self::BorderRightWidth(v) => write!(f, "border-right-width: {};", v),
            Self::BorderRightStyle(v) => write!(f, "border-right-style: {};", v),
            Self::BorderRightColor(v) => write!(f, "border-right-color: {};", v),
            Self::BorderBottomWidth(v) => write!(f, "border-bottom-width: {};", v),
            Self::BorderBottomStyle(v) => write!(f, "border-bottom-style: {};", v),
            Self::BorderBottomColor(v) => write!(f, "border-bottom-color: {};", v),
            Self::BorderLeftWidth(v) => write!(f, "border-left-width: {};", v),
            Self::BorderLeftStyle(v) => write!(f, "border-left-style: {};", v),
            Self::BorderLeftColor(v) => write!(f, "border-left-color: {};", v),
        
            // shadow
            Self::BoxShadow(v) => write!(f, "box-shadow: {};", v),
        
            // flex
            Self::FlexBasis(v) => write!(f, "flex-basis: {};", v),
            Self::FlexGrow(v) => write!(f, "flex-grow: {};", v),
            Self::FlexShrink(v) => write!(f, "flex-shrink: {};", v),
            Self::FlexDirection(v) => write!(f, "flex-direction: {};", v),
            Self::FlexWrap(v) => write!(f, "flex-wrap: {};", v),
            Self::AlignContent(v) => write!(f, "align-content: {};", v),
            Self::AlignItems(v) => write!(f, "align-items: {};", v),
            Self::AlignSelf(v) => write!(f, "align-self: {};", v),
            Self::JustifyContent(v) => write!(f, "justify-content: {};", v),
        
            // text
            Self::FontFamily(v) => write!(f, "font-family: {};", v),
            Self::FontSize(v) => write!(f, "font-size: {};", v),
            Self::LineHeight(v) => write!(f, "line-height: {};", v),
            Self::TextAlign(v) => write!(f, "text-align: {};", v),
            Self::Color(v) => write!(f, "color: {};", v),
        
            // outline
            Self::OutlineColor(v) => write!(f, "outline-color: {};", v),
            Self::OutlineStyle(v) => write!(f, "outline-style: {};", v),
            Self::OutlineWidth(v) => write!(f, "outline-width: {};", v),
        
            // overflow
            Self::OverflowX(v) => write!(f, "overflow-x: {};", v),
            Self::OverflowY(v) => write!(f, "overflow-y: {};", v),
        
            // position
            Self::Position(v) => write!(f, "position: {};", v),
            Self::Top(v) => write!(f, "top: {};", v),
            Self::Right(v) => write!(f, "right: {};", v),
            Self::Bottom(v) => write!(f, "bottom: {};", v),
            Self::Left(v) => write!(f, "left: {};", v),
        
            // other
            Self::Display(v) => write!(f, "display: {};", v),
            Self::Opacity(v) => write!(f, "opacity: {};", v),
            Self::Visibility(v) => write!(f, "visibility: {};", v),            
        }
    }
}
