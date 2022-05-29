// subset of CSS selectors for CSS-in-JS

use super::parsing::{ident, skip, sym, Parsable, ParseError, Parser};
use crate::util::Atom;
use std::fmt;

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
}
