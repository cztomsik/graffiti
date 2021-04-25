// supported CSS props (longhand)

use std::fmt::{Display, Error, Formatter};

use super::values::*;
use crate::util::Atom;

// type shorthand
type V<T> = CssValue<T>;

macro_rules! css_properties {
    ($($variant:ident($value:ty) = $name:literal,)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum StyleProp {
            $($variant(V<$value>),)*
        }

        impl StyleProp {
            pub fn name(&self) -> &'static str {
                use StyleProp::*;

                match self {
                    $($variant(_) => $name,)*
                }
            }

            pub(super) fn value_as_string(&self) -> String {
                use StyleProp::*;

                let v: &dyn std::fmt::Display = match self {
                    $($variant(ref v) => v),*
                };

                format!("{}", v)
            }
        }
    }
}

css_properties! {
    // size
    Width(CssDimension) = "width",
    Height(CssDimension) = "height",
    MinWidth(CssDimension) = "min-width",
    MinHeight(CssDimension) = "min-height",
    MaxWidth(CssDimension) = "max-width",
    MaxHeight(CssDimension) = "max-height",

    // padding
    PaddingTop(CssDimension) = "padding-top",
    PaddingRight(CssDimension) = "padding-right",
    PaddingBottom(CssDimension) = "padding-bottom",
    PaddingLeft(CssDimension) = "padding-left",

    // margin
    MarginTop(CssDimension) = "margin-top",
    MarginRight(CssDimension) = "margin-right",
    MarginBottom(CssDimension) = "margin-bottom",
    MarginLeft(CssDimension) = "margin-left",

    // background
    BackgroundColor(CssColor) = "background-color",

    // border-radius
    BorderTopLeftRadius(CssDimension) = "border-top-left-radius",
    BorderTopRightRadius(CssDimension) = "border-top-right-radius",
    BorderBottomRightRadius(CssDimension) = "border-bottom-right-radius",
    BorderBottomLeftRadius(CssDimension) = "border-bottom-left-radius",

    // border
    BorderTopWidth(CssDimension) = "border-top-width",
    BorderTopStyle(CssBorderStyle) = "border-top-style",
    BorderTopColor(CssColor) = "border-top-color",
    BorderRightWidth(CssDimension) = "border-right-width",
    BorderRightStyle(CssBorderStyle) = "border-right-style",
    BorderRightColor(CssColor) = "border-right-color",
    BorderBottomWidth(CssDimension) = "border-bottom-width",
    BorderBottomStyle(CssBorderStyle) = "border-bottom-style",
    BorderBottomColor(CssColor) = "border-bottom-color",
    BorderLeftWidth(CssDimension) = "border-left-width",
    BorderLeftStyle(CssBorderStyle) = "border-left-style",
    BorderLeftColor(CssColor) = "border-left-color",

    // shadow
    BoxShadow(CssBoxShadow) = "box-shadow",

    // flex
    FlexBasis(CssDimension) = "flex-basis",
    FlexGrow(f32) = "flex-grow",
    FlexShrink(f32) = "flex-shrink",
    FlexDirection(CssFlexDirection) = "flex-direction",
    FlexWrap(CssFlexWrap) = "flex-wrap",
    AlignContent(CssAlign) = "align-content",
    AlignItems(CssAlign) = "align-items",
    AlignSelf(CssAlign) = "align-self",
    JustifyContent(CssAlign) = "justify-content",

    // text
    FontFamily(Atom<String>) = "font-family",
    FontSize(CssDimension) = "font-size",
    LineHeight(CssDimension) = "line-height",
    TextAlign(CssTextAlign) = "text-align",
    Color(CssColor) = "color",

    // outline
    OutlineColor(CssColor) = "outline-color",
    OutlineStyle(CssBorderStyle) = "outline-style",
    OutlineWidth(CssDimension) = "outline-width",

    // overflow
    OverflowX(CssOverflow) = "overflow-x",
    OverflowY(CssOverflow) = "overflow-y",

    // position
    Position(CssPosition) = "position",
    Top(CssDimension) = "top",
    Right(CssDimension) = "right",
    Bottom(CssDimension) = "bottom",
    Left(CssDimension) = "left",

    // other
    Display(CssDisplay) = "display",
    Opacity(f32) = "opacity",
    Visibility(CssVisibility) = "visibility",
}

impl Display for StyleProp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {};", self.name(), self.value_as_string())
    }
}
