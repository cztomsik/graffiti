// supported CSS props

use super::{
    CssAlign, CssBorderStyle, CssBoxShadow, CssColor, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap,
    CssOverflow, CssPosition, CssTextAlign, CssVisibility,
};
use crate::util::Atom;

macro_rules! css_properties {
    ($(($name:literal, $parser:ident) => $variant:ident($value_type:ty),)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum StyleProp {
            $($variant($value_type),)*
        }

        impl StyleProp {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant(_) => $name,)*
                }
            }

            pub(super) fn value_as_string(&self) -> String {
                let v: &dyn std::fmt::Display = match self {
                    $(Self::$variant(ref v) => v),*
                };

                format!("{}", v)
            }
        }

        pub(super) fn prop_parser<'a>(prop: &str) -> super::parser::Parser<'a, StyleProp> {
            match prop {
                $($name => super::parser::$parser().map(StyleProp::$variant),)*
                _ => super::parser::fail("unknown prop")
            }
        }
    }
}

// longhand props
css_properties! {
    // size
    ("width", dimension) => Width(CssDimension),
    ("height", dimension) => Height(CssDimension),
    ("min-width", dimension) => MinWidth(CssDimension),
    ("min-height", dimension) => MinHeight(CssDimension),
    ("max-width", dimension) => MaxWidth(CssDimension),
    ("max-height", dimension) => MaxHeight(CssDimension),

    // padding
    ("padding-top", dimension) => PaddingTop(CssDimension),
    ("padding-right", dimension) => PaddingRight(CssDimension),
    ("padding-bottom", dimension) => PaddingBottom(CssDimension),
    ("padding-left", dimension) => PaddingLeft(CssDimension),

    // margin
    ("margin-top", dimension) => MarginTop(CssDimension),
    ("margin-right", dimension) => MarginRight(CssDimension),
    ("margin-bottom", dimension) => MarginBottom(CssDimension),
    ("margin-left", dimension) => MarginLeft(CssDimension),

    // background
    ("background-color", color) => BackgroundColor(CssColor),

    // border-radius
    ("border-top-left-radius", dimension) => BorderTopLeftRadius(CssDimension),
    ("border-top-right-radius", dimension) => BorderTopRightRadius(CssDimension),
    ("border-bottom-right-radius", dimension) => BorderBottomRightRadius(CssDimension),
    ("border-bottom-left-radius", dimension) => BorderBottomLeftRadius(CssDimension),

    // border
    ("border-top-width", dimension) => BorderTopWidth(CssDimension),
    ("border-top-style", try_from) => BorderTopStyle(CssBorderStyle),
    ("border-top-color", color) => BorderTopColor(CssColor),
    ("border-right-width", dimension) => BorderRightWidth(CssDimension),
    ("border-right-style", try_from) => BorderRightStyle(CssBorderStyle),
    ("border-right-color", color) => BorderRightColor(CssColor),
    ("border-bottom-width", dimension) => BorderBottomWidth(CssDimension),
    ("border-bottom-style", try_from) => BorderBottomStyle(CssBorderStyle),
    ("border-bottom-color", color) => BorderBottomColor(CssColor),
    ("border-left-width", dimension) => BorderLeftWidth(CssDimension),
    ("border-left-style", try_from) => BorderLeftStyle(CssBorderStyle),
    ("border-left-color", color) => BorderLeftColor(CssColor),

    // shadow
    ("box-shadow", box_shadow) => BoxShadow(CssBoxShadow),

    // flex
    ("flex-basis", dimension) => FlexBasis(CssDimension),
    ("flex-grow", float) => FlexGrow(f32),
    ("flex-shrink", float) => FlexShrink(f32),
    ("flex-direction", try_from) => FlexDirection(CssFlexDirection),
    ("flex-wrap", try_from) => FlexWrap(CssFlexWrap),
    ("align-content", try_from) => AlignContent(CssAlign),
    ("align-items", try_from) => AlignItems(CssAlign),
    ("align-self", try_from) => AlignSelf(CssAlign),
    ("justify-content", try_from) => JustifyContent(CssAlign),

    // text
    ("font-family", font_family) => FontFamily(Atom<String>),
    ("font-size", dimension) => FontSize(CssDimension),
    ("line-height", dimension) => LineHeight(CssDimension),
    ("text-align", try_from) => TextAlign(CssTextAlign),
    ("color", color) => Color(CssColor),

    // outline
    ("outline-color", color) => OutlineColor(CssColor),
    ("outline-style", try_from) => OutlineStyle(CssBorderStyle),
    ("outline-width", dimension) => OutlineWidth(CssDimension),

    // overflow
    ("overflow-x", try_from) => OverflowX(CssOverflow),
    ("overflow-y", try_from) => OverflowY(CssOverflow),

    // position
    ("position", try_from) => Position(CssPosition),
    ("top", dimension) => Top(CssDimension),
    ("right", dimension) => Right(CssDimension),
    ("bottom", dimension) => Bottom(CssDimension),
    ("left", dimension) => Left(CssDimension),

    // other
    ("display", try_from) => Display(CssDisplay),
    ("opacity", float) => Opacity(f32),
    ("visibility", try_from) => Visibility(CssVisibility),
}
