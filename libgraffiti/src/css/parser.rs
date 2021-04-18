use super::*;
use crate::util::Atom;
use pom::char_class::{alpha, alphanum, hex_digit};
use pom::parser::*;

type Parser<'a, T> = pom::parser::Parser<'a, u8, T>;

pub(super) fn sheet<'a>() -> Parser<'a, StyleSheet> {
    let comment = seq(b"/*") * (!seq(b"*/") * take(1)).repeat(0..) * seq(b"*/");

    ((space() * comment).repeat(0..) * space() * rule())
        .repeat(0..)
        .map(|rules| StyleSheet { rules })
}

fn rule<'a>() -> Parser<'a, Rule> {
    let rule = selector() - space() - sym(b'{') - space() + style() - space() - sym(b'}');

    rule.map(|(selector, style)| Rule::new(selector, style))
}

pub(super) fn selector<'a>() -> Parser<'a, Selector> {
    let tag = || {
        let ident = || ident().convert(std::str::from_utf8).map(Atom::from);
        let local_name = ident().map(Component::LocalName);
        let id = sym(b'#') * ident().map(Component::Identifier);
        let class_name = sym(b'.') * ident().map(Component::ClassName);
        let universal = sym(b'*').map(|_| SelectorPart::Combinator(Combinator::Universal));

        universal | (local_name | id | class_name).map(SelectorPart::Component)
    };

    // note we parse child/descendant but we flip the final order so it's parent/ancestor
    let child = space() * sym(b'>') * space().map(|_| Combinator::Parent);
    let descendant = sym(b' ').repeat(1..).map(|_| Combinator::Ancestor);
    let or = space() * sym(b',') * space().map(|_| Combinator::Or);
    let comb = (child | descendant | or).map(SelectorPart::Combinator);

    let selector = tag() + (comb.opt() + tag()).repeat(0..);

    selector.map(|(head, tail)| {
        let mut parts = Vec::with_capacity(tail.len() + 1);

        // reversed (child/descendant -> parent/ancestor)
        for (comb, tag) in tail.into_iter().rev() {
            parts.push(tag);

            if let Some(comb) = comb {
                parts.push(comb);
            }
        }

        parts.push(head);

        Selector { parts }
    })
}

pub(super) fn style<'a>() -> Parser<'a, Style> {
    // TODO: quotes, comments, etc.
    let prop_name = is_a(alpha_dash).repeat(1..).collect();
    let prop_value = none_of(b";{}\"'").repeat(0..).collect();
    let prop = prop_name - sym(b':') - space() + prop_value - one_of(b"; \n\r\t").repeat(0..);

    prop.repeat(0..).map(|props| Style {
        // skip unknown
        props: props.iter().filter_map(|(p, v)| parse_style_prop(p, v).ok()).collect(),
    })
}

pub(super) fn parse_style_prop<'a>(prop: &'a [u8], value: &'a [u8]) -> Result<StyleProp, &'a str> {
    // TODO: better error reporting
    prop_parser(prop).parse(value).map_err(|_| "invalid style prop")
}

