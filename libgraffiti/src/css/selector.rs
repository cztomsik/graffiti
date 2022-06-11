// subset of CSS selectors for CSS-in-JS

use super::parsing::{ident, skip, sym, Parsable, ParseError, Parser};
use crate::util::Atom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Specificity(u32);

// SoA-friendly, to be implemented by client
pub trait MatchingContext: Sized {
    type ElementRef: Copy;

    fn parent_element(&self, element: Self::ElementRef) -> Option<Self::ElementRef>;
    fn local_name(&self, element: Self::ElementRef) -> &str;
    fn attribute(&self, element: Self::ElementRef, attribute: &str) -> Option<&str>;

    // TODO: fast-path has_* methods (with default impls)
    //       or maybe introduce type LocalName: PartialEq<Atom>? and make the whole trait parametrized?
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub(super) parts: Vec<SelectorPart>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum SelectorPart {
    Universal,
    LocalName(Atom),
    Identifier(Atom),
    ClassName(Atom),
    AttrExists(Atom),
    // AttrEq(Atom, Atom),
    // AttrStartsWith(Atom, Atom),
    // AttrEndsWith(Atom, Atom),
    // AttrContains(Atom, Atom),
    Combinator(Combinator),

    // FirstChild // (prev_element_sibling == None)
    // LastChild // (next_element_sibling == None)
    // OnlyChild // (prev_element_sibling == None && next_element_sibling == None)

    // BTW: many are just compound shorthands and can be resolved here (:disabled is like [disabled] & input, select, ...)
    // PseudoClass(Atom) // :root, :hover, :focus, :active, :enabled, :disabled, :valid, :invalid, ...
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum Combinator {
    Parent,
    Ancestor,
    Or,
    // Adjacent,
    // Sibling,
}

impl Selector {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Parsable::parse(input)
    }

    pub(crate) fn unsupported() -> Self {
        Self {
            parts: vec![SelectorPart::Unsupported],
        }
    }

    pub fn match_element<C: MatchingContext>(&self, element: C::ElementRef, ctx: &C) -> Option<Specificity> {
        // so we can fast-forward to next OR
        let mut parts_iter = self.parts.iter();

        // state
        let mut current = element;
        let mut parent = false;
        let mut ancestors = false;
        let specificity = Specificity(0);

        // we are always going forward
        'next_part: while let Some(p) = parts_iter.next() {
            match p {
                SelectorPart::Combinator(comb) => {
                    match comb {
                        // state changes
                        Combinator::Parent => parent = true,
                        Combinator::Ancestor => ancestors = true,

                        // end-of-branch and we still have a match, no need to check others
                        Combinator::Or => break 'next_part,
                    }
                }

                comp => {
                    loop {
                        if parent || ancestors {
                            parent = false;

                            match ctx.parent_element(current) {
                                Some(parent) => current = parent,

                                // nothing left to match
                                None => break,
                            }
                        }

                        if Self::match_component(current, comp, ctx) {
                            ancestors = false;
                            continue 'next_part;
                        }

                        // we got no match on parent
                        if !ancestors {
                            break;
                        }
                    }

                    // no match, fast-forward to next OR
                    for p in parts_iter.by_ref() {
                        if p == &SelectorPart::Combinator(Combinator::Or) {
                            // reset stack
                            current = element;
                            continue 'next_part;
                        }
                    }

                    // or fail otherwise
                    return None;
                }
            }
        }

        // everything was fine
        Some(specificity)
    }

