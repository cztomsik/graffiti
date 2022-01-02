// TODO: @keyframes xxx { from { .. } to { .. } }

// notes:
// - we are using parser-combinators (for both tokenizing & parsing)
//   - see https://github.com/J-F-Liu/pom for reference
//   - tokens are just &str, there are no other token types
//   - it's probably a bit inefficient but very expressive (~350 lines)
// - repeat() for skip/discard() should be alloc-free because of zero-sized types
// - collect() creates slice from start to end regardless of the results "inside"
//   (which means (a + b).collect() only takes "super-slice" of both matches)
// - we are only parsing known/valid props, which means tokenizer can be simpler
//   and we also get correct overriding for free (only valid prop should override prev one)

use super::{
    properties::{prop_parser, shorthand_parser},
    selector::{Combinator, Component, Selector, SelectorPart},
    CssBorderStyle, CssBoxShadow, CssColor, CssDimension, CssOverflow, CssStyleDeclaration, CssStyleRule,
    CssStyleSheet,
};
use crate::util::Atom;
use pom::char_class::alphanum;
use pom::parser::{any, empty, is_a, list, none_of, one_of, seq, skip, sym};
use std::convert::TryFrom;

pub type Parser<'a, T> = pom::parser::Parser<'a, Token<'a>, T>;

// TODO: maybe we could have a struct (with row/col), it just needs to be Deref<str>
//       but on the other hand, &str contains offset so we can compute row/col easily anyway
type Token<'a> = &'a str;

pub type ParseError = pom::Error;

pub fn sheet<'a>() -> Parser<'a, CssStyleSheet> {
    // anything until next "}}" (empty media is matched with unknown)
    let media = sym("@") * sym("media") * (!seq(&["}", "}"]) * skip(1)).repeat(1..).map(|_| None) - sym("}") - sym("}");
    // anything until next "}"
    let unknown = (!sym("}") * skip(1)).repeat(1..).map(|_| None) - sym("}").opt();

    (rule().map(Option::Some) | media | unknown)
        .repeat(0..)
        .map(|maybe_rules| CssStyleSheet::new(maybe_rules.into_iter().flatten().collect()))
}

pub fn rule<'a>() -> Parser<'a, CssStyleRule> {
    let rule = selector() - sym("{") + style() - sym("}");

    rule.map(|(selector, style)| CssStyleRule::new(selector, style))
}

pub fn selector<'a>() -> Parser<'a, Selector> {
    let tag = || {
        let ident = || ident().map(Atom::from);
        let local_name = ident().map(Component::LocalName);
        let id = sym("#") * ident().map(Component::Identifier);
        let class_name = sym(".") * ident().map(Component::ClassName);
        let attr = sym("[") * (!sym("]") * skip(1)).repeat(1..).map(|_| Component::Unsupported) - sym("]");
        let pseudo = sym(":").discard().repeat(1..3) * ident().map(|_| Component::Unsupported);
        let universal = sym("*").map(|_| SelectorPart::Combinator(Combinator::Universal));

        universal | (id | class_name | local_name | attr | pseudo).map(SelectorPart::Component)
    };

    // note we parse child/descendant but we flip the final order so it's parent/ancestor
    let child = sym(">").map(|_| Combinator::Parent);
    let descendant = sym(" ").map(|_| Combinator::Ancestor);
    let or = sym(",").map(|_| Combinator::Or);
    let unsupported = (sym("+") | sym("~")).map(|_| SelectorPart::Component(Component::Unsupported));
    let comb = (child | descendant | or).map(SelectorPart::Combinator) | unsupported;

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

pub fn style<'a>() -> Parser<'a, CssStyleDeclaration> {
    // any chunk of tokens before ";" or "}"
    let prop_value = (!sym(";") * !sym("}") * skip(1)).repeat(1..).collect();
    let prop = any() - sym(":") + prop_value - sym(";").discard().repeat(0..);

    prop.repeat(0..).map(|props| {
        let mut style = CssStyleDeclaration::default();

        for (p, v) in props {
            // skip unknown
            parse_prop_into(p, v, &mut style);
        }

        style
    })
}

pub fn parse_prop_into(prop: &str, value: &[&str], style: &mut CssStyleDeclaration) {
    if let Ok(p) = prop_parser(prop).parse(value) {
        style.add_prop(p);
    } else if let Ok(props) = shorthand_parser(prop).parse(value) {
        for p in props {
            style.add_prop(p);
        }
    }
}

pub fn css_enum<'a, T: 'a + TryFrom<&'a str, Error = &'static str>>() -> Parser<'a, T> {
    ident().convert(T::try_from)
}