fn prop_parser<'a>(prop: &'a [u8]) -> Parser<'a, StyleProp> {
    use self::value as v;

    match prop {
        // size
        b"width" => v(dimension()).map(StyleProp::Width),
        b"height" => v(dimension()).map(StyleProp::Height),
        b"min-width" => v(dimension()).map(StyleProp::MinWidth),
        b"min-height" => v(dimension()).map(StyleProp::MinHeight),
        b"max-width" => v(dimension()).map(StyleProp::MaxWidth),
        b"max-height" => v(dimension()).map(StyleProp::MaxHeight),

        // padding
        b"padding-top" => v(dimension()).map(StyleProp::PaddingTop),
        b"padding-right" => v(dimension()).map(StyleProp::PaddingRight),
        b"padding-bottom" => v(dimension()).map(StyleProp::PaddingBottom),
        b"padding-left" => v(dimension()).map(StyleProp::PaddingLeft),

        // margin
        b"margin-top" => v(dimension()).map(StyleProp::MarginTop),
        b"margin-right" => v(dimension()).map(StyleProp::MarginRight),
        b"margin-bottom" => v(dimension()).map(StyleProp::MarginBottom),
        b"margin-left" => v(dimension()).map(StyleProp::MarginLeft),

        // background
        b"background-color" => v(color()).map(StyleProp::BackgroundColor),

        // border-radius
        b"border-top-left-radius" => v(dimension()).map(StyleProp::BorderTopLeftRadius),
        b"border-top-right-radius" => v(dimension()).map(StyleProp::BorderTopRightRadius),
        b"border-bottom-right-radius" => v(dimension()).map(StyleProp::BorderBottomRightRadius),
        b"border-bottom-left-radius" => v(dimension()).map(StyleProp::BorderBottomLeftRadius),

        // border
        b"border-top-width" => v(dimension()).map(StyleProp::BorderTopWidth),
        b"border-top-style" => v(border_style()).map(StyleProp::BorderTopStyle),
        b"border-top-color" => v(color()).map(StyleProp::BorderTopColor),
        b"border-right-width" => v(dimension()).map(StyleProp::BorderRightWidth),
        b"border-right-style" => v(border_style()).map(StyleProp::BorderRightStyle),
        b"border-right-color" => v(color()).map(StyleProp::BorderRightColor),
        b"border-bottom-width" => v(dimension()).map(StyleProp::BorderBottomWidth),
        b"border-bottom-style" => v(border_style()).map(StyleProp::BorderBottomStyle),
        b"border-bottom-color" => v(color()).map(StyleProp::BorderBottomColor),
        b"border-left-width" => v(dimension()).map(StyleProp::BorderLeftWidth),
        b"border-left-style" => v(border_style()).map(StyleProp::BorderLeftStyle),
        b"border-left-color" => v(color()).map(StyleProp::BorderLeftColor),

        // flex
        b"flex-basis" => v(dimension()).map(StyleProp::FlexBasis),
        b"flex-grow" => v(float()).map(StyleProp::FlexGrow),
        b"flex-shrink" => v(float()).map(StyleProp::FlexShrink),
        b"flex-direction" => v(flex_direction()).map(StyleProp::FlexDirection),
        b"flex-wrap" => v(flex_wrap()).map(StyleProp::FlexWrap),
        b"align-content" => v(align()).map(StyleProp::AlignContent),
        b"align-items" => v(align()).map(StyleProp::AlignItems),
        b"align-self" => v(align()).map(StyleProp::AlignSelf),
        b"justify-content" => v(align()).map(StyleProp::JustifyContent),

        // text
        b"font-family" => v(font_family()).map(StyleProp::FontFamily),
        b"font-size" => v(dimension()).map(StyleProp::FontSize),
        b"line-height" => v(dimension()).map(StyleProp::LineHeight),
        b"text-align" => v(text_align()).map(StyleProp::TextAlign),
        b"color" => v(color()).map(StyleProp::Color),

        // outline
        b"outline-color" => v(color()).map(StyleProp::OutlineColor),
        b"outline-style" => v(border_style()).map(StyleProp::OutlineStyle),
        b"outline-width" => v(dimension()).map(StyleProp::OutlineWidth),

        // overflow
        b"overflow-x" => v(overflow()).map(StyleProp::OverflowX),
        b"overflow-y" => v(overflow()).map(StyleProp::OverflowY),

        // position
        b"position" => v(position()).map(StyleProp::Position),
        b"top" => v(dimension()).map(StyleProp::Top),
        b"right" => v(dimension()).map(StyleProp::Right),
        b"bottom" => v(dimension()).map(StyleProp::Bottom),
        b"left" => v(dimension()).map(StyleProp::Left),

        // other
        b"display" => v(display()).map(StyleProp::Display),
        b"opacity" => v(float()).map(StyleProp::Opacity),
        b"visibility" => v(visibility()).map(StyleProp::Visibility),

        _ => fail("unknown style prop"),
    }
}

