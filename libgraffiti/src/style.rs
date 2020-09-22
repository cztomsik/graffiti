// TODO:
// - color
// - shorthands
// - normalize (bold -> 700)

// TODO
#![allow(unused)]

use core::convert::TryFrom;
use std::error::Error;
use std::mem::discriminant;

#[derive(Debug, Clone)]
pub struct Style {
    props: Vec<StyleProp>,
}

impl Style {
    pub const EMPTY: Self = Self::new();

    pub const fn new() -> Self {
        Self { props: Vec::new() }
    }

    pub fn props(&self) -> impl Iterator<Item = &StyleProp> + '_ {
        self.props.iter()
    }

    pub fn set_prop_value<'a>(&'a mut self, prop: &'a str, value: &'a str) -> Result<(), &'a str> {
        // TODO: shorthands, maybe it's enough to call some helper, pass it slice of parsers
        //       and do what's below for each match?
        //
        // shorthand should set all long-hands and what's not included
        // should be set to initial
        let new_prop = parse::parse_style_prop(prop, value)?;

        let d = discriminant(&new_prop);

        if let Some(existing) = self.props.iter_mut().find(|p| d == discriminant(p)) {
            *existing = new_prop;
        } else {
            self.props.push(new_prop);
        }

        Ok(())
    }
}

// supported props
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleProp {
    AlignContent(Align),
    AlignItems(Align),
    AlignSelf(Align),
    //Background,
    //BackgroundAttachment,
    //BackgroundClip,
    BackgroundColor(Color),
    //BackgroundImage,
    //BackgroundOrigin,
    //BackgroundPosition,
    //BackgroundRepeat,
    //BackgroundSize,
    //Border,
    //BorderBottom,
    BorderBottomColor(Color),
    BorderBottomLeftRadius(Dimension),
    BorderBottomRightRadius(Dimension),
    BorderBottomStyle(BorderStyle),
    BorderBottomWidth(Dimension),
    //BorderCollapse,
    //BorderColor,
    //BorderImage,
    //BorderImageOutset,
    //BorderImageRepeat,
    //BorderImageSlice,
    //BorderImageSource,
    //BorderImageWidth,
    //BorderLeft,
    BorderLeftColor(Color),
    BorderLeftStyle(BorderStyle),
    BorderLeftWidth(Dimension),
    //BorderRadius,
    //BorderRight,
    BorderRightColor(Color),
    BorderRightStyle(BorderStyle),
    BorderRightWidth(Dimension),
    //BorderSpacing,
    //BorderStyle,
    //BorderTop,
    BorderTopColor(Color),
    BorderTopLeftRadius(Dimension),
    BorderTopRightRadius(Dimension),
    BorderTopStyle(BorderStyle),
    BorderTopWidth(Dimension),
    //BorderWidth,
    Bottom(Dimension),
    BoxShadow(BoxShadow),
    //CaptionSide,
    //Clear,
    //Clip,
    Color(Color),
    //Content,
    //CounterIncrement,
    //CounterReset,
    //Cursor,
    //Direction,
    Display(Display),
    //EmptyCells,
    FlexBasis(Dimension),
    FlexDirection(FlexDirection),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexWrap(FlexWrap),
    //Float,
    Font,
    FontFamily,
    FontSize(Dimension),
    //FontSizeAdjust,
    //FontStretch,
    //FontStyle,
    //FontSynthesis,
    //FontVariant,
    //FontWeight,
    Height(Dimension),
    JustifyContent(Align),
    Left(Dimension),
    //LetterSpacing,
    LineHeight(Dimension),
    //ListStyle,
    //ListStyleImage,
    //ListStylePosition,
    //ListStyleType,
    //Margin,
    MarginBottom(Dimension),
    MarginLeft(Dimension),
    MarginRight(Dimension),
    MarginTop(Dimension),
    MaxHeight(Dimension),
    MaxWidth(Dimension),
    MinHeight(Dimension),
    MinWidth(Dimension),
    Opacity(f32),
    //Orphans,
    //Outline,
    OutlineColor(Color),
    OutlineStyle(BorderStyle),
    OutlineWidth(Dimension),
    //Overflow(Overflow),
    OverflowX(Overflow),
    OverflowY(Overflow),
    //Padding,
    PaddingBottom(Dimension),
    PaddingLeft(Dimension),
    PaddingRight(Dimension),
    PaddingTop(Dimension),
    //PageBreakAfter,
    //PageBreakBefore,
    //PageBreakInside,
    Position(Position),
    //Quotes,
    Right(Dimension),
    //TableLayout,
    TextAlign(TextAlign),
    //TextDecoration,
    //TextDecorationColor,
    //TextDecorationLine,
    //TextDecorationStyle,
    //TextIndent,
    //TextShadow
    //TextTransform,
    Top(Dimension),
    //Transform(Transform),
    //TransformOrigin,
    //Transition,
    //TransitionDelay,
    //TransitionDuration,
    //TransitionProperty,
    //TransitionTimingFunction,
    //UnicodeBidi,
    //VerticalAlign,
    Visibility(Visibility),
    //WhiteSpace,
    //Widows,
    Width(Dimension),
    //WordSpacing,
    //ZIndex
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
    //Vw(f32)
    //Vh(f32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
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
pub struct BoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    None,
    Block,
    Inline,
    Flex,
    // Grid,
    // Table, TableRow, TableCell, ...
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

// TODO, enum?
//pub struct Transform { ? }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_props() {
        let mut s = Style::new();

        s.set_prop_value("display", "none");
        s.set_prop_value("display", "block");

        assert!(Iterator::eq(s.props(), &vec![StyleProp::Display(Display::Block)]));
    }
}

