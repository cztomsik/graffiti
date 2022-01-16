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
use crate::util::Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub(super) parts: Vec<SelectorPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum SelectorPart {
    // TODO: I think inner discriminant could be squashed but it's not
    //       maybe part.is_component() + inline these?
    Component(Component),
    Combinator(Combinator),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Component {
    LocalName(Atom),
    Identifier(Atom),
    ClassName(Atom),

    Unsupported,
    // AttrExists(Atom),
    // AttrEq(Atom<(Atom, Atom)>) // deref first, then compare both atoms
    // FirstChild // (prev_element_sibling == None)
    // LastChild // (next_element_sibling == None)
    // OnlyChild // (prev_element_sibling == None && next_element_sibling == None)

    // BTW: many are just compound shorthands and can be resolved here (:disabled is like [disabled] & input, select, ...)
    // PseudoClass(Atom) // :root, :hover, :focus, :active, :enabled, :disabled, :valid, :invalid, ...
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Combinator {
    Universal,
    Parent,
    Ancestor,
    // Adjacent,
    // Sibling,
    Or,
}

impl Selector {
    pub(super) fn from_parts(parts: Vec<SelectorPart>) -> Self {
        Self { parts }
    }

    pub fn unsupported() -> Self {
        Self::from_parts(vec![SelectorPart::Component(Component::Unsupported)])
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = selector() - pom::parser::end();

        parser.parse(&tokens)
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
        assert_eq!(size_of::<SelectorPart>(), 2 * size_of::<Atom>());
    }
}