fn value<'a, T: 'static>(specified: Parser<'a, T>) -> Parser<'a, CssValue<T>> {
    let inherit = seq(b"inherit").map(|_| CssValue::Inherit);
    let initial = seq(b"initial").map(|_| CssValue::Initial);
    let unset = seq(b"unset").map(|_| CssValue::Unset);

    specified.map(CssValue::Specified) | inherit | initial | unset
}

fn dimension<'a>() -> Parser<'a, CssDimension> {
    let px = (float() - seq(b"px")).map(CssDimension::Px);
    let percent = (float() - sym(b'%')).map(CssDimension::Percent);
    let auto = seq(b"auto").map(|_| CssDimension::Auto);
    let zero = sym(b'0').map(|_| CssDimension::Px(0.));

    px | percent | auto | zero
}

fn color<'a>() -> Parser<'a, CssColor> {
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

                Ok(CssColor {
                    r: ((num >> 24) & 0xFF) as u8,
                    g: ((num >> 16) & 0xFF) as u8,
                    b: ((num >> 8) & 0xFF) as u8,
                    a: (num & 0xFF) as u8,
                })
            }

            4 | 3 => Ok(CssColor {
                r: hex_val(hex[0]) * 17,
                g: hex_val(hex[1]) * 17,
                b: hex_val(hex[2]) * 17,
                a: hex.get(3).map(|&v| hex_val(v) * 17).unwrap_or(255),
            }),

            _ => Err("invalid color"),
        })
}

fn align<'a>() -> Parser<'a, CssAlign> {
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

fn border_style<'a>() -> Parser<'a, CssBorderStyle> {
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

fn display<'a>() -> Parser<'a, CssDisplay> {
    keyword().convert(|kw| match kw {
        b"none" => Ok(CssDisplay::None),
        b"block" => Ok(CssDisplay::Block),
        b"inline" => Ok(CssDisplay::Inline),
        b"flex" => Ok(CssDisplay::Flex),

        _ => Err("invalid display"),
    })
}

fn flex_direction<'a>() -> Parser<'a, CssFlexDirection> {
    keyword().convert(|kw| match kw {
        b"row" => Ok(CssFlexDirection::Row),
        b"column" => Ok(CssFlexDirection::Column),
        b"row-reverse" => Ok(CssFlexDirection::RowReverse),
        b"column-reverse" => Ok(CssFlexDirection::ColumnReverse),

        _ => Err("invalid flex direction"),
    })
}

fn flex_wrap<'a>() -> Parser<'a, CssFlexWrap> {
    keyword().convert(|kw| match kw {
        b"nowrap" => Ok(CssFlexWrap::NoWrap),
        b"wrap" => Ok(CssFlexWrap::Wrap),
        b"wrap-reverse" => Ok(CssFlexWrap::WrapReverse),

        _ => Err("invalid flex wrap"),
    })
}

fn overflow<'a>() -> Parser<'a, CssOverflow> {
    keyword().convert(|kw| match kw {
        b"visible" => Ok(CssOverflow::Visible),
        b"hidden" => Ok(CssOverflow::Hidden),
        b"scroll" => Ok(CssOverflow::Scroll),
        b"auto" => Ok(CssOverflow::Auto),

        _ => Err("invalid overflow"),
    })
}

fn position<'a>() -> Parser<'a, CssPosition> {
    keyword().convert(|kw| match kw {
        b"static" => Ok(CssPosition::Static),
        b"relative" => Ok(CssPosition::Relative),
        b"absolute" => Ok(CssPosition::Absolute),
        b"sticky" => Ok(CssPosition::Sticky),

        _ => Err("invalid position"),
    })
}

fn font_family<'a>() -> Parser<'a, Atom<String>> {
    // TODO: extend pattern for quoted strings, support commas
    //       but keep it as Atom<String> because that is easy to
    //       map/cache to FontQuery and I'd like to keep CSS unaware of fonts
    is_a(alphanum_dash)
        .repeat(1..)
        .collect()
        .convert(std::str::from_utf8)
        .map(Atom::from)
}

fn text_align<'a>() -> Parser<'a, CssTextAlign> {
    keyword().convert(|kw| match kw {
        b"left" => Ok(CssTextAlign::Left),
        b"center" => Ok(CssTextAlign::Center),
        b"right" => Ok(CssTextAlign::Right),
        b"justify" => Ok(CssTextAlign::Justify),

        _ => Err("invalid text align"),
    })
}

fn visibility<'a>() -> Parser<'a, CssVisibility> {
    keyword().convert(|kw| match kw {
        b"visible" => Ok(CssVisibility::Visible),
        b"hidden" => Ok(CssVisibility::Hidden),
        b"collapse" => Ok(CssVisibility::Collapse),

        _ => Err("invalid visibility"),
    })
}

fn float<'a>() -> Parser<'a, f32> {
    num().convert(std::str::from_utf8).convert(str::parse)
}

fn num<'a>() -> Parser<'a, &'a [u8]> {
    one_of(b".0123456789").repeat(1..).collect()
}

fn space<'a>() -> Parser<'a, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn ident<'a>() -> Parser<'a, &'a [u8]> {
    is_a(alphanum_dash).repeat(1..).collect()
}

