// - individual props
// - parse Vec of props
// - parse rule
// - parse sheet
// - generic selectors
// - cascading
// - update() -> Vec<(el NodeId, Vec<StyleProp>)>
// - :hover
//
// unsupported:
// - dynamic props (Dimension in non-layout props)

// x dimension
// x align
// - color
// - box shadow
// - shorthands
// - normalize (bold -> 700)

#![allow(unused)]

use super::selectors::{parse_selector, Selector};

#[derive(Debug, Clone, PartialEq)]
pub struct CssStyleRule {
    selector: Selector,
    props: Vec<CssStyleProp>,
}

// supported props
//
// TODO: any prop can be also set to initial/inherit/unset
//       inherit is out-of-scope but initial might be useful
#[derive(Debug, Clone, PartialEq)]
pub enum CssStyleProp {
    AlignContent(CssAlign),
    AlignItems(CssAlign),
    AlignSelf(CssAlign),
    //Background,
    //BackgroundAttachment,
    //BackgroundClip,
    BackgroundColor(CssColor),
    //BackgroundImage,
    //BackgroundOrigin,
    //BackgroundPosition,
    //BackgroundRepeat,
    //BackgroundSize,
    //Border,
    //BorderBottom,
    BorderBottomColor(CssColor),
    BorderBottomLeftRadius(CssDimension),
    BorderBottomRightRadius(CssDimension),
    BorderBottomStyle(CssBorderStyle),
    BorderBottomWidth(CssDimension),
    //BorderCollapse,
    //BorderColor,
    //BorderImage,
    //BorderImageOutset,
    //BorderImageRepeat,
    //BorderImageSlice,
    //BorderImageSource,
    //BorderImageWidth,
    //BorderLeft,
    BorderLeftColor(CssColor),
    BorderLeftStyle(CssBorderStyle),
    BorderLeftWidth(CssDimension),
    //BorderRadius,
    //BorderRight,
    BorderRightColor(CssColor),
    BorderRightStyle(CssBorderStyle),
    BorderRightWidth(CssDimension),
    //BorderSpacing,
    //BorderStyle,
    //BorderTop,
    BorderTopColor(CssColor),
    BorderTopLeftRadius(CssDimension),
    BorderTopRightRadius(CssDimension),
    BorderTopStyle(CssBorderStyle),
    BorderTopWidth(CssDimension),
    //BorderWidth,
    Bottom(CssDimension),
    BoxShadow(CssBoxShadow),
    //CaptionSide,
    //Clear,
    //Clip,
    Color(CssColor),
    //Content,
    //CounterIncrement,
    //CounterReset,
    //Cursor,
    //Direction,
    Display(CssDisplay),
    //EmptyCells,
    FlexBasis(CssDimension),
    FlexDirection(CssFlexDirection),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexWrap(CssFlexWrap),
    //Float,
    Font,
    FontFamily,
    FontSize(CssDimension),
    //FontSizeAdjust,
    //FontStretch,
    //FontStyle,
    //FontSynthesis,
    //FontVariant,
    //FontWeight,
    Height(CssDimension),
    JustifyContent(CssAlign),
    Left(CssDimension),
    //LetterSpacing,
    LineHeight(CssDimension),
    //ListStyle,
    //ListStyleImage,
    //ListStylePosition,
    //ListStyleType,
    //Margin,
    MarginBottom(CssDimension),
    MarginLeft(CssDimension),
    MarginRight(CssDimension),
    MarginTop(CssDimension),
    MaxHeight(CssDimension),
    MaxWidth(CssDimension),
    MinHeight(CssDimension),
    MinWidth(CssDimension),
    Opacity(f32),
    //Orphans,
    //Outline,
    OutlineColor(CssColor),
    OutlineStyle(CssBorderStyle),
    OutlineWidth(CssDimension),
    //Overflow(CssOverflow),
    OverflowX(CssOverflow),
    OverflowY(CssOverflow),
    //Padding,
    PaddingBottom(CssDimension),
    PaddingLeft(CssDimension),
    PaddingRight(CssDimension),
    PaddingTop(CssDimension),
    //PageBreakAfter,
    //PageBreakBefore,
    //PageBreakInside,
    Position(CssPosition),
    //Quotes,
    Right(CssDimension),
    //TableLayout,
    TextAlign(CssTextAlign),
    //TextDecoration,
    //TextDecorationColor,
    //TextDecorationLine,
    //TextDecorationStyle,
    //TextIndent,
    //TextShadow
    //TextTransform,
    Top(CssDimension),
    //Transform(CssTransform),
    //TransformOrigin,
    //Transition,
    //TransitionDelay,
    //TransitionDuration,
    //TransitionProperty,
    //TransitionTimingFunction,
    //UnicodeBidi,
    //VerticalAlign,
    Visibility(CssVisibility),
    //WhiteSpace,
    //Widows,
    Width(CssDimension),
    //WordSpacing,
    //ZIndex
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssAlign {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssDimension {
    Auto,
    Px(f32),
    Percent(f32),
    //Vw(f32)
    //Vh(f32)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssBorderStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssBoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: CssColor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssDisplay {
    None,
    Block,
    Inline,
    Flex,
    // Grid,
    // Table, TableRow, TableCell, ...
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssFlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssFlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssOverflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssPosition {
    Static,
    Relative,
    Absolute,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssTextAlign {
    Left,
    Right,
    Center,
    Justify,
}

// enum?
//pub struct CssTransform { ? }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssVisibility {
    Visible,
    Hidden,
    Collapse,
}

pub fn parse_rules(s: &str) -> Vec<CssStyleRule> {
    parse::rules().parse(s.as_bytes()).unwrap_or(Vec::new())
}

pub fn parse_props(s: &str) -> Vec<CssStyleProp> {
    parse::style_props().parse(s.as_bytes()).unwrap_or(Vec::new())
}

mod parse {
    use super::*;
    use pom::char_class::{alpha, hex_digit};
    use pom::parser::*;

    pub fn rules<'a>() -> Parser<'a, u8, Vec<CssStyleRule>> {
        rule().repeat(0..)
    }

    fn rule<'a>() -> Parser<'a, u8, CssStyleRule> {
        let selector = none_of(b"{}").repeat(1..).collect().convert(std::str::from_utf8).convert(parse_selector);
        let rule = selector - sym(b'{') - space() + style_props() - space() - sym(b'}');

        rule.map(|(selector, props)| CssStyleRule { selector, props })
    }

    pub fn style_props<'a>() -> Parser<'a, u8, Vec<CssStyleProp>> {
        list(style_prop(), one_of(b"; ").repeat(1..))
    }

    fn style_prop<'a>() -> Parser<'a, u8, CssStyleProp> {
        prop_name() - sym(b':') - space()
            >> |p| match p {
                b"align-content" => align().map(CssStyleProp::AlignContent),
                b"align-items" => align().map(CssStyleProp::AlignItems),
                b"align-self" => align().map(CssStyleProp::AlignSelf),
                b"background-color" => color().map(CssStyleProp::BackgroundColor),
                b"border-bottom-color" => color().map(CssStyleProp::BorderBottomColor),
                b"border-bottom-left-radius" => dimension().map(CssStyleProp::BorderBottomLeftRadius),
                b"border-bottom-right-radius" => dimension().map(CssStyleProp::BorderBottomRightRadius),
                b"border-bottom-style" => border_style().map(CssStyleProp::BorderBottomStyle),
                b"border-bottom-width" => dimension().map(CssStyleProp::BorderBottomWidth),
                b"border-left-color" => color().map(CssStyleProp::BorderLeftColor),
                b"border-left-style" => border_style().map(CssStyleProp::BorderLeftStyle),
                b"border-left-width" => dimension().map(CssStyleProp::BorderLeftWidth),
                b"border-right-color" => color().map(CssStyleProp::BorderRightColor),
                b"border-right-style" => border_style().map(CssStyleProp::BorderRightStyle),
                b"border-right-width" => dimension().map(CssStyleProp::BorderRightWidth),
                b"border-top-color" => color().map(CssStyleProp::BorderTopColor),
                b"border-top-left-radius" => dimension().map(CssStyleProp::BorderTopLeftRadius),
                b"border-top-right-radius" => dimension().map(CssStyleProp::BorderTopRightRadius),
                b"border-top-style" => border_style().map(CssStyleProp::BorderTopStyle),
                b"border-top-width" => dimension().map(CssStyleProp::BorderTopWidth),
                b"bottom" => dimension().map(CssStyleProp::Bottom),
                //b"box-shadow" => box_shadow().map(CssStyleProp::BoxShadow),
                b"color" => color().map(CssStyleProp::Color),
                b"display" => display().map(CssStyleProp::Display),
                b"flex-basis" => dimension().map(CssStyleProp::FlexBasis),
                b"flex-direction" => flex_direction().map(CssStyleProp::FlexDirection),
                b"flex-grow" => float().map(CssStyleProp::FlexGrow),
                b"flex-shrink" => float().map(CssStyleProp::FlexShrink),
                b"flex-wrap" => flex_wrap().map(CssStyleProp::FlexWrap),
                //b"font" => Font,
                //b"font-family" => FontFamily,
                b"font-size" => dimension().map(CssStyleProp::FontSize),
                b"height" => dimension().map(CssStyleProp::Height),
                b"justify-content" => align().map(CssStyleProp::JustifyContent),
                b"left" => dimension().map(CssStyleProp::Left),
                b"line-height" => dimension().map(CssStyleProp::LineHeight),
                b"margin-bottom" => dimension().map(CssStyleProp::MarginBottom),
                b"margin-left" => dimension().map(CssStyleProp::MarginLeft),
                b"margin-right" => dimension().map(CssStyleProp::MarginRight),
                b"margin-top" => dimension().map(CssStyleProp::MarginTop),
                b"max-height" => dimension().map(CssStyleProp::MaxHeight),
                b"max-width" => dimension().map(CssStyleProp::MaxWidth),
                b"min-height" => dimension().map(CssStyleProp::MinHeight),
                b"min-width" => dimension().map(CssStyleProp::MinWidth),
                b"opacity" => float().map(CssStyleProp::Opacity),
                b"outline-color" => color().map(CssStyleProp::OutlineColor),
                b"outline-style" => border_style().map(CssStyleProp::OutlineStyle),
                b"outline-width" => dimension().map(CssStyleProp::OutlineWidth),
                b"overflow-x" => overflow().map(CssStyleProp::OverflowX),
                b"overflow-y" => overflow().map(CssStyleProp::OverflowY),
                b"padding-bottom" => dimension().map(CssStyleProp::PaddingBottom),
                b"padding-left" => dimension().map(CssStyleProp::PaddingLeft),
                b"padding-right" => dimension().map(CssStyleProp::PaddingRight),
                b"padding-top" => dimension().map(CssStyleProp::PaddingTop),
                b"position" => position().map(CssStyleProp::Position),
                b"right" => dimension().map(CssStyleProp::Right),
                b"text-align" => text_align().map(CssStyleProp::TextAlign),
                b"top" => dimension().map(CssStyleProp::Top),
                b"visibility" => visibility().map(CssStyleProp::Visibility),
                b"width" => dimension().map(CssStyleProp::Width),

                _ => fail("unknown style prop"),
            }
    }

    fn align<'a>() -> Parser<'a, u8, CssAlign> {
        keyword().convert(|kw| match kw {
            b"auto" => Ok(CssAlign::Auto),
            b"start" => Ok(CssAlign::Start),
            b"flex-start" => Ok(CssAlign::Start),
            b"center" => Ok(CssAlign::Center),
            b"end" => Ok(CssAlign::End),
            b"flex-end" => Ok(CssAlign::End),
            b"stretch" => Ok(CssAlign::Stretch),
            b"baseline" => Ok(CssAlign::Baseline),
            b"space-between" => Ok(CssAlign::SpaceBetween),
            b"space-around" => Ok(CssAlign::SpaceAround),
            b"space-evenly" => Ok(CssAlign::SpaceEvenly),

            _ => Err("invalid align"),
        })
    }

    fn color<'a>() -> Parser<'a, u8, CssColor> {
        // TODO: rgb/rgba()

        sym(b'#')
            * is_a(hex_digit).repeat(3..9).collect().convert(|hex| {
                (match hex.len() {
                    8 | 6 => std::str::from_utf8(hex)
                        .ok()
                        .and_then(|s| u32::from_str_radix(s, 16).ok())
                        .map(|num| if hex.len() == 6 { num << 8 | 0xFF } else { num })
                        .map(|num| CssColor {
                            r: ((num >> 24) & 0xFF) as u8,
                            g: ((num >> 16) & 0xFF) as u8,
                            b: ((num >> 8) & 0xFF) as u8,
                            a: (num & 0xFF) as u8,
                        }),

                    4 => unimplemented!(),
                    3 => unimplemented!(),

                    _ => None,
                })
                .ok_or("invalid color")
            })
    }

    fn dimension<'a>() -> Parser<'a, u8, CssDimension> {
        let px = (float() - seq(b"px")).map(CssDimension::Px);
        let percent = (float() - sym(b'%')).map(CssDimension::Percent);
        let auto = seq(b"auto").map(|_| CssDimension::Auto);
        let zero = sym(b'0').map(|_| CssDimension::Px(0.));

        px | percent | auto | zero
    }

    fn border_style<'a>() -> Parser<'a, u8, CssBorderStyle> {
        keyword().convert(|kw| match kw {
            b"none" => Ok(CssBorderStyle::None),
            b"hidden" => Ok(CssBorderStyle::Hidden),
            b"dotted" => Ok(CssBorderStyle::Dotted),
            b"dashed" => Ok(CssBorderStyle::Dashed),
            b"solid" => Ok(CssBorderStyle::Solid),
            b"double" => Ok(CssBorderStyle::Double),
            b"groove" => Ok(CssBorderStyle::Groove),
            b"ridge" => Ok(CssBorderStyle::Ridge),
            b"inset" => Ok(CssBorderStyle::Inset),
            b"outset" => Ok(CssBorderStyle::Outset),

            _ => Err("invalid border style"),
        })
    }

    // TODO: box_shadow

    fn display<'a>() -> Parser<'a, u8, CssDisplay> {
        keyword().convert(|kw| match kw {
            b"none" => Ok(CssDisplay::None),
            b"block" => Ok(CssDisplay::Block),
            b"inline" => Ok(CssDisplay::Inline),
            b"flex" => Ok(CssDisplay::Flex),

            _ => Err("invalid display"),
        })
    }

    fn flex_direction<'a>() -> Parser<'a, u8, CssFlexDirection> {
        keyword().convert(|kw| match kw {
            b"row" => Ok(CssFlexDirection::Row),
            b"column" => Ok(CssFlexDirection::Column),
            b"row-reverse" => Ok(CssFlexDirection::RowReverse),
            b"column-reverse" => Ok(CssFlexDirection::ColumnReverse),

            _ => Err("invalid flex direction"),
        })
    }

    fn flex_wrap<'a>() -> Parser<'a, u8, CssFlexWrap> {
        keyword().convert(|kw| match kw {
            b"nowrap" => Ok(CssFlexWrap::NoWrap),
            b"wrap" => Ok(CssFlexWrap::Wrap),
            b"wrap-reverse" => Ok(CssFlexWrap::WrapReverse),

            _ => Err("invalid flex wrap"),
        })
    }

    fn overflow<'a>() -> Parser<'a, u8, CssOverflow> {
        keyword().convert(|kw| match kw {
            b"visible" => Ok(CssOverflow::Visible),
            b"hidden" => Ok(CssOverflow::Hidden),
            b"scroll" => Ok(CssOverflow::Scroll),
            b"auto" => Ok(CssOverflow::Auto),

            _ => Err("invalid overflow"),
        })
    }

    fn position<'a>() -> Parser<'a, u8, CssPosition> {
        keyword().convert(|kw| match kw {
            b"static" => Ok(CssPosition::Static),
            b"relative" => Ok(CssPosition::Relative),
            b"absolute" => Ok(CssPosition::Absolute),
            b"sticky" => Ok(CssPosition::Sticky),

            _ => Err("invalid position"),
        })
    }

    fn text_align<'a>() -> Parser<'a, u8, CssTextAlign> {
        keyword().convert(|kw| match kw {
            b"left" => Ok(CssTextAlign::Left),
            b"center" => Ok(CssTextAlign::Center),
            b"right" => Ok(CssTextAlign::Right),
            b"justify" => Ok(CssTextAlign::Justify),

            _ => Err("invalid text align"),
        })
    }

    // TODO: transform

    fn visibility<'a>() -> Parser<'a, u8, CssVisibility> {
        keyword().convert(|kw| match kw {
            b"visible" => Ok(CssVisibility::Visible),
            b"hidden" => Ok(CssVisibility::Hidden),
            b"collapse" => Ok(CssVisibility::Collapse),

            _ => Err("invalid visibility"),
        })
    }

    fn prop_name<'a>() -> Parser<'a, u8, &'a [u8]> {
        is_a(alpha_dash).repeat(1..).collect()
    }

    fn keyword<'a>() -> Parser<'a, u8, &'a [u8]> {
        is_a(alpha_dash).repeat(1..).collect()
    }

    fn float<'a>() -> Parser<'a, u8, f32> {
        num().convert(std::str::from_utf8).convert(str::parse)
    }

    fn num<'a>() -> Parser<'a, u8, &'a [u8]> {
        one_of(b".0123456789").repeat(1..).collect()
    }