    fn match_component<C: MatchingContext>(el: C::ElementRef, comp: &SelectorPart, ctx: &C) -> bool {
        match comp {
            SelectorPart::Universal => true,
            SelectorPart::LocalName(name) => ctx.local_name(el) == &**name,
            SelectorPart::Identifier(id) => ctx.attribute(el, "id") == Some(id),
            SelectorPart::ClassName(cls) => match ctx.attribute(el, "class") {
                Some(s) => s.split_ascii_whitespace().any(|part| part == &**cls),
                _ => false,
            },
            SelectorPart::AttrExists(att) => ctx.attribute(el, att).is_some(),
            // SelectorPart::AttrEq(att, val) => ctx.attribute(el, att) == Some(val),
            // SelectorPart::AttrStartsWith(att, s) => ctx.attribute(el, att).map_or(false, |v| v.starts_with(&**s)),
            // SelectorPart::AttrEndsWith(att, s) => ctx.attribute(el, att).map_or(false, |v| v.ends_with(&**s)),
            // SelectorPart::AttrContains(att, s) => ctx.attribute(el, att).map_or(false, |v| v.contains(&**s)),
            SelectorPart::Unsupported => false,
            SelectorPart::Combinator(_) => unreachable!(),
        }
    }
}

impl Parsable for Selector {
    fn parser<'a>() -> Parser<'a, Self> {
        let tag = || {
            let ident = || ident().map(Atom::from);
            let universal = sym("*").map(|_| SelectorPart::Universal);
            let local_name = ident().map(SelectorPart::LocalName);
            let id = sym("#") * ident().map(SelectorPart::Identifier);
            let class_name = sym(".") * ident().map(SelectorPart::ClassName);
            let attr_exists = sym("[") * ident().map(SelectorPart::AttrExists) - sym("]");
            let unknown_attr =
                sym("[") * (!sym("]") * skip(1)).repeat(1..).map(|_| SelectorPart::Unsupported) - sym("]");
            let attr = attr_exists | unknown_attr;
            let pseudo = sym(":").discard().repeat(1..3) * ident().map(|_| SelectorPart::Unsupported);

            universal | local_name | id | class_name | attr | pseudo
        };

        // note we parse child/descendant but we flip the final order so it's parent/ancestor
        let child = sym(">").map(|_| Combinator::Parent);
        let descendant = sym(" ").map(|_| Combinator::Ancestor);
        let or = sym(",").map(|_| Combinator::Or);
        let unsupported = (sym("+") | sym("~")).map(|_| SelectorPart::Unsupported);
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
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.parts.iter().rev() {
            write!(f, "{}", r)?;
        }

        Ok(())
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Universal => write!(f, "*"),
            Self::LocalName(name) => write!(f, "{}", name),
            Self::Identifier(id) => write!(f, "#{}", id),
            Self::ClassName(clz) => write!(f, ".{}", clz),
            Self::AttrExists(att) => write!(f, "[{}]", att),
            Self::Combinator(comb) => write!(f, "{}", comb),
            Self::Unsupported => write!(f, "???"),
        }
    }
}

