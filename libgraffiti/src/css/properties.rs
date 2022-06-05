// supported CSS props (longhand)

use super::parsing::{fail, ident, sym, Parsable, Parser};
use super::{
    Align, BorderStyle, BoxShadow, Color, Dimension, Display, FlexDirection, FlexWrap, Justify, Overflow, Position, Px,
    TextAlign, Transform, Visibility,
};
use std::fmt;

macro_rules! css_properties {
    ($($name:literal => $variant:ident($value_type:ty),)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum StyleProp {
            $($variant($value_type),)*
        }

        impl Parsable for StyleProp {
            fn parser<'a>() -> Parser<'a, Self> {
                ident() - sym(":") >> |name| {
                    match name {
                        $($name => <$value_type>::parser().map(StyleProp::$variant),)*
                        _ => fail("unknown prop")
                    }
                }
            }
        }

        impl fmt::Display for StyleProp {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant(v) => write!(f, "{}: {}", $name, v)?),*
                }

                Ok(())
            }
        }

    }
}

// (name, parser_name) => Variant(ValueType)
css_properties! {
    // size
    "width" => Width(Dimension),
    "height" => Height(Dimension),
    "min-width" => MinWidth(Dimension),
    "min-height" => MinHeight(Dimension),
    "max-width" => MaxWidth(Dimension),
    "max-height" => MaxHeight(Dimension),

    // padding
    "padding-top" => PaddingTop(Dimension),
    "padding-right" => PaddingRight(Dimension),
    "padding-bottom" => PaddingBottom(Dimension),
    "padding-left" => PaddingLeft(Dimension),

    // margin
    "margin-top" => MarginTop(Dimension),
    "margin-right" => MarginRight(Dimension),
    "margin-bottom" => MarginBottom(Dimension),
    "margin-left" => MarginLeft(Dimension),

    // background
    "background-color" => BackgroundColor(Color),

    // border-radius
    "border-top-left-radius" => BorderTopLeftRadius(Px),
    "border-top-right-radius" => BorderTopRightRadius(Px),
    "border-bottom-right-radius" => BorderBottomRightRadius(Px),
    "border-bottom-left-radius" => BorderBottomLeftRadius(Px),

    // border
    "border-top-width" => BorderTopWidth(Px),
    "border-top-style" => BorderTopStyle(BorderStyle),
    "border-top-color" => BorderTopColor(Color),
    "border-right-width" => BorderRightWidth(Px),
    "border-right-style" => BorderRightStyle(BorderStyle),
    "border-right-color" => BorderRightColor(Color),
    "border-bottom-width" => BorderBottomWidth(Px),
    "border-bottom-style" => BorderBottomStyle(BorderStyle),
    "border-bottom-color" => BorderBottomColor(Color),
    "border-left-width" => BorderLeftWidth(Px),
    "border-left-style" => BorderLeftStyle(BorderStyle),
    "border-left-color" => BorderLeftColor(Color),

    // shadow
    "box-shadow" => BoxShadow(BoxShadow),

    // flex
    "flex-grow" => FlexGrow(f32),
    "flex-shrink" => FlexShrink(f32),
    "flex-basis" => FlexBasis(Dimension),
    "flex-direction" => FlexDirection(FlexDirection),
    "flex-wrap" => FlexWrap(FlexWrap),
    "align-content" => AlignContent(Align),
    "align-items" => AlignItems(Align),
    "align-self" => AlignSelf(Align),
    "justify-content" => JustifyContent(Justify),

    // text
    "font-family" => FontFamily(String),
    "font-size" => FontSize(Dimension),
    "line-height" => LineHeight(Dimension),
    "text-align" => TextAlign(TextAlign),
    "color" => Color(Color),

    // outline
    "outline-color" => OutlineColor(Color),
    "outline-style" => OutlineStyle(BorderStyle),
    "outline-width" => OutlineWidth(Px),

    // overflow
    "overflow-x" => OverflowX(Overflow),
    "overflow-y" => OverflowY(Overflow),

    // position
    "position" => Position(Position),
    "top" => Top(Dimension),
    "right" => Right(Dimension),
    "bottom" => Bottom(Dimension),
    "left" => Left(Dimension),

    // other
    "display" => Display(Display),
    "opacity" => Opacity(f32),
    "visibility" => Visibility(Visibility),
    "transform" => Transform(Transform),
}
