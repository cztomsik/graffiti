use super::*;
use crate::util::Atom;
use pom::char_class::{alphanum, digit, hex_digit};
use pom::parser::*;

type Parser<'a, T> = pom::parser::Parser<'a, Token<'a>, T>;
type Token<'a> = &'a str;

pub(super) fn sheet<'a>() -> Parser<'a, StyleSheet> {
    // super-dumb forgiving
    let unknown = (!rule() * (!sym("}") * take(1)).repeat(0..) - sym("}")).map(|u| {
        println!("unknown: {:?}", u);
    });

    (unknown.repeat(0..) * rule())
        .repeat(0..)
        .map(|rules| StyleSheet { rules })
}

fn rule<'a>() -> Parser<'a, Rule> {
    let rule = selector() - sym("{") + style() - sym("}");

    rule.map(|(selector, style)| Rule::new(selector, style))
}

pub(super) fn selector<'a>() -> Parser<'a, Selector> {
    let tag = || {
        let ident = || ident().map(Atom::from);
        let local_name = ident().map(Component::LocalName);
        let id = sym("#") * ident().map(Component::Identifier);
        let class_name = sym(".") * ident().map(Component::ClassName);
        let universal = sym("*").map(|_| SelectorPart::Combinator(Combinator::Universal));

        universal | (id | class_name | local_name).map(SelectorPart::Component)
    };

    // note we parse child/descendant but we flip the final order so it's parent/ancestor
    let child = sym(">").map(|_| Combinator::Parent);
    let descendant = sym(" ").map(|_| Combinator::Ancestor);
    let or = sym(",").map(|_| Combinator::Or);
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
    // TODO: forgiving parser should probably work a bit differently

    let prop_name = ident();
    // any chunk of tokens before ";" or "}"
    let prop_value = (!sym(";") * !sym("}") * any()).repeat(1..);
    let prop = prop_name - sym(":") + prop_value - sym(";").repeat(0..);

    prop.repeat(0..).map(|props| Style {
        // skip unknown
        props: props.iter().filter_map(|(p, v)| parse_style_prop(p, v).ok()).collect(),
    })
}

pub(super) fn parse_style_prop<'a>(prop: Token, value: &[Token]) -> pom::Result<StyleProp> {
    prop_parser(prop).parse(value)
}