pub fn dimension<'a>() -> Parser<'a, CssDimension> {
    let px = (float() - sym("px")).map(CssDimension::Px);
    let percent = (float() - sym("%")).map(CssDimension::Percent);
    let auto = sym("auto").map(|_| CssDimension::Auto);
    let zero = sym("0").map(|_| CssDimension::ZERO);
    let em = (float() - sym("em")).map(CssDimension::Em);
    let rem = (float() - sym("rem")).map(CssDimension::Rem);
    let vw = (float() - sym("vw")).map(CssDimension::Vw);
    let vh = (float() - sym("vh")).map(CssDimension::Vh);
    let vmin = sym("vmin").map(|_| CssDimension::Vmin);
    let vmax = sym("vmax").map(|_| CssDimension::Vmax);

    px | percent | auto | zero | em | rem | vw | vh | vmin | vmax
}

pub fn sides_of<'a, V: Copy + 'a>(parser: Parser<'a, V>) -> Parser<'a, (V, V, V, V)> {
    list(parser, sym(" ")).convert(|sides| {
        #[allow(clippy::match_ref_pats)]
        Ok(match &sides[..] {
            &[a, b, c, d] => (a, b, c, d),
            &[a, b, c] => (a, b, c, b),
            &[a, b] => (a, b, a, b),
            &[a] => (a, a, a, a),
            _ => return Err("expected 1-4 values"),
        })
    })
}

pub fn flex<'a>() -> Parser<'a, (f32, f32, CssDimension)> {
    (float() + (sym(" ") * float()).opt() + (sym(" ") * dimension()).opt())
        .map(|((grow, shrink), basis)| (grow, shrink.unwrap_or(1.), basis.unwrap_or(CssDimension::ZERO)))
}

pub fn overflow<'a>() -> Parser<'a, (CssOverflow, CssOverflow)> {
    (css_enum() + (sym(" ") * css_enum()).opt()).map(|(x, y)| (x, y.unwrap_or(x)))
}

pub fn outline<'a>() -> Parser<'a, (CssDimension, CssBorderStyle, CssColor)> {
    (dimension() + (sym(" ") * css_enum()) + (sym(" ") * color())).map(|((dim, style), color)| (dim, style, color))
}

pub fn background<'a>() -> Parser<'a, CssColor> {
    sym("none").map(|_| CssColor::TRANSPARENT) | color()
}

pub fn color<'a>() -> Parser<'a, CssColor> {
    fn hex_val(byte: u8) -> u8 {
        (byte as char).to_digit(16).unwrap() as u8
    }

    let hex_color = sym("#")
        * any().convert(|hex: &str| {
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

                _ => return Err("invalid hex color"),
            })
        });

    let rgb =
        sym("rgb") * sym("(") * (u8() - sym(",") + u8() - sym(",") + u8()).map(|((r, g), b)| CssColor::rgb(r, g, b))
            - sym(")");

    let rgba = sym("rgba")
        * sym("(")
        * (u8() - sym(",") + u8() - sym(",") + u8() - sym(",") + float())
            .map(|(((r, g), b), a)| CssColor::rgba(r, g, b, (255. * a) as _))
        - sym(")");

    let named_color = ident().convert(|name| CssColor::named(name).ok_or("unknown named color"));

    hex_color | rgb | rgba | named_color
}