mod parse {
    use super::*;
    use pom::char_class::{alpha, hex_digit};
    use pom::parser::*;

    pub fn parse_style_prop<'a>(prop: &'a str, value: &'a str) -> Result<StyleProp, &'a str> {
        let parser = match prop {
            "align-content" => align().map(StyleProp::AlignContent),
            "align-items" => align().map(StyleProp::AlignItems),
            "align-self" => align().map(StyleProp::AlignSelf),
            "background-color" => color().map(StyleProp::BackgroundColor),
            "border-bottom-color" => color().map(StyleProp::BorderBottomColor),
            "border-bottom-left-radius" => dimension().map(StyleProp::BorderBottomLeftRadius),
            "border-bottom-right-radius" => dimension().map(StyleProp::BorderBottomRightRadius),
            "border-bottom-style" => border_style().map(StyleProp::BorderBottomStyle),
            "border-bottom-width" => dimension().map(StyleProp::BorderBottomWidth),
            "border-left-color" => color().map(StyleProp::BorderLeftColor),
            "border-left-style" => border_style().map(StyleProp::BorderLeftStyle),
            "border-left-width" => dimension().map(StyleProp::BorderLeftWidth),
            "border-right-color" => color().map(StyleProp::BorderRightColor),
            "border-right-style" => border_style().map(StyleProp::BorderRightStyle),
            "border-right-width" => dimension().map(StyleProp::BorderRightWidth),
            "border-top-color" => color().map(StyleProp::BorderTopColor),
            "border-top-left-radius" => dimension().map(StyleProp::BorderTopLeftRadius),
            "border-top-right-radius" => dimension().map(StyleProp::BorderTopRightRadius),
            "border-top-style" => border_style().map(StyleProp::BorderTopStyle),
            "border-top-width" => dimension().map(StyleProp::BorderTopWidth),
            "bottom" => dimension().map(StyleProp::Bottom),
            //"box-shadow" => box_shadow().map(StyleProp::BoxShadow),
            "color" => color().map(StyleProp::Color),
            "display" => display().map(StyleProp::Display),
            "flex-basis" => dimension().map(StyleProp::FlexBasis),
            "flex-direction" => flex_direction().map(StyleProp::FlexDirection),
            "flex-grow" => float().map(StyleProp::FlexGrow),
            "flex-shrink" => float().map(StyleProp::FlexShrink),
            "flex-wrap" => flex_wrap().map(StyleProp::FlexWrap),
            //"font" => Font,
            //"font-family" => FontFamily,
            "font-size" => dimension().map(StyleProp::FontSize),
            "height" => dimension().map(StyleProp::Height),
            "justify-content" => align().map(StyleProp::JustifyContent),
            "left" => dimension().map(StyleProp::Left),
            "line-height" => dimension().map(StyleProp::LineHeight),
            "margin-bottom" => dimension().map(StyleProp::MarginBottom),
            "margin-left" => dimension().map(StyleProp::MarginLeft),
            "margin-right" => dimension().map(StyleProp::MarginRight),
            "margin-top" => dimension().map(StyleProp::MarginTop),
            "max-height" => dimension().map(StyleProp::MaxHeight),
            "max-width" => dimension().map(StyleProp::MaxWidth),
            "min-height" => dimension().map(StyleProp::MinHeight),
            "min-width" => dimension().map(StyleProp::MinWidth),
            "opacity" => float().map(StyleProp::Opacity),
            "outline-color" => color().map(StyleProp::OutlineColor),
            "outline-style" => border_style().map(StyleProp::OutlineStyle),
            "outline-width" => dimension().map(StyleProp::OutlineWidth),
            "overflow-x" => overflow().map(StyleProp::OverflowX),
            "overflow-y" => overflow().map(StyleProp::OverflowY),
            "padding-bottom" => dimension().map(StyleProp::PaddingBottom),
            "padding-left" => dimension().map(StyleProp::PaddingLeft),
            "padding-right" => dimension().map(StyleProp::PaddingRight),
            "padding-top" => dimension().map(StyleProp::PaddingTop),
            "position" => position().map(StyleProp::Position),
            "right" => dimension().map(StyleProp::Right),
            "text-align" => text_align().map(StyleProp::TextAlign),
            "top" => dimension().map(StyleProp::Top),
            "visibility" => visibility().map(StyleProp::Visibility),
            "width" => dimension().map(StyleProp::Width),
            _ => return Err("unknown style prop"),
        };

        // TODO: better error reporting
        parser.parse(value.as_bytes()).map_err(|_| "parse error")
    }