fn prop_parser<'a>(prop: &str) -> Parser<'a, StyleProp> {
    use self::value as v;

    match prop {
        // size
        "width" => v(dimension()).map(StyleProp::Width),
        "height" => v(dimension()).map(StyleProp::Height),
        "min-width" => v(dimension()).map(StyleProp::MinWidth),
        "min-height" => v(dimension()).map(StyleProp::MinHeight),
        "max-width" => v(dimension()).map(StyleProp::MaxWidth),
        "max-height" => v(dimension()).map(StyleProp::MaxHeight),

        // padding
        "padding-top" => v(dimension()).map(StyleProp::PaddingTop),
        "padding-right" => v(dimension()).map(StyleProp::PaddingRight),
        "padding-bottom" => v(dimension()).map(StyleProp::PaddingBottom),
        "padding-left" => v(dimension()).map(StyleProp::PaddingLeft),

        // margin
        "margin-top" => v(dimension()).map(StyleProp::MarginTop),
        "margin-right" => v(dimension()).map(StyleProp::MarginRight),
        "margin-bottom" => v(dimension()).map(StyleProp::MarginBottom),
        "margin-left" => v(dimension()).map(StyleProp::MarginLeft),

        // background
        "background-color" => v(color()).map(StyleProp::BackgroundColor),

        // border-radius
        "border-top-left-radius" => v(dimension()).map(StyleProp::BorderTopLeftRadius),
        "border-top-right-radius" => v(dimension()).map(StyleProp::BorderTopRightRadius),
        "border-bottom-right-radius" => v(dimension()).map(StyleProp::BorderBottomRightRadius),
        "border-bottom-left-radius" => v(dimension()).map(StyleProp::BorderBottomLeftRadius),

        // border
        "border-top-width" => v(dimension()).map(StyleProp::BorderTopWidth),
        "border-top-style" => v(border_style()).map(StyleProp::BorderTopStyle),
        "border-top-color" => v(color()).map(StyleProp::BorderTopColor),
        "border-right-width" => v(dimension()).map(StyleProp::BorderRightWidth),
        "border-right-style" => v(border_style()).map(StyleProp::BorderRightStyle),
        "border-right-color" => v(color()).map(StyleProp::BorderRightColor),
        "border-bottom-width" => v(dimension()).map(StyleProp::BorderBottomWidth),
        "border-bottom-style" => v(border_style()).map(StyleProp::BorderBottomStyle),
        "border-bottom-color" => v(color()).map(StyleProp::BorderBottomColor),
        "border-left-width" => v(dimension()).map(StyleProp::BorderLeftWidth),
        "border-left-style" => v(border_style()).map(StyleProp::BorderLeftStyle),
        "border-left-color" => v(color()).map(StyleProp::BorderLeftColor),

        // flex
        "flex-basis" => v(dimension()).map(StyleProp::FlexBasis),
        "flex-grow" => v(float()).map(StyleProp::FlexGrow),
        "flex-shrink" => v(float()).map(StyleProp::FlexShrink),
        "flex-direction" => v(flex_direction()).map(StyleProp::FlexDirection),
        "flex-wrap" => v(flex_wrap()).map(StyleProp::FlexWrap),
        "align-content" => v(align()).map(StyleProp::AlignContent),
        "align-items" => v(align()).map(StyleProp::AlignItems),
        "align-self" => v(align()).map(StyleProp::AlignSelf),
        "justify-content" => v(align()).map(StyleProp::JustifyContent),

        // text
        "font-family" => v(font_family()).map(StyleProp::FontFamily),
        "font-size" => v(dimension()).map(StyleProp::FontSize),
        "line-height" => v(dimension()).map(StyleProp::LineHeight),
        "text-align" => v(text_align()).map(StyleProp::TextAlign),
        "color" => v(color()).map(StyleProp::Color),

        // outline
        "outline-color" => v(color()).map(StyleProp::OutlineColor),
        "outline-style" => v(border_style()).map(StyleProp::OutlineStyle),
        "outline-width" => v(dimension()).map(StyleProp::OutlineWidth),

        // overflow
        "overflow-x" => v(overflow()).map(StyleProp::OverflowX),
        "overflow-y" => v(overflow()).map(StyleProp::OverflowY),

        // position
        "position" => v(position()).map(StyleProp::Position),
        "top" => v(dimension()).map(StyleProp::Top),
        "right" => v(dimension()).map(StyleProp::Right),
        "bottom" => v(dimension()).map(StyleProp::Bottom),
        "left" => v(dimension()).map(StyleProp::Left),

        // other
        "display" => v(display()).map(StyleProp::Display),
        "opacity" => v(float()).map(StyleProp::Opacity),
        "visibility" => v(visibility()).map(StyleProp::Visibility),

        _ => fail("unknown style prop"),
    }
}