impl fmt::Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Parent => " ",
            Self::Ancestor => ">",
            Self::Or => ",",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_selector() {
        use super::Combinator::*;
        use SelectorPart::*;

        let s = |s| Selector::parse(s).unwrap().parts;

        // simple
        assert_eq!(s("*"), &[Universal]);
        assert_eq!(s("body"), &[LocalName("body".into())]);
        assert_eq!(s("h2"), &[LocalName("h2".into())]);
        assert_eq!(s("#app"), &[Identifier("app".into())]);
        assert_eq!(s(".btn"), &[ClassName("btn".into())]);

        // attrs
        assert_eq!(s(r"[href]"), &[AttrExists("href".into())]);
        // assert_eq!(s(r#"[href="foo"]"#), &[AttrEq("href".into(), "foo".into())]);
        // assert_eq!(s(r#"[href^="http"]"#), &[AttrStartsWith("href".into(), "http".into())]);
        // assert_eq!(s(r#"[href$=".org"]"#), &[AttrEndsWith("href".into(), ".org".into())]);
        // assert_eq!(s(r#"[href*="foo"]"#), &[AttrContains("href".into(), "foo".into())]);

        // combined
        assert_eq!(
            s(".btn.btn-primary"),
            &[ClassName("btn-primary".into()), ClassName("btn".into())]
        );
        assert_eq!(s("*.test"), &[ClassName("test".into()), Universal]);
        assert_eq!(
            s("div#app.test"),
            &[
                ClassName("test".into()),
                Identifier("app".into()),
                LocalName("div".into())
            ]
        );

        // combined with combinators
        assert_eq!(
            s("body > div.test div#test"),
            &[
                Identifier("test".into()),
                LocalName("div".into()),
                Combinator(Ancestor),
                ClassName("test".into()),
                LocalName("div".into()),
                Combinator(Parent),
                LocalName("body".into())
            ]
        );

        // multi
        assert_eq!(
            s("html, body"),
            &[LocalName("body".into()), Combinator(Or), LocalName("html".into())]
        );
        assert_eq!(
            s("body > div, div button span"),
            &[
                LocalName("span".into()),
                Combinator(Ancestor),
                LocalName("button".into()),
                Combinator(Ancestor),
                LocalName("div".into()),
                Combinator(Or),
                LocalName("div".into()),
                Combinator(Parent),
                LocalName("body".into()),
            ]
        );

        // unsupported for now
        assert_eq!(s(":root"), &[Unsupported]);
        assert_eq!(s("* + *"), &[Universal, Unsupported, Universal]);
        assert_eq!(s("* ~ *"), &[Universal, Unsupported, Universal]);

        // invalid
        assert!(Selector::parse("").is_err());
        assert!(Selector::parse(" ").is_err());
        assert!(Selector::parse("a,,b").is_err());
        assert!(Selector::parse("a>>b").is_err());

        // bugs & edge-cases
        assert_eq!(s("input[type=\"submit\"]"), &[Unsupported, LocalName("input".into())]);
    }

    #[test]
    fn matching() {
        struct Ctx;

        impl MatchingContext for Ctx {
            type ElementRef = usize;

            fn parent_element(&self, el: usize) -> Option<usize> {
                [None, Some(0), Some(1), Some(2), Some(3)][el]
            }

            fn local_name(&self, el: usize) -> &str {
                ["html", "body", "div", "button", "span"][el]
            }

            fn attribute(&self, el: usize, attr: &str) -> Option<&str> {
                let v = match attr {
                    "id" => ["", "app", "panel", "", ""][el],
                    "class" => ["", "", "", "btn", ""][el],
                    _ => "",
                };

                match v {
                    "" => None,
                    v => Some(v),
                }
            }
        }

        let match_sel = |s, el| Selector::parse(s).unwrap().match_element(el, &Ctx).is_some();

        // invalid
        assert!(Selector::unsupported().match_element(0, &Ctx).is_none());

        // basic
        assert!(match_sel("*", 0));
        assert!(match_sel("html", 0));
        assert!(match_sel("body", 1));
        assert!(match_sel("#app", 1));
        assert!(match_sel("div", 2));
        assert!(match_sel("#panel", 2));
        assert!(match_sel("button", 3));
        assert!(match_sel(".btn", 3));
        assert!(match_sel("span", 4));

        // combined
        assert!(match_sel("body#app", 1));
        assert!(match_sel("div#panel", 2));
        assert!(match_sel("button.btn", 3));

        // parent
        assert!(match_sel("button > span", 4));
        assert!(match_sel("div#panel > button.btn > span", 4));

        // ancestor
        assert!(match_sel("button span", 4));
        assert!(match_sel("div#panel span", 4));
        assert!(match_sel("body div .btn span", 4));

        // OR
        assert!(match_sel("div, span", 4));
        assert!(match_sel("a, b, c, span, d", 4));
        assert!(match_sel("html, body", 1));

        // complex
        assert!(match_sel("div, span.foo, #panel span", 4));
        assert!(match_sel("a b c d e f g, span", 4));
    }
}