    fn align<'a>() -> Parser<'a, u8, Align> {
        keyword().convert(|kw| match kw {
            b"auto" => Ok(Align::Auto),
            b"start" => Ok(Align::Start),
            b"flex-start" => Ok(Align::Start),
            b"center" => Ok(Align::Center),
            b"end" => Ok(Align::End),
            b"flex-end" => Ok(Align::End),
            b"stretch" => Ok(Align::Stretch),
            b"baseline" => Ok(Align::Baseline),
            b"space-between" => Ok(Align::SpaceBetween),
            b"space-around" => Ok(Align::SpaceAround),
            b"space-evenly" => Ok(Align::SpaceEvenly),

            _ => Err("invalid align"),
        })
    }

    fn color<'a>() -> Parser<'a, u8, Color> {
        fn hex_val(byte: u8) -> u8 {
            (byte as char).to_digit(16).unwrap() as u8
        }

        // TODO: rgb/rgba()

        sym(b'#')
            * is_a(hex_digit).repeat(3..9).collect().convert(|hex| match hex.len() {
                8 | 6 => {
                    let mut num = u32::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();

                    if hex.len() == 6 {
                        num = num << 8 | 0xFF;
                    }

                    Ok(Color {
                        r: ((num >> 24) & 0xFF) as u8,
                        g: ((num >> 16) & 0xFF) as u8,
                        b: ((num >> 8) & 0xFF) as u8,
                        a: (num & 0xFF) as u8,
                    })
                }

                4 | 3 => Ok(Color {
                    r: hex_val(hex[0]) * 17,
                    g: hex_val(hex[1]) * 17,
                    b: hex_val(hex[2]) * 17,
                    a: hex.get(3).map(|&v| hex_val(v) * 17).unwrap_or(255),
                }),

                _ => Err("invalid color"),
            })
    }

    fn dimension<'a>() -> Parser<'a, u8, Dimension> {
        let px = (float() - seq(b"px")).map(Dimension::Px);
        let percent = (float() - sym(b'%')).map(Dimension::Percent);
        let auto = seq(b"auto").map(|_| Dimension::Auto);
        let zero = sym(b'0').map(|_| Dimension::Px(0.));

        px | percent | auto | zero
    }

    fn border_style<'a>() -> Parser<'a, u8, BorderStyle> {
        keyword().convert(|kw| match kw {
            b"none" => Ok(BorderStyle::None),
            b"hidden" => Ok(BorderStyle::Hidden),
            b"dotted" => Ok(BorderStyle::Dotted),
            b"dashed" => Ok(BorderStyle::Dashed),
            b"solid" => Ok(BorderStyle::Solid),
            b"double" => Ok(BorderStyle::Double),
            b"groove" => Ok(BorderStyle::Groove),
            b"ridge" => Ok(BorderStyle::Ridge),
            b"inset" => Ok(BorderStyle::Inset),
            b"outset" => Ok(BorderStyle::Outset),

            _ => Err("invalid border style"),
        })
    }

    // TODO: box_shadow

    fn display<'a>() -> Parser<'a, u8, Display> {
        keyword().convert(|kw| match kw {
            b"none" => Ok(Display::None),
            b"block" => Ok(Display::Block),
            b"inline" => Ok(Display::Inline),
            b"flex" => Ok(Display::Flex),

            _ => Err("invalid display"),
        })
    }

    fn flex_direction<'a>() -> Parser<'a, u8, FlexDirection> {
        keyword().convert(|kw| match kw {
            b"row" => Ok(FlexDirection::Row),
            b"column" => Ok(FlexDirection::Column),
            b"row-reverse" => Ok(FlexDirection::RowReverse),
            b"column-reverse" => Ok(FlexDirection::ColumnReverse),

            _ => Err("invalid flex direction"),
        })
    }

