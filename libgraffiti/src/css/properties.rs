// supported CSS props (longhand)

use super::values::*;
use crate::util::Atom;

// type shorthand
type V<T> = Value<T>;

#[derive(Debug, Clone, PartialEq)]
pub enum StyleProp {
    // size
    Width(V<Dimension>),
    Height(V<Dimension>),
    MinWidth(V<Dimension>),
    MinHeight(V<Dimension>),
    MaxWidth(V<Dimension>),
    MaxHeight(V<Dimension>),

    // padding
    PaddingTop(V<Dimension>),
    PaddingRight(V<Dimension>),
    PaddingBottom(V<Dimension>),
    PaddingLeft(V<Dimension>),

    // margin
    MarginTop(V<Dimension>),
    MarginRight(V<Dimension>),
    MarginBottom(V<Dimension>),
    MarginLeft(V<Dimension>),

    // background
    BackgroundColor(V<Color>),

    // border-radius
    BorderTopLeftRadius(V<Dimension>),
    BorderTopRightRadius(V<Dimension>),
    BorderBottomRightRadius(V<Dimension>),
    BorderBottomLeftRadius(V<Dimension>),

    // border
    BorderTopWidth(V<Dimension>),
    BorderTopStyle(V<BorderStyle>),
    BorderTopColor(V<Color>),
    BorderRightWidth(V<Dimension>),
    BorderRightStyle(V<BorderStyle>),
    BorderRightColor(V<Color>),
    BorderBottomWidth(V<Dimension>),
    BorderBottomStyle(V<BorderStyle>),
    BorderBottomColor(V<Color>),
    BorderLeftWidth(V<Dimension>),
    BorderLeftStyle(V<BorderStyle>),
    BorderLeftColor(V<Color>),

    // shadow
    BoxShadow(V<BoxShadow>),

    // flex
    FlexBasis(V<Dimension>),
    FlexGrow(V<f32>),
    FlexShrink(V<f32>),
    FlexDirection(V<FlexDirection>),
    FlexWrap(V<FlexWrap>),
    AlignContent(V<Align>),
    AlignItems(V<Align>),
    AlignSelf(V<Align>),
    JustifyContent(V<Align>),

    // text
    FontFamily(V<Atom<String>>),
    FontSize(V<Dimension>),
    LineHeight(V<Dimension>),
    TextAlign(V<TextAlign>),
    Color(V<Color>),

    // outline
    OutlineColor(V<Color>),
    OutlineStyle(V<BorderStyle>),
    OutlineWidth(V<Dimension>),

    // overflow
    OverflowX(V<Overflow>),
    OverflowY(V<Overflow>),

    // position
    Position(V<Position>),
    Top(V<Dimension>),
    Right(V<Dimension>),
    Bottom(V<Dimension>),
    Left(V<Dimension>),

    // other
    Display(V<Display>),
    Opacity(V<f32>),
    Visibility(V<Visibility>),
}
