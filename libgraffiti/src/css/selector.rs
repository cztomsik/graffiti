// subset of CSS selectors
// x to support CSS-in-JS libs
// x no specificity for now
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

use crate::util::Atom;
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selector {
    pub(super) parts: Vec<SelectorPart>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum SelectorPart {
    // TODO: I think inner discriminant could be squashed but it's not
    Component(Component),
    Combinator(Combinator),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Component {
    LocalName(Atom<String>),
    Identifier(Atom<String>),
    ClassName(Atom<String>),
    // AttrExists(Atom<String>),
    // AttrEq(Atom<(Atom<String>, Atom<String>)>) // deref first, then compare both atoms
    // FirstChild // (prev_element_sibling == None)
    // LastChild // (next_element_sibling == None)
    // OnlyChild // (prev_element_sibling == None && next_element_sibling == None)

    // PseudoClass(Atom<String>) // :root, :hover, :focus, :active, :enabled, :disabled, :valid, :invalid, ...
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Combinator {
    Universal,
    Parent,
    Ancestor,
    // Adjacent,
    // Sibling,
    Or,
}

pub(crate) struct MatchingContext<'a, E> {
    pub has_local_name: &'a dyn Fn(E, &Atom<String>) -> bool,
    pub has_identifier: &'a dyn Fn(E, &Atom<String>) -> bool,
    pub has_class: &'a dyn Fn(E, &Atom<String>) -> bool,
    //has_pseudo_class: &'a dyn Fn(E, &Atom<String>) -> bool,
    pub parent: &'a dyn Fn(E) -> Option<E>,
}

impl<E: Copy> MatchingContext<'_, E> {
    fn match_component(&self, component: &Component, el: E) -> bool {
        use Component::*;

        match component {
            LocalName(name) => (self.has_local_name)(el, name),
            Identifier(id) => (self.has_identifier)(el, id),
            ClassName(cls) => (self.has_class)(el, cls),
        }
    }

    pub fn match_selector<'a>(&'a self, selector: &Selector, el: E) -> bool {
        // so we can fast-forward to next OR
        let mut parts_iter = selector.parts.iter();

        // state
        let mut curr = el;
        let mut parent = false;
        let mut ancestors = false;

        // we are always going forward
        'next_part: while let Some(p) = parts_iter.next() {
            match p {
                SelectorPart::Component(comp) => {
                    loop {
                        if parent || ancestors {
                            parent = false;

                            match (self.parent)(curr) {
                                Some(p) => curr = p,

                                // nothing left to match
                                None => break,
                            }
                        }

                        if self.match_component(&comp, curr) {
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
                            curr = el;
                            continue 'next_part;
                        }
                    }

                    // or fail otherwise
                    return false;
                }

                // state changes
                SelectorPart::Combinator(Combinator::Parent) => parent = true,
                SelectorPart::Combinator(Combinator::Ancestor) => ancestors = true,

                // no-op
                SelectorPart::Combinator(Combinator::Universal) => {}

                // we still have a match, no need to check others
                SelectorPart::Combinator(Combinator::Or) => return true,
            }
        }

        // everything was fine
        true
    }
}

#[derive(Debug)]
pub struct InvalidSelector;

impl TryFrom<&str> for Selector {
    type Error = InvalidSelector;

    fn try_from(selector: &str) -> Result<Self, Self::Error> {
        (super::parser::selector() - pom::parser::end())
            .parse(selector.as_bytes())
            .map_err(|_| InvalidSelector)
    }
}

/*
pub struct SelectorMask(u32);

impl SelectorMask {
    const BITS: usize = core::mem::size_of::<usize>() * 8;

    pub fn includes(&self, other: SelectorMask) -> bool {
        (self.0 & other.0) != 0
    }
}

impl<'a, T: IntoIterator<Item = &'a Atom<Tag>>> From<T> for SelectorMask {
    fn from(tags: T) -> Self {
        use std::hash::{Hash, Hasher};

        // TODO: maybe it could be oneliner too (fold)
        let hash = |tag: &Tag| {
            let mut hasher = fnv::FnvHasher::default();
            tag.hash(&mut hasher);
            hasher.finish()
        };

        // TODO: test
        // TODO: zero?
        Self(
            tags.into_iter()
                .fold(0, |res, t| res | 1 << (hash(t) as usize - 1) % Self::BITS),
        )
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn part_size() {
        use std::mem::size_of;

        // TODO: either find a way or inline components in SelectorPart
        // TODO: make Atom NonZeroU32 to further push this down
        assert_eq!(size_of::<SelectorPart>(), 2 * size_of::<Atom<String>>())
    }

    #[test]
    fn matching() {
        let local_names = &vec!["html", "body", "div", "button", "span"];
        let ids = &vec!["", "app", "panel", "", ""];
        let class_names = &vec!["", "", "", "btn", ""];
        let parents = &vec![None, Some(0), Some(1), Some(2), Some(3)];

        let ctx = MatchingContext {
            has_local_name: &|e, n| **n == local_names[e],
            has_identifier: &|e, id| **id == ids[e],
            has_class: &|e, cls| **cls == class_names[e],
            parent: &|e| parents[e],
        };

        let match_sel = |s, el| ctx.match_selector(&Selector::try_from(s).unwrap(), el);

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