fn value<'a, T: 'static>(specified: Parser<'a, T>) -> Parser<'a, CssValue<T>> {
    let inherit = sym("inherit").map(|_| CssValue::Inherit);
    let initial = sym("initial").map(|_| CssValue::Initial);
    let unset = sym("unset").map(|_| CssValue::Unset);

    specified.map(CssValue::Specified) | inherit | initial | unset
}

fn dimension<'a>() -> Parser<'a, CssDimension> {
    let px = (float() - sym("px")).map(CssDimension::Px);
    let percent = (float() - sym("%")).map(CssDimension::Percent);
    let auto = sym("auto").map(|_| CssDimension::Auto);
    let zero = sym("0").map(|_| CssDimension::Px(0.));

    px | percent | auto | zero
}

fn color<'a>() -> Parser<'a, CssColor> {
    fn hex_val(byte: u8) -> u8 {
        (byte as char).to_digit(16).unwrap() as u8
    }

    // TODO: rgb/rgba()

    sym("#")
        * any().convert(|hex: &str| {
            println!("color: #{}", hex);
            let hex = hex.as_bytes();

            Ok(match hex.len() {
                8 | 6 => {
                    let mut num = u32::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();

                    if hex.len() == 6 {
                        num = num << 8 | 0xFF;
                    }

                    CssColor {
                        r: ((num >> 24) & 0xFF) as u8,
                        g: ((num >> 16) & 0xFF) as u8,
                        b: ((num >> 8) & 0xFF) as u8,
                        a: (num & 0xFF) as u8,
                    }
                }

                4 | 3 => CssColor {
                    r: hex_val(hex[0]) * 17,
                    g: hex_val(hex[1]) * 17,
                    b: hex_val(hex[2]) * 17,
                    a: hex.get(3).map(|&v| hex_val(v) * 17).unwrap_or(255),
                },

                _ => return Err("invalid color"),
            })
        })
}

fn align<'a>() -> Parser<'a, CssAlign> {
    ident().convert(|ident| {
        Ok(match ident {
            "auto" => CssAlign::Auto,
            "start" => CssAlign::Start,
            "flex-start" => CssAlign::Start,
            "center" => CssAlign::Center,
            "end" => CssAlign::End,
            "flex-end" => CssAlign::End,
            "stretch" => CssAlign::Stretch,
            "baseline" => CssAlign::Baseline,
            "space-between" => CssAlign::SpaceBetween,
            "space-around" => CssAlign::SpaceAround,
            "space-evenly" => CssAlign::SpaceEvenly,

            _ => return Err("invalid align"),
        })
    })
}

fn border_style<'a>() -> Parser<'a, CssBorderStyle> {
    ident().convert(|ident| {
        Ok(match ident {
            "none" => CssBorderStyle::None,
            "hidden" => CssBorderStyle::Hidden,
            "dotted" => CssBorderStyle::Dotted,
            "dashed" => CssBorderStyle::Dashed,
            "solid" => CssBorderStyle::Solid,
            "double" => CssBorderStyle::Double,
            "groove" => CssBorderStyle::Groove,
            "ridge" => CssBorderStyle::Ridge,
            "inset" => CssBorderStyle::Inset,
            "outset" => CssBorderStyle::Outset,

            _ => return Err("invalid border style"),
        })
    })
}

fn display<'a>() -> Parser<'a, CssDisplay> {
    ident().convert(|ident| {
        Ok(match ident {
            "none" => CssDisplay::None,
            "block" => CssDisplay::Block,
            "inline" => CssDisplay::Inline,
            "flex" => CssDisplay::Flex,

            _ => return Err("invalid display"),
        })
    })
}

fn flex_direction<'a>() -> Parser<'a, CssFlexDirection> {
    ident().convert(|ident| {
        Ok(match ident {
            "row" => CssFlexDirection::Row,
            "column" => CssFlexDirection::Column,
            "row-reverse" => CssFlexDirection::RowReverse,
            "column-reverse" => CssFlexDirection::ColumnReverse,

            _ => return Err("invalid flex direction"),
        })
    })
}

fn flex_wrap<'a>() -> Parser<'a, CssFlexWrap> {
    ident().convert(|ident| {
        Ok(match ident {
            "nowrap" => CssFlexWrap::NoWrap,
            "wrap" => CssFlexWrap::Wrap,
            "wrap-reverse" => CssFlexWrap::WrapReverse,

            _ => return Err("invalid flex wrap"),
        })
    })
}

fn overflow<'a>() -> Parser<'a, CssOverflow> {
    ident().convert(|ident| {
        Ok(match ident {
            "visible" => CssOverflow::Visible,
            "hidden" => CssOverflow::Hidden,
            "scroll" => CssOverflow::Scroll,
            "auto" => CssOverflow::Auto,

            _ => return Err("invalid overflow"),
        })
    })
}

fn position<'a>() -> Parser<'a, CssPosition> {
    ident().convert(|ident| {
        Ok(match ident {
            "static" => CssPosition::Static,
            "relative" => CssPosition::Relative,
            "absolute" => CssPosition::Absolute,
            "sticky" => CssPosition::Sticky,

            _ => return Err("invalid position"),
        })
    })
}

fn font_family<'a>() -> Parser<'a, Atom<String>> {
    // TODO: multiple, strings
    //       but keep it as Atom<String> because that is easy to
    //       map/cache to FontQuery and I'd like to keep CSS unaware of fonts
    is_a(|t: &str| alphanum_dash(t.as_bytes()[0])).map(Atom::from)
}

fn text_align<'a>() -> Parser<'a, CssTextAlign> {
    ident().convert(|ident| {
        Ok(match ident {
            "left" => CssTextAlign::Left,
            "center" => CssTextAlign::Center,
            "right" => CssTextAlign::Right,
            "justify" => CssTextAlign::Justify,

            _ => return Err("invalid text align"),
        })
    })
}

fn visibility<'a>() -> Parser<'a, CssVisibility> {
    ident().convert(|ident| {
        Ok(match ident {
            "visible" => CssVisibility::Visible,
            "hidden" => CssVisibility::Hidden,
            "collapse" => CssVisibility::Collapse,

            _ => return Err("invalid visibility"),
        })
    })
}

fn float<'a>() -> Parser<'a, f32> {
    num().convert(str::parse)
}

fn num<'a>() -> Parser<'a, &'a str> {
    is_a(|t: &str| digit(t.as_bytes()[0]))
}