fn keyword<'a>() -> Parser<'a, &'a [u8]> {
    is_a(alpha_dash).repeat(1..).collect()
}

fn fail<'a, T: 'static>(msg: &'static str) -> Parser<'a, T> {
    empty().convert(move |_| Err(msg))
}

fn alpha_dash(b: u8) -> bool {
    alpha(b) || b == b'-'
}

fn alphanum_dash(b: u8) -> bool {
    alphanum(b) || b == b'-'
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn basic() {
        let sheet = StyleSheet::from("div { color: #fff }");

        assert_eq!(
            sheet,
            StyleSheet {
                rules: vec![Rule::new(Selector::from("div"), Style::from("color: #fff"))]
            }
        );

        // white-space
        assert_eq!(StyleSheet::from(" *{}").rules.len(), 1);
        assert_eq!(StyleSheet::from("\n*{\n}\n").rules.len(), 1);
    }

    #[test]
    fn parse_ua() {
        let ua = include_str!("../../resources/ua.css");
        let sheet = super::sheet().parse(ua.as_bytes()).unwrap();

        assert_eq!(sheet.rules.len(), 22);
    }

    #[test]
    fn parse_selector() {
        use super::Combinator::*;
        use super::Component::*;
        use SelectorPart::{Combinator, Component};

        let s = |s| Selector::from(s).parts;

        // simple
        assert_eq!(s("*"), &[Combinator(Universal)]);
        assert_eq!(s("body"), &[Component(LocalName("body".into()))]);
        assert_eq!(s("h2"), &[Component(LocalName("h2".into()))]);
        assert_eq!(s("#app"), &[Component(Identifier("app".into()))]);
        assert_eq!(s(".btn"), &[Component(ClassName("btn".into()))]);

        // combined
        assert_eq!(
            s(".btn.btn-primary"),
            &[
                Component(ClassName("btn-primary".into())),
                Component(ClassName("btn".into()))
            ]
        );
        assert_eq!(
            s("*.test"),
            &[Component(ClassName("test".into())), Combinator(Universal)]
        );
        assert_eq!(
            s("div#app.test"),
            &[
                Component(ClassName("test".into())),
                Component(Identifier("app".into())),
                Component(LocalName("div".into()))
            ]
        );

        // combined with combinators
        assert_eq!(
            s("body > div.test div#test"),
            &[
                Component(Identifier("test".into())),
                Component(LocalName("div".into())),
                Combinator(Ancestor),
                Component(ClassName("test".into())),
                Component(LocalName("div".into())),
                Combinator(Parent),
                Component(LocalName("body".into()))
            ]
        );

        // multi
        assert_eq!(
            s("html, body"),
            &[
                Component(LocalName("body".into())),
                Combinator(Or),
                Component(LocalName("html".into()))
            ]
        );
        assert_eq!(
            s("body > div, div button span"),
            &[
                Component(LocalName("span".into())),
                Combinator(Ancestor),
                Component(LocalName("button".into())),
                Combinator(Ancestor),
                Component(LocalName("div".into())),
                Combinator(Or),
                Component(LocalName("div".into())),
                Combinator(Parent),
                Component(LocalName("body".into())),
            ]
        );

        // invalid
        assert!(Selector::try_from("").is_err());
        assert!(Selector::try_from(" ").is_err());
        assert!(Selector::try_from("a,,b").is_err());
        assert!(Selector::try_from("a>>b").is_err());
    }

    #[test]
    fn parse_prop() {
        assert_eq!(
            parse_style_prop(b"text-align", b"inherit"),
            Ok(StyleProp::TextAlign(CssValue::Inherit))
        );
        assert_eq!(
            parse_style_prop(b"padding-left", b"10px"),
            Ok(StyleProp::PaddingLeft(CssValue::Specified(CssDimension::Px(10.))))
        );
        assert_eq!(
            parse_style_prop(b"margin-top", b"5%"),
            Ok(StyleProp::MarginTop(CssValue::Specified(CssDimension::Percent(5.))))
        );
        assert_eq!(
            parse_style_prop(b"opacity", b"1"),
            Ok(StyleProp::Opacity(CssValue::Specified(1.)))
        );
        assert_eq!(
            parse_style_prop(b"color", b"#000000"),
            Ok(StyleProp::Color(CssValue::Specified(CssColor::BLACK)))
        );
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
        assert_eq!(color().parse(b"#000000"), Ok(CssColor::BLACK));
        assert_eq!(color().parse(b"#ff0000"), Ok(CssColor::RED));
        assert_eq!(color().parse(b"#00ff00"), Ok(CssColor::GREEN));
        assert_eq!(color().parse(b"#0000ff"), Ok(CssColor::BLUE));

        assert_eq!(
            color().parse(b"#80808080"),
            Ok(CssColor::from_rgba8(128, 128, 128, 128))
        );
        assert_eq!(color().parse(b"#00000080"), Ok(CssColor::from_rgba8(0, 0, 0, 128)));

        assert_eq!(color().parse(b"#000"), Ok(CssColor::BLACK));
        assert_eq!(color().parse(b"#f00"), Ok(CssColor::RED));
        assert_eq!(color().parse(b"#fff"), Ok(CssColor::WHITE));

        assert_eq!(color().parse(b"#0000"), Ok(CssColor::TRANSPARENT));
        assert_eq!(color().parse(b"#f00f"), Ok(CssColor::RED));

        //assert_eq!(color().parse(b"rgb(0, 0, 0)"), Ok(Color { r: 0, g: 0, b: 0, a: 255 }));
        //assert_eq!(color().parse(b"rgba(0, 0, 0, 0)"), Ok(Color { r: 0, g: 0, b: 0, a: 0 }));
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
        assert_eq!(
            flex_direction().parse(b"column-reverse"),
            Ok(CssFlexDirection::ColumnReverse)
        );
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

    #[test]
    fn parse_visibility() {
        assert_eq!(visibility().parse(b"visible"), Ok(CssVisibility::Visible));
        assert_eq!(visibility().parse(b"hidden"), Ok(CssVisibility::Hidden));
        assert_eq!(visibility().parse(b"collapse"), Ok(CssVisibility::Collapse));
    }
}
