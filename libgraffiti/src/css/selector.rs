// subset of CSS selectors
// x to support CSS-in-JS libs
// - specificity (TODO, u32-only)
// x no first/last/nth/siblings
// x universal
// x local name
// x id
// x class
// x child
// x descendant
// x multiple (div, span)
// x combination
// x decoupled from other systems

use super::parser::{selector, tokenize, ParseError};
use crate::util::{Atom, Bloom};

// TODO: find better name? CssElement? MatchedElement?
pub trait Element: Clone {
    fn parent(&self) -> Option<Self>;
    fn local_name(&self) -> Atom<String>;
    fn attribute(&self, name: &str) -> Option<Atom<String>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    // TODO: Bloom<TailPart>
    // TODO: Box<[]> because it wont change once parsed
    pub(super) parts: Vec<SelectorPart>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum SelectorPart {
    // TODO: I think inner discriminant could be squashed but it's not
    //       maybe part.is_component() + inline these?
    Component(Component),
    Combinator(Combinator),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Component {
    LocalName(Atom<String>),
    Identifier(Atom<String>),
    ClassName(Atom<String>),

    Unsupported,
    // AttrExists(Atom<String>),
    // AttrEq(Atom<(Atom<String>, Atom<String>)>) // deref first, then compare both atoms
    // FirstChild // (prev_element_sibling == None)
    // LastChild // (next_element_sibling == None)
    // OnlyChild // (prev_element_sibling == None && next_element_sibling == None)

    // BTW: many are just compound shorthands and can be resolved here (:disabled is like [disabled] & input, select, ...)
    // PseudoClass(Atom<String>) // :root, :hover, :focus, :active, :enabled, :disabled, :valid, :invalid, ...
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Combinator {
    Universal,
    Parent,
    Ancestor,
    // Adjacent,
    // Sibling,
    Or,
}

type Specificity = u32;

impl Selector {
    pub fn unsupported() -> Self {
        Self {
            parts: vec![SelectorPart::Component(Component::Unsupported)],
        }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = selector() - pom::parser::end();

        parser.parse(&tokens)
    }

    pub(super) fn tail_mask(&self) -> Bloom<()> {
        Bloom::MAX
    }

    pub fn match_element(&self, element: &impl Element) -> Option<Specificity> {
        // so we can fast-forward to next OR
        let mut parts_iter = self.parts.iter();

        // state
        let mut current = element.clone();
        let mut parent = false;
        let mut ancestors = false;
        let specificity = 0;

        // we are always going forward
        'next_part: while let Some(p) = parts_iter.next() {
            match p {
                SelectorPart::Component(comp) => {
                    loop {
                        if parent || ancestors {
                            parent = false;

                            match current.parent() {
                                Some(parent) => current = parent,

                                // nothing left to match
                                None => break,
                            }
                        }

                        if Self::match_component(comp, &current) {
                            ancestors = false;
                            continue 'next_part;
                        }

                        // we got no match on parent
                        if !ancestors {
                            break;
                        }
                    }

                    // no match, fast-forward to next OR
                    while let Some(p) = parts_iter.next() {
                        if p == &SelectorPart::Combinator(Combinator::Or) {
                            // reset stack
                            current = element.clone();
                            continue 'next_part;
                        }
                    }

                    // or fail otherwise
                    return None;
                }

                // state changes
                SelectorPart::Combinator(Combinator::Parent) => parent = true,
                SelectorPart::Combinator(Combinator::Ancestor) => ancestors = true,

                // no-op
                SelectorPart::Combinator(Combinator::Universal) => {}

                // we still have a match, no need to check others
                SelectorPart::Combinator(Combinator::Or) => break 'next_part,
            }
        }

        // everything was fine
        Some(specificity)
    }

    fn match_component(comp: &Component, el: &impl Element) -> bool {
        match comp {
            Component::LocalName(name) => name == &el.local_name(),
            Component::Identifier(id) => Some(id) == el.attribute("id").as_ref(),
            Component::ClassName(cls) => match el.attribute("class") {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                _ => false,
            },
            Component::Unsupported => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn part_size() {
        use std::mem::size_of;

        // TODO: either find a way or inline components in SelectorPart
        // TODO: make Atom NonZeroU32 to further push this down
        assert_eq!(size_of::<SelectorPart>(), 2 * size_of::<Atom<String>>());
    }

    #[test]
    fn matching() {
        impl Element for usize {
            fn parent(&self) -> Option<Self> {
                [None, Some(0), Some(1), Some(2), Some(3)][*self]
            }

            fn local_name(&self) -> Atom<String> {
                ["html", "body", "div", "button", "span"][*self].into()
            }

            fn attribute(&self, name: &str) -> Option<Atom<String>> {
                let v = match name {
                    "id" => ["", "app", "panel", "", ""][*self],
                    "class" => ["", "", "", "btn", ""][*self],
                    _ => "",
                };

                match v {
                    "" => None,
                    v => Some(Atom::from(v)),
                }
            }
        }

        // let local_names = &vec!["html", "body", "div", "button", "span"];
        // let ids = &vec!["", "app", "panel", "", ""];
        // let class_names = &vec!["", "", "", "btn", ""];
        // let parents = &vec![None, Some(0), Some(1), Some(2), Some(3)];

        let match_sel = |s, el| Selector::parse(s).unwrap().match_element(&el).is_some();

        // invalid
        assert!(Selector::unsupported().match_element(&0).is_none());

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