    fn flex_wrap<'a>() -> Parser<'a, u8, FlexWrap> {
        keyword().convert(|kw| match kw {
            b"nowrap" => Ok(FlexWrap::NoWrap),
            b"wrap" => Ok(FlexWrap::Wrap),
            b"wrap-reverse" => Ok(FlexWrap::WrapReverse),

            _ => Err("invalid flex wrap"),
        })
    }

    fn overflow<'a>() -> Parser<'a, u8, Overflow> {
        keyword().convert(|kw| match kw {
            b"visible" => Ok(Overflow::Visible),
            b"hidden" => Ok(Overflow::Hidden),
            b"scroll" => Ok(Overflow::Scroll),
            b"auto" => Ok(Overflow::Auto),

            _ => Err("invalid overflow"),
        })
    }

    fn position<'a>() -> Parser<'a, u8, Position> {
        keyword().convert(|kw| match kw {
            b"static" => Ok(Position::Static),
            b"relative" => Ok(Position::Relative),
            b"absolute" => Ok(Position::Absolute),
            b"sticky" => Ok(Position::Sticky),

            _ => Err("invalid position"),
        })
    }

    fn text_align<'a>() -> Parser<'a, u8, TextAlign> {
        keyword().convert(|kw| match kw {
            b"left" => Ok(TextAlign::Left),
            b"center" => Ok(TextAlign::Center),
            b"right" => Ok(TextAlign::Right),
            b"justify" => Ok(TextAlign::Justify),

            _ => Err("invalid text align"),
        })
    }

    // TODO: transform

