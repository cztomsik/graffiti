use super::selector::{Combinator, Component, Selector, SelectorPart};
use crate::util::Atom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Specificity(u32);

// public part, to be implemented by client
pub trait MatchingContext: Sized {
    type ElementRef: Copy;

    fn parent_element(&self, element: Self::ElementRef) -> Option<Self::ElementRef>;
    fn local_name(&self, element: Self::ElementRef) -> Atom;
    fn attribute(&self, element: Self::ElementRef, attribute: &str) -> Option<&str>;

    fn match_selector(&self, selector: &Selector, element: Self::ElementRef) -> Option<Specificity> {
        MatchingContextExt::match_parts(self, &selector.parts, element)
    }
}

// private part, only available here
trait MatchingContextExt: MatchingContext {
    fn match_parts(&self, parts: &[SelectorPart], element: Self::ElementRef) -> Option<Specificity> {
        // so we can fast-forward to next OR
        let mut parts_iter = parts.iter();

        // state
        let mut current = element;
        let mut parent = false;
        let mut ancestors = false;
        let specificity = Specificity(0);

        // we are always going forward
        'next_part: while let Some(p) = parts_iter.next() {
            match p {
                SelectorPart::Component(comp) => {
                    loop {
                        if parent || ancestors {
                            parent = false;

                            match self.parent_element(current) {
                                Some(parent) => current = parent,

                                // nothing left to match
                                None => break,
                            }
                        }

                        if self.match_component(current, comp) {
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

    fn match_component(&self, el: Self::ElementRef, comp: &Component) -> bool {
        match *comp {
            Component::LocalName(name) => name == self.local_name(el),
            Component::Identifier(id) => Some(&*id) == self.attribute(el, "id"),
            Component::ClassName(cls) => match self.attribute(el, "class") {
                Some(s) => s.split_ascii_whitespace().any(|part| part == &*cls),
                _ => false,
            },
            Component::Unsupported => false,
        }
    }
}

impl<T: MatchingContext> MatchingContextExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching() {
        struct Ctx;

        impl MatchingContext for Ctx {
            type ElementRef = usize;

            fn parent_element(&self, el: usize) -> Option<usize> {
                [None, Some(0), Some(1), Some(2), Some(3)][el]
            }

            fn local_name(&self, el: usize) -> Atom {
                ["html", "body", "div", "button", "span"][el].into()
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

        let match_sel = |s, el| Ctx.match_selector(&Selector::parse(s).unwrap(), el).is_some();

        // invalid
        assert!(Ctx.match_selector(&Selector::unsupported(), 0).is_none());

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