    fn space<'a>() -> Parser<'a, u8, ()> {
        sym(b' ').repeat(0..).discard()
    }

    fn fail<'a, T: 'static>(msg: &'static str) -> Parser<'a, u8, T> {
        empty().convert(move |_| Err(msg))
    }

    fn alpha_dash(b: u8) -> bool {
        alpha(b) || b == b'-'
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_rule() {
            assert_eq!(
                rule().parse(b"* { left: 0; opacity: 1 }"),
                Ok(CssStyleRule {
                    selector: Selector::Universal,
                    props: vec![CssStyleProp::Left(CssDimension::Px(0.)), CssStyleProp::Opacity(1.)]
                })
            );
        }

        #[test]
        fn parse_props() {
            assert_eq!(
                style_props().parse(b"left: 0; opacity: 1"),
                Ok(vec![CssStyleProp::Left(CssDimension::Px(0.)), CssStyleProp::Opacity(1.)])
            );
        }

        #[test]
        fn parse_prop() {
            assert_eq!(style_prop().parse(b"padding-left: 10px"), Ok(CssStyleProp::PaddingLeft(CssDimension::Px(10.))));
            assert_eq!(style_prop().parse(b"margin-top: 5%"), Ok(CssStyleProp::MarginTop(CssDimension::Percent(5.))));
            assert_eq!(style_prop().parse(b"opacity: 1"), Ok(CssStyleProp::Opacity(1.)));
            assert_eq!(style_prop().parse(b"color: #000000"), Ok(CssStyleProp::Color(CssColor { r: 0, g: 0, b: 0, a: 255 })));
        }

        #[test]
        fn parse_align() {
            assert_eq!(align().parse(b"auto"), Ok(CssAlign::Auto));
            assert_eq!(align().parse(b"start"), Ok(CssAlign::Start));
            assert_eq!(align().parse(b"flex-start"), Ok(CssAlign::Start));
            assert_eq!(align().parse(b"center"), Ok(CssAlign::Center));
            assert_eq!(align().parse(b"end"), Ok(CssAlign::End));
            assert_eq!(align().parse(b"flex-end"), Ok(CssAlign::End));
            assert_eq!(align().parse(b"stretch"), Ok(CssAlign::Stretch));
            assert_eq!(align().parse(b"baseline"), Ok(CssAlign::Baseline));
            assert_eq!(align().parse(b"space-between"), Ok(CssAlign::SpaceBetween));
            assert_eq!(align().parse(b"space-around"), Ok(CssAlign::SpaceAround));
            assert_eq!(align().parse(b"space-evenly"), Ok(CssAlign::SpaceEvenly));
        }

        #[test]
        fn parse_dimension() {
            assert_eq!(dimension().parse(b"auto"), Ok(CssDimension::Auto));
            assert_eq!(dimension().parse(b"10px"), Ok(CssDimension::Px(10.)));
            assert_eq!(dimension().parse(b"100%"), Ok(CssDimension::Percent(100.)));
            assert_eq!(dimension().parse(b"0"), Ok(CssDimension::Px(0.)));
        }

        #[test]
        fn parse_color() {
            assert_eq!(color().parse(b"#000000"), Ok(CssColor { r: 0, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#ff0000"), Ok(CssColor { r: 255, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#00ff00"), Ok(CssColor { r: 0, g: 255, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#0000ff"), Ok(CssColor { r: 0, g: 0, b: 255, a: 255 }));

            assert_eq!(color().parse(b"#00000080"), Ok(CssColor { r: 0, g: 0, b: 0, a: 128 }));

            //assert_eq!(color().parse(b"#000"), Ok(CssColor { r: 0, g: 0, b: 0, a: 255 }));
            //assert_eq!(color().parse(b"#f00"), Ok(CssColor { r: 255, g: 0, b: 0, a: 255 }));

            //assert_eq!(color().parse(b"#0000"), Ok(CssColor { r: 0, g: 0, b: 0, a: 0 }));
            //assert_eq!(color().parse(b"#f00f"), Ok(CssColor { r: 255, g: 0, b: 0, a: 255 }));

            //assert_eq!(color().parse(b"rgb(0, 0, 0)"), Ok(CssColor { r: 0, g: 0, b: 0, a: 255 }));
            //assert_eq!(color().parse(b"rgba(0, 0, 0, 0)"), Ok(CssColor { r: 0, g: 0, b: 0, a: 0 }));
        }

        #[test]
        fn parse_border_style() {
            assert_eq!(border_style().parse(b"none"), Ok(CssBorderStyle::None));
            assert_eq!(border_style().parse(b"hidden"), Ok(CssBorderStyle::Hidden));
            assert_eq!(border_style().parse(b"dotted"), Ok(CssBorderStyle::Dotted));
            assert_eq!(border_style().parse(b"dashed"), Ok(CssBorderStyle::Dashed));
            assert_eq!(border_style().parse(b"solid"), Ok(CssBorderStyle::Solid));
            assert_eq!(border_style().parse(b"double"), Ok(CssBorderStyle::Double));
            assert_eq!(border_style().parse(b"groove"), Ok(CssBorderStyle::Groove));
            assert_eq!(border_style().parse(b"ridge"), Ok(CssBorderStyle::Ridge));
            assert_eq!(border_style().parse(b"inset"), Ok(CssBorderStyle::Inset));
            assert_eq!(border_style().parse(b"outset"), Ok(CssBorderStyle::Outset));
        }

        // TODO: parse_box_shadow

        #[test]
        fn parse_display() {
            assert_eq!(display().parse(b"none"), Ok(CssDisplay::None));
            assert_eq!(display().parse(b"block"), Ok(CssDisplay::Block));
            assert_eq!(display().parse(b"inline"), Ok(CssDisplay::Inline));
            assert_eq!(display().parse(b"flex"), Ok(CssDisplay::Flex));
        }

        #[test]
        fn parse_flex_direction() {
            assert_eq!(flex_direction().parse(b"row"), Ok(CssFlexDirection::Row));
            assert_eq!(flex_direction().parse(b"column"), Ok(CssFlexDirection::Column));
            assert_eq!(flex_direction().parse(b"row-reverse"), Ok(CssFlexDirection::RowReverse));
            assert_eq!(flex_direction().parse(b"column-reverse"), Ok(CssFlexDirection::ColumnReverse));
        }

        #[test]
        fn parse_flex_wrap() {
            assert_eq!(flex_wrap().parse(b"nowrap"), Ok(CssFlexWrap::NoWrap));
            assert_eq!(flex_wrap().parse(b"wrap"), Ok(CssFlexWrap::Wrap));
            assert_eq!(flex_wrap().parse(b"wrap-reverse"), Ok(CssFlexWrap::WrapReverse));
        }

        #[test]
        fn parse_overflow() {
            assert_eq!(overflow().parse(b"visible"), Ok(CssOverflow::Visible));
            assert_eq!(overflow().parse(b"hidden"), Ok(CssOverflow::Hidden));
            assert_eq!(overflow().parse(b"scroll"), Ok(CssOverflow::Scroll));
            assert_eq!(overflow().parse(b"auto"), Ok(CssOverflow::Auto));
        }

        #[test]
        fn parse_position() {
            assert_eq!(position().parse(b"static"), Ok(CssPosition::Static));
            assert_eq!(position().parse(b"relative"), Ok(CssPosition::Relative));
            assert_eq!(position().parse(b"absolute"), Ok(CssPosition::Absolute));
            assert_eq!(position().parse(b"sticky"), Ok(CssPosition::Sticky));
        }

        #[test]
        fn parse_text_align() {
            assert_eq!(text_align().parse(b"left"), Ok(CssTextAlign::Left));
            assert_eq!(text_align().parse(b"center"), Ok(CssTextAlign::Center));
            assert_eq!(text_align().parse(b"right"), Ok(CssTextAlign::Right));
            assert_eq!(text_align().parse(b"justify"), Ok(CssTextAlign::Justify));
        }

        // TODO: parse_transform

        #[test]
        fn parse_visibility() {
            assert_eq!(visibility().parse(b"visible"), Ok(CssVisibility::Visible));
            assert_eq!(visibility().parse(b"hidden"), Ok(CssVisibility::Hidden));
            assert_eq!(visibility().parse(b"collapse"), Ok(CssVisibility::Collapse));
        }
    }
}