    fn visibility<'a>() -> Parser<'a, u8, Visibility> {
        keyword().convert(|kw| match kw {
            b"visible" => Ok(Visibility::Visible),
            b"hidden" => Ok(Visibility::Hidden),
            b"collapse" => Ok(Visibility::Collapse),

            _ => Err("invalid visibility"),
        })
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
        fn parse_prop() {
            assert_eq!(parse_style_prop("padding-left", "10px"), Ok(StyleProp::PaddingLeft(Dimension::Px(10.))));
            assert_eq!(parse_style_prop("margin-top", "5%"), Ok(StyleProp::MarginTop(Dimension::Percent(5.))));
            assert_eq!(parse_style_prop("opacity", "1"), Ok(StyleProp::Opacity(1.)));
            assert_eq!(parse_style_prop("color", "#000000"), Ok(StyleProp::Color(Color { r: 0, g: 0, b: 0, a: 255 })));
        }

        #[test]
        fn parse_align() {
            assert_eq!(align().parse(b"auto"), Ok(Align::Auto));
            assert_eq!(align().parse(b"start"), Ok(Align::Start));
            assert_eq!(align().parse(b"flex-start"), Ok(Align::Start));
            assert_eq!(align().parse(b"center"), Ok(Align::Center));
            assert_eq!(align().parse(b"end"), Ok(Align::End));
            assert_eq!(align().parse(b"flex-end"), Ok(Align::End));
            assert_eq!(align().parse(b"stretch"), Ok(Align::Stretch));
            assert_eq!(align().parse(b"baseline"), Ok(Align::Baseline));
            assert_eq!(align().parse(b"space-between"), Ok(Align::SpaceBetween));
            assert_eq!(align().parse(b"space-around"), Ok(Align::SpaceAround));
            assert_eq!(align().parse(b"space-evenly"), Ok(Align::SpaceEvenly));
        }

        #[test]
        fn parse_dimension() {
            assert_eq!(dimension().parse(b"auto"), Ok(Dimension::Auto));
            assert_eq!(dimension().parse(b"10px"), Ok(Dimension::Px(10.)));
            assert_eq!(dimension().parse(b"100%"), Ok(Dimension::Percent(100.)));
            assert_eq!(dimension().parse(b"0"), Ok(Dimension::Px(0.)));
        }

        #[test]
        fn parse_color() {
            assert_eq!(color().parse(b"#000000"), Ok(Color { r: 0, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#ff0000"), Ok(Color { r: 255, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#00ff00"), Ok(Color { r: 0, g: 255, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#0000ff"), Ok(Color { r: 0, g: 0, b: 255, a: 255 }));

            assert_eq!(color().parse(b"#80808080"), Ok(Color { r: 128, g: 128, b: 128, a: 128 }));
            assert_eq!(color().parse(b"#00000080"), Ok(Color { r: 0, g: 0, b: 0, a: 128 }));

            assert_eq!(color().parse(b"#000"), Ok(Color { r: 0, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#f00"), Ok(Color { r: 255, g: 0, b: 0, a: 255 }));
            assert_eq!(color().parse(b"#fff"), Ok(Color { r: 255, g: 255, b: 255, a: 255 }));

            assert_eq!(color().parse(b"#0000"), Ok(Color { r: 0, g: 0, b: 0, a: 0 }));
            assert_eq!(color().parse(b"#f00f"), Ok(Color { r: 255, g: 0, b: 0, a: 255 }));

            //assert_eq!(color().parse(b"rgb(0, 0, 0)"), Ok(Color { r: 0, g: 0, b: 0, a: 255 }));
            //assert_eq!(color().parse(b"rgba(0, 0, 0, 0)"), Ok(Color { r: 0, g: 0, b: 0, a: 0 }));
        }

        #[test]
        fn parse_border_style() {
            assert_eq!(border_style().parse(b"none"), Ok(BorderStyle::None));
            assert_eq!(border_style().parse(b"hidden"), Ok(BorderStyle::Hidden));
            assert_eq!(border_style().parse(b"dotted"), Ok(BorderStyle::Dotted));
            assert_eq!(border_style().parse(b"dashed"), Ok(BorderStyle::Dashed));
            assert_eq!(border_style().parse(b"solid"), Ok(BorderStyle::Solid));
            assert_eq!(border_style().parse(b"double"), Ok(BorderStyle::Double));
            assert_eq!(border_style().parse(b"groove"), Ok(BorderStyle::Groove));
            assert_eq!(border_style().parse(b"ridge"), Ok(BorderStyle::Ridge));
            assert_eq!(border_style().parse(b"inset"), Ok(BorderStyle::Inset));
            assert_eq!(border_style().parse(b"outset"), Ok(BorderStyle::Outset));
        }

        // TODO: parse_box_shadow

        #[test]
        fn parse_display() {
            assert_eq!(display().parse(b"none"), Ok(Display::None));
            assert_eq!(display().parse(b"block"), Ok(Display::Block));
            assert_eq!(display().parse(b"inline"), Ok(Display::Inline));
            assert_eq!(display().parse(b"flex"), Ok(Display::Flex));
        }

        #[test]
        fn parse_flex_direction() {
            assert_eq!(flex_direction().parse(b"row"), Ok(FlexDirection::Row));
            assert_eq!(flex_direction().parse(b"column"), Ok(FlexDirection::Column));
            assert_eq!(flex_direction().parse(b"row-reverse"), Ok(FlexDirection::RowReverse));
            assert_eq!(flex_direction().parse(b"column-reverse"), Ok(FlexDirection::ColumnReverse));
        }

        #[test]
        fn parse_flex_wrap() {
            assert_eq!(flex_wrap().parse(b"nowrap"), Ok(FlexWrap::NoWrap));
            assert_eq!(flex_wrap().parse(b"wrap"), Ok(FlexWrap::Wrap));
            assert_eq!(flex_wrap().parse(b"wrap-reverse"), Ok(FlexWrap::WrapReverse));
        }

        #[test]
        fn parse_overflow() {
            assert_eq!(overflow().parse(b"visible"), Ok(Overflow::Visible));
            assert_eq!(overflow().parse(b"hidden"), Ok(Overflow::Hidden));
            assert_eq!(overflow().parse(b"scroll"), Ok(Overflow::Scroll));
            assert_eq!(overflow().parse(b"auto"), Ok(Overflow::Auto));
        }

        #[test]
        fn parse_position() {
            assert_eq!(position().parse(b"static"), Ok(Position::Static));
            assert_eq!(position().parse(b"relative"), Ok(Position::Relative));
            assert_eq!(position().parse(b"absolute"), Ok(Position::Absolute));
            assert_eq!(position().parse(b"sticky"), Ok(Position::Sticky));
        }

        #[test]
        fn parse_text_align() {
            assert_eq!(text_align().parse(b"left"), Ok(TextAlign::Left));
            assert_eq!(text_align().parse(b"center"), Ok(TextAlign::Center));
            assert_eq!(text_align().parse(b"right"), Ok(TextAlign::Right));
            assert_eq!(text_align().parse(b"justify"), Ok(TextAlign::Justify));
        }

        // TODO: parse_transform

        #[test]
        fn parse_visibility() {
            assert_eq!(visibility().parse(b"visible"), Ok(Visibility::Visible));
            assert_eq!(visibility().parse(b"hidden"), Ok(Visibility::Hidden));
            assert_eq!(visibility().parse(b"collapse"), Ok(Visibility::Collapse));
        }
    }
}