pub fn font_family<'a>() -> Parser<'a, Atom<String>> {
    // TODO: multiple, strings
    //       but keep it as Atom<String> because that is easy to
    //       map/cache to FontQuery and I'd like to keep CSS unaware of fonts
    is_a(|t: &str| alphanum_dash(t.as_bytes()[0])).map(Atom::from)
}

pub fn box_shadow<'a>() -> Parser<'a, Box<CssBoxShadow>> {
    fail("TODO: parse box-shadow")
}

pub fn float<'a>() -> Parser<'a, f32> {
    any().convert(str::parse)
}

fn u8<'a>() -> Parser<'a, u8> {
    any().convert(str::parse)
}

fn ident<'a>() -> Parser<'a, &'a str> {
    is_a(|t: &str| alphanum_dash(t.as_bytes()[0]))
}

pub fn fail<'a, T: 'static>(msg: &'static str) -> Parser<'a, T> {
    empty().convert(move |_| Err(msg))
}

fn alphanum_dash(b: u8) -> bool {
    alphanum(b) || b == b'-'
}

// not sure if this is a good idea but it's useful for tokenization
// (hex is only consumed if it's after `#` but `#` is a separate token)
pub fn prev<'a, I: Clone>(n: usize) -> pom::parser::Parser<'a, I, ()> {
    pom::parser::Parser::new(move |_, position: usize| {
        if position >= n {
            Ok(((), position - n))
        } else {
            Err(pom::Error::Incomplete)
        }
    })
}

// different from https://drafts.csswg.org/css-syntax/#tokenization
// (main purpose here is to strip comments and to keep strings together)
pub fn tokenize(input: &[u8]) -> Vec<Token> {
    let comment = seq(b"/*") * (!seq(b"*/") * skip(1)).repeat(0..) - seq(b"*/");
    let space = one_of(b" \t\r\n").discard().repeat(1..).map(|_| &b" "[..]);
    let hex_or_id = prev(1) * sym(b'#') * is_a(alphanum).repeat(1..).collect();
    let num = (sym(b'-').opt() + one_of(b".0123456789").repeat(1..)).collect();
    let ident = is_a(alphanum_dash).repeat(1..).collect();
    let string1 = (sym(b'\'') + none_of(b"'").repeat(0..) + sym(b'\'')).collect();
    let string2 = (sym(b'"') + none_of(b"\"").repeat(0..) + sym(b'"')).collect();
    let other = any().collect();

    // spaces are "normalized" but they still can appear multiple times because of stripped comments
    let token = comment.opt() * (space | hex_or_id | num | ident | string1 | string2 | other);
    let tokens = token.convert(std::str::from_utf8).repeat(0..).parse(input).unwrap();

    // strip whitespace except for selectors & multi-values
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
        keep_space = alphanum_dash(t.as_bytes()[0]) || t == "*" || t == "]";
    }

    res
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(b""), Vec::<Token>::new());
        assert_eq!(tokenize(b" "), Vec::<Token>::new());
        assert_eq!(tokenize(b" /**/ /**/ "), Vec::<Token>::new());

        assert_eq!(tokenize(b"block"), vec!["block"]);
        assert_eq!(tokenize(b"10px"), vec!["10", "px"]);
        assert_eq!(tokenize(b"-10px"), vec!["-10", "px"]);
        assert_eq!(tokenize(b"ident2"), vec!["ident2"]);
        assert_eq!(tokenize(b"ff0"), vec!["ff0"]);
        assert_eq!(tokenize(b"00f"), vec!["00", "f"]);
        assert_eq!(tokenize(b"#00f"), vec!["#", "00f"]);
        assert_eq!(tokenize(b"0 0 10px 0"), vec!["0", " ", "0", " ", "10", "px", " ", "0"]);

        assert_eq!(tokenize(b"a b"), vec!["a", " ", "b"]);
        assert_eq!(tokenize(b".a .b"), vec![".", "a", " ", ".", "b"]);
        assert_eq!(
            tokenize(b" a .b #c *"),
            vec!["a", " ", ".", "b", " ", "#", "c", " ", "*"]
        );

        assert_eq!(tokenize(b"-webkit-xxx"), vec!["-webkit-xxx"]);
        assert_eq!(tokenize(b"--var"), vec!["--var"]);

        assert_eq!(
            tokenize(b"parent .btn { /**/ padding: 10px }"),
            vec!["parent", " ", ".", "btn", "{", "padding", ":", "10", "px", "}"]
        );

        assert_eq!(
            tokenize(b"@media { a b { left: 10% } }"),
            vec!["@", "media", "{", "a", " ", "b", "{", "left", ":", "10", "%", "}", "}"]
        );

        assert_eq!(tokenize(b"/**/ a /**/ b {}"), vec!["a", " ", "b", "{", "}"]);

        let ua = include_bytes!("../../resources/ua.css");
        let _tokens = tokenize(ua);

        // println!("{:#?}", _tokens);
    }

    #[test]
    fn basic() -> Result<(), ParseError> {
        let style = CssStyleDeclaration::parse("color: #fff")?;
        assert_eq!(style, CssStyleDeclaration::new(vec![StyleProp::Color(CssColor::WHITE)]));

        let selector = Selector::parse("div")?;
        assert_eq!(
            selector,
            Selector {
                parts: vec![SelectorPart::Component(Component::LocalName(Atom::from("div")))]
            }
        );

        let sheet = CssStyleSheet::parse("div { color: #fff }")?;

        assert_eq!(sheet.rules()[0], CssStyleRule::new(Selector::parse("div")?, style));
        assert_eq!(sheet.rules()[0].style().to_string(), "color:rgba(255, 255, 255, 255);");

        // white-space
        assert_eq!(CssStyleSheet::parse(" *{}")?.rules().len(), 1);
        assert_eq!(CssStyleSheet::parse("\n*{\n}\n")?.rules().len(), 1);

        // forgiving/future-compatibility
        assert_eq!(CssStyleSheet::parse(":root {} a { v: 0 }")?.rules().len(), 2);
        assert_eq!(
            CssStyleSheet::parse("a {} @media { a { v: 0 } } b {}")?.rules().len(),
            2
        );
        assert_eq!(
            CssStyleSheet::parse("@media { a { v: 0 } } a {} b {}")?.rules().len(),
            2
        );

        Ok(())
    }

    #[test]
    fn shorthands() -> Result<(), ParseError> {
        use StyleProp::*;

        let assert_eq_props = |style: &CssStyleDeclaration, props: &[StyleProp]| Iterator::eq(style.props(), props);

        assert_eq_props(
            &CssStyleDeclaration::parse("overflow: hidden")?,
            &[OverflowX(CssOverflow::Hidden), OverflowY(CssOverflow::Hidden)],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("overflow: visible hidden")?,
            &[OverflowX(CssOverflow::Visible), OverflowY(CssOverflow::Hidden)],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("flex: 1")?,
            &[FlexGrow(1.), FlexShrink(1.), FlexBasis(CssDimension::ZERO)],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("flex: 2 3 10px")?,
            &[FlexGrow(2.), FlexShrink(3.), FlexBasis(CssDimension::Px(10.))],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("padding: 0")?,
            &[
                PaddingTop(CssDimension::ZERO),
                PaddingRight(CssDimension::ZERO),
                PaddingBottom(CssDimension::ZERO),
                PaddingLeft(CssDimension::ZERO),
            ],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("padding: 10px 20px")?,
            &[
                PaddingTop(CssDimension::Px(10.)),
                PaddingRight(CssDimension::Px(20.)),
                PaddingBottom(CssDimension::Px(10.)),
                PaddingLeft(CssDimension::Px(20.)),
            ],
        );

        assert_eq_props(
            &CssStyleDeclaration::parse("background: none")?,
            &[StyleProp::BackgroundColor(CssColor::TRANSPARENT)],
        );
        assert_eq_props(
            &CssStyleDeclaration::parse("background: #000")?,
            &[StyleProp::BackgroundColor(CssColor::BLACK)],
        );

        // override
        let mut s = CssStyleDeclaration::parse("background-color: #fff")?;
        s.set_property("background", "#000");
        assert_eq_props(&s, &[StyleProp::BackgroundColor(CssColor::BLACK)]);

        // override with shorthand
        let mut s = CssStyleDeclaration::parse("background-color: #fff")?;
        s.set_property("background", "none");
        assert_eq_props(&s, &[StyleProp::BackgroundColor(CssColor::TRANSPARENT)]);

        Ok(())
    }

    #[test]
    fn parse_ua() {
        let ua = include_str!("../../resources/ua.css");
        let tokens = tokenize(ua.as_bytes());
        let sheet = super::sheet().parse(&tokens).unwrap();

        assert_eq!(sheet.rules().len(), 23);
    }

    #[test]
    fn parse_selector() {
        use super::Combinator::*;
        use super::Component::*;
        use SelectorPart::{Combinator, Component};

        let s = |s| Selector::parse(s).unwrap().parts;

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

        // unsupported for now
        assert_eq!(s(":root"), &[Component(Unsupported)]);
        assert_eq!(
            s("* + *"),
            &[Combinator(Universal), Component(Unsupported), Combinator(Universal)]
        );
        assert_eq!(
            s("* ~ *"),
            &[Combinator(Universal), Component(Unsupported), Combinator(Universal)]
        );

        // invalid
        assert!(Selector::parse("").is_err());
        assert!(Selector::parse(" ").is_err());
        assert!(Selector::parse("a,,b").is_err());
        assert!(Selector::parse("a>>b").is_err());

        // bugs & edge-cases
        assert_eq!(
            s("input[type=\"submit\"]"),
            &[Component(Unsupported), Component(LocalName("input".into()))]
        );
    }

    #[test]
    fn parse_prop() {
        assert_eq!(
            prop_parser("padding-left").parse(&["10", "px"]),
            Ok(StyleProp::PaddingLeft(CssDimension::Px(10.)))
        );
        assert_eq!(
            prop_parser("margin-top").parse(&["5", "%"]),
            Ok(StyleProp::MarginTop(CssDimension::Percent(5.)))
        );
        assert_eq!(prop_parser("opacity").parse(&["1"]), Ok(StyleProp::Opacity(1.)));
        assert_eq!(
            prop_parser("color").parse(&["#", "000000"]),
            Ok(StyleProp::Color(CssColor::BLACK))
        );
    }

    #[test]
    fn parse_align() {
        assert_eq!(css_enum().parse(&["auto"]), Ok(CssAlign::Auto));
        //assert_eq!(css_enum().parse(&["start"]), Ok(CssAlign::Start));
        assert_eq!(css_enum().parse(&["flex-start"]), Ok(CssAlign::FlexStart));
        assert_eq!(css_enum().parse(&["center"]), Ok(CssAlign::Center));
        //assert_eq!(css_enum().parse(&["end"]), Ok(CssAlign::End));
        assert_eq!(css_enum().parse(&["flex-end"]), Ok(CssAlign::FlexEnd));
        assert_eq!(css_enum().parse(&["stretch"]), Ok(CssAlign::Stretch));
        assert_eq!(css_enum().parse(&["baseline"]), Ok(CssAlign::Baseline));
        assert_eq!(css_enum().parse(&["space-between"]), Ok(CssAlign::SpaceBetween));
        assert_eq!(css_enum().parse(&["space-around"]), Ok(CssAlign::SpaceAround));
        //assert_eq!(css_enum().parse(&["space-evenly"]), Ok(CssAlign::SpaceEvenly));
    }

    #[test]
    fn parse_justify() {
        //assert_eq!(css_enum().parse(&["start"]), Ok(CssJustify::Start));
        assert_eq!(css_enum().parse(&["flex-start"]), Ok(CssJustify::FlexStart));
        assert_eq!(css_enum().parse(&["center"]), Ok(CssJustify::Center));
        //assert_eq!(css_enum().parse(&["end"]), Ok(CssJustify::End));
        assert_eq!(css_enum().parse(&["flex-end"]), Ok(CssJustify::FlexEnd));
        assert_eq!(css_enum().parse(&["space-between"]), Ok(CssJustify::SpaceBetween));
        assert_eq!(css_enum().parse(&["space-around"]), Ok(CssJustify::SpaceAround));
        assert_eq!(css_enum().parse(&["space-evenly"]), Ok(CssJustify::SpaceEvenly));
    }

    #[test]
    fn parse_dimension() {
        assert_eq!(dimension().parse(&["auto"]), Ok(CssDimension::Auto));
        assert_eq!(dimension().parse(&["10", "px"]), Ok(CssDimension::Px(10.)));
        assert_eq!(dimension().parse(&["100", "%"]), Ok(CssDimension::Percent(100.)));
        assert_eq!(dimension().parse(&["1.2", "em"]), Ok(CssDimension::Em(1.2)));
        assert_eq!(dimension().parse(&["2.1", "rem"]), Ok(CssDimension::Rem(2.1)));
        assert_eq!(dimension().parse(&["0"]), Ok(CssDimension::Px(0.)));
        assert_eq!(dimension().parse(&["100", "vw"]), Ok(CssDimension::Vw(100.)));
        assert_eq!(dimension().parse(&["100", "vh"]), Ok(CssDimension::Vh(100.)));
        assert_eq!(dimension().parse(&["vmin"]), Ok(CssDimension::Vmin));
        assert_eq!(dimension().parse(&["vmax"]), Ok(CssDimension::Vmax));
    }

    #[test]
    fn parse_color() {
        assert_eq!(color().parse(&["#", "000000"]), Ok(CssColor::BLACK));
        assert_eq!(color().parse(&["#", "ff0000"]), Ok(CssColor::RED));
        assert_eq!(color().parse(&["#", "00ff00"]), Ok(CssColor::GREEN));
        assert_eq!(color().parse(&["#", "0000ff"]), Ok(CssColor::BLUE));

        assert_eq!(
            color().parse(&["#", "80808080"]),
            Ok(CssColor::rgba(128, 128, 128, 128))
        );
        assert_eq!(color().parse(&["#", "00000080"]), Ok(CssColor::rgba(0, 0, 0, 128)));

        assert_eq!(color().parse(&["#", "000"]), Ok(CssColor::BLACK));
        assert_eq!(color().parse(&["#", "f00"]), Ok(CssColor::RED));
        assert_eq!(color().parse(&["#", "fff"]), Ok(CssColor::WHITE));

        assert_eq!(color().parse(&["#", "0000"]), Ok(CssColor::TRANSPARENT));
        assert_eq!(color().parse(&["#", "f00f"]), Ok(CssColor::RED));

        let toks = tokenize(b"rgb(0, 0, 0)");
        assert_eq!(color().parse(&toks), Ok(CssColor::BLACK));

        let toks = tokenize(b"rgba(0, 0, 0, 0)");
        assert_eq!(color().parse(&toks), Ok(CssColor::TRANSPARENT));

        assert_eq!(color().parse(&["transparent"]), Ok(CssColor::TRANSPARENT));
        assert_eq!(color().parse(&["black"]), Ok(CssColor::BLACK));
    }

    #[test]
    fn parse_border_style() {
        assert_eq!(css_enum().parse(&["none"]), Ok(CssBorderStyle::None));
        assert_eq!(css_enum().parse(&["hidden"]), Ok(CssBorderStyle::Hidden));
        assert_eq!(css_enum().parse(&["dotted"]), Ok(CssBorderStyle::Dotted));
        assert_eq!(css_enum().parse(&["dashed"]), Ok(CssBorderStyle::Dashed));
        assert_eq!(css_enum().parse(&["solid"]), Ok(CssBorderStyle::Solid));
        assert_eq!(css_enum().parse(&["double"]), Ok(CssBorderStyle::Double));
        assert_eq!(css_enum().parse(&["groove"]), Ok(CssBorderStyle::Groove));
        assert_eq!(css_enum().parse(&["ridge"]), Ok(CssBorderStyle::Ridge));
        assert_eq!(css_enum().parse(&["inset"]), Ok(CssBorderStyle::Inset));
        assert_eq!(css_enum().parse(&["outset"]), Ok(CssBorderStyle::Outset));
    }

    #[test]
    fn parse_display() {
        assert_eq!(css_enum().parse(&["none"]), Ok(CssDisplay::None));
        assert_eq!(css_enum().parse(&["block"]), Ok(CssDisplay::Block));
        assert_eq!(css_enum().parse(&["inline"]), Ok(CssDisplay::Inline));
        assert_eq!(css_enum().parse(&["inline-block"]), Ok(CssDisplay::InlineBlock));
        assert_eq!(css_enum().parse(&["flex"]), Ok(CssDisplay::Flex));
        assert_eq!(css_enum().parse(&["table"]), Ok(CssDisplay::Table));
        assert_eq!(
            css_enum().parse(&["table-header-group"]),
            Ok(CssDisplay::TableHeaderGroup)
        );
        assert_eq!(css_enum().parse(&["table-row-group"]), Ok(CssDisplay::TableRowGroup));
        assert_eq!(css_enum().parse(&["table-row"]), Ok(CssDisplay::TableRow));
        assert_eq!(css_enum().parse(&["table-cell"]), Ok(CssDisplay::TableCell));
    }

    #[test]
    fn parse_flex_direction() {
        assert_eq!(css_enum().parse(&["row"]), Ok(CssFlexDirection::Row));
        assert_eq!(css_enum().parse(&["column"]), Ok(CssFlexDirection::Column));
        assert_eq!(css_enum().parse(&["row-reverse"]), Ok(CssFlexDirection::RowReverse));
        assert_eq!(
            css_enum().parse(&["column-reverse"]),
            Ok(CssFlexDirection::ColumnReverse)
        );
    }

    #[test]
    fn parse_flex_wrap() {
        assert_eq!(css_enum().parse(&["nowrap"]), Ok(CssFlexWrap::NoWrap));
        assert_eq!(css_enum().parse(&["wrap"]), Ok(CssFlexWrap::Wrap));
        assert_eq!(css_enum().parse(&["wrap-reverse"]), Ok(CssFlexWrap::WrapReverse));
    }

    #[test]
    fn parse_overflow() {
        assert_eq!(css_enum().parse(&["visible"]), Ok(CssOverflow::Visible));
        assert_eq!(css_enum().parse(&["hidden"]), Ok(CssOverflow::Hidden));
        assert_eq!(css_enum().parse(&["scroll"]), Ok(CssOverflow::Scroll));
        assert_eq!(css_enum().parse(&["auto"]), Ok(CssOverflow::Auto));
    }

    #[test]
    fn parse_position() {
        assert_eq!(css_enum().parse(&["static"]), Ok(CssPosition::Static));
        assert_eq!(css_enum().parse(&["relative"]), Ok(CssPosition::Relative));
        assert_eq!(css_enum().parse(&["absolute"]), Ok(CssPosition::Absolute));
        assert_eq!(css_enum().parse(&["sticky"]), Ok(CssPosition::Sticky));
    }

    #[test]
    fn parse_text_align() {
        assert_eq!(css_enum().parse(&["left"]), Ok(CssTextAlign::Left));
        assert_eq!(css_enum().parse(&["center"]), Ok(CssTextAlign::Center));
        assert_eq!(css_enum().parse(&["right"]), Ok(CssTextAlign::Right));
        assert_eq!(css_enum().parse(&["justify"]), Ok(CssTextAlign::Justify));
    }

    #[test]
    fn parse_visibility() {
        assert_eq!(css_enum().parse(&["visible"]), Ok(CssVisibility::Visible));
        assert_eq!(css_enum().parse(&["hidden"]), Ok(CssVisibility::Hidden));
        assert_eq!(css_enum().parse(&["collapse"]), Ok(CssVisibility::Collapse));
    }
}