fn ident<'a>() -> Parser<'a, &'a str> {
    is_a(|t: &str| alphanum_dash(t.as_bytes()[0]))
}

fn fail<'a, T: 'static>(msg: &'static str) -> Parser<'a, T> {
    empty().convert(move |_| Err(msg))
}

fn alphanum_dash(b: u8) -> bool {
    alphanum(b) || b == b'-'
}

// different from https://drafts.csswg.org/css-syntax/#tokenization
// (main purpose here is to strip comments and to keep strings together)
pub(super) fn tokenize(input: &[u8]) -> Vec<Token> {
    use pom::parser::*;

    let comment = seq(b"/*") * (!seq(b"*/") * take(1)).repeat(0..).discard() - seq(b"*/");
    let space = one_of(b" \t\r\n").repeat(1..).map(|_| &b" "[..]);
    let hexnum = is_a(hex_digit).repeat(1..).collect();
    let num = one_of(b"-.0123456789").repeat(1..).collect();
    let ident = is_a(alphanum_dash).repeat(1..).collect();
    let string1 = (sym(b'\'') + none_of(b"'").repeat(0..) + sym(b'\'')).collect();
    let string2 = (sym(b'"') + none_of(b"\"").repeat(0..) + sym(b'"')).collect();
    let special = any().collect();

    // spaces are "normalized" but they still can appear multiple times because of stripped comments
    let token = comment.opt() * (space | ident | hexnum | num | string1 | string2 | special);
    let tokens = token.convert(std::str::from_utf8).repeat(0..).parse(input).unwrap();

    // keep space for selectors & multi-values
    // TODO: this was easier than combinators
    let (mut res, mut keep_space) = (Vec::new(), false);
    for (i, &t) in tokens.iter().enumerate() {
        if t == " " {
            if !keep_space {
                continue;
            }

            if let Some(&next) = tokens.get(i + 1) {
                if !(alphanum_dash(next.as_bytes()[0]) || next == "." || next == "#" || next == "*") {
                    continue;
                }
            }
        }

        res.push(t);
        keep_space = alphanum_dash(t.as_bytes()[0]) || t == "*" || t == "]"
    }

    res
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

    /*
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
    */
}
