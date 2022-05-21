// supported CSS props

use super::parsing::{fail, Parsable, Parser};
use super::{
    Align, BorderStyle, Color, Dimension, Display, FlexDirection, FlexWrap, Justify, Overflow, Position, TextAlign,
    Visibility,
};

macro_rules! css_properties {
    ($($name:literal => $variant:ident($value_type:ty),)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum StyleProp {
            $($variant($value_type),)*
        }

        impl StyleProp {
            pub fn css_name(&self) -> &'static str {
                match self {
                    $(Self::$variant(_) => $name,)*
                }
            }

            pub fn css_value(&self) -> String {
                let v: &dyn std::fmt::Display = match self {
                    $(Self::$variant(ref v) => v),*
                };

                format!("{}", v)
            }
        }

        pub(super) fn prop_parser<'a>(prop: &str) -> Parser<'a, StyleProp> {
            match prop {
                $($name => <$value_type>::parser().map(StyleProp::$variant),)*
                _ => fail("unknown prop")
            }
        }
    }
}

// longhand props
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
    "border-top-left-radius" => BorderTopLeftRadius(Dimension),
    "border-top-right-radius" => BorderTopRightRadius(Dimension),
    "border-bottom-right-radius" => BorderBottomRightRadius(Dimension),
    "border-bottom-left-radius" => BorderBottomLeftRadius(Dimension),

    // border
    "border-top-width" => BorderTopWidth(Dimension),
    "border-top-style" => BorderTopStyle(BorderStyle),
    "border-top-color" => BorderTopColor(Color),
    "border-right-width" => BorderRightWidth(Dimension),
    "border-right-style" => BorderRightStyle(BorderStyle),
    "border-right-color" => BorderRightColor(Color),
    "border-bottom-width" => BorderBottomWidth(Dimension),
    "border-bottom-style" => BorderBottomStyle(BorderStyle),
    "border-bottom-color" => BorderBottomColor(Color),
    "border-left-width" => BorderLeftWidth(Dimension),
    "border-left-style" => BorderLeftStyle(BorderStyle),
    "border-left-color" => BorderLeftColor(Color),

    // shadow
    "box-shadow" => BoxShadow(String),

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
    "outline-width" => OutlineWidth(Dimension),

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
}

/*
macro_rules! css_shorthands {
    ($(($name:literal, $parser:expr) => ($($variant:ident),*),)*) => {
        pub(super) fn shorthand_parser<'a>(prop: &str) -> super::parser::Parser<'a, Vec<StyleProp>> {
            #[allow(non_snake_case, unused_parens)]
            match prop {
                $($name => $parser.map(|($($variant),*)| vec![$(StyleProp::$variant($variant)),*]),)*
                _ => super::parser::fail("unknown prop")
            }
        }

        impl super::Style {
            pub(super) fn shorthand_value(&self, shorthand_name: &str) -> Option<String> {
                match shorthand_name {
                    //$($name => todo!(),)*
                    _ => return None
                }
            }
        }
    };
}

css_shorthands! {
    // TODO: multi, image, gradient
    (BackgroundColor) =     "background",

    // TODO: line-height should be delimited with /
    //"font" => (FontStyle, FontVariant, FontWeight, FontStretch, FontSize, LineHeight, FontFamily],

        (FlexGrow, FlexShrink, FlexBasis), =         "flex",
    ("padding", sides_of(dimension())) => (PaddingTop, PaddingRight, PaddingBottom, PaddingLeft),
    ("margin", sides_of(dimension())) => (MarginTop, MarginRight, MarginBottom, MarginLeft),

    // TODO
    (BorderTopWidth, BorderTopStyle, BorderTopColor, BorderRightWidth, BorderRightStyle, BorderRightColor, BorderBottomWidth, BorderBottomStyle, BorderBottomColor, BorderLeftWidth, BorderLeftStyle, BorderLeftColor =     / ("border",

    ("border-width", sides_of(dimension())) => (BorderTopWidth, BorderRightWidth, BorderBottomWidth, BorderLeftWidth),
    ("border-style", sides_of(css_enum())) => (BorderTopStyle, BorderRightStyle, BorderBottomStyle, BorderLeftStyle),
    ("border-color", sides_of(color())) => (BorderTopColor, BorderRightColor, BorderBottomColor, BorderLeftColor),

    // TODO(maybe): two dimensions
    ("border-radius", sides_of(dimension())) => (BorderTopLeftRadius, BorderTopRightRadius, BorderBottomRightRadius, BorderBottomLeftRadius),

    (OverflowX, OverflowY) =     "overflow",
    (OutlineWidth, OutlineStyle, OutlineColor) =     "outline",
    //"text-decoration" => ["text-decoration-color", "text-decoration-style", "text-decoration-line", "text-decoration-thickness"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_size() {
        use std::mem::size_of;

        assert_eq!(size_of::<Box<CssBoxShadow>>(), size_of::<usize>());
        assert_eq!(size_of::<Atom>(), size_of::<usize>());

        assert_eq!(size_of::<CssDimension>(), size_of::<(u32, f32)>());

        // TODO: gets broken when Atom<> or Box<> is added
        assert_eq!(size_of::<StyleProp>(), size_of::<(u8, Dimension)>());
    }
}
*/
