// subset of CSS selectors for CSS-in-JS

use super::parser::{selector, tokenize, ParseError};
use crate::util::Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub(super) parts: Vec<SelectorPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum SelectorPart {
    Universal,
    LocalName(Atom),
    Identifier(Atom),
    ClassName(Atom),
    AttrExists(Atom),
    AttrEq(Atom, Atom),
    AttrStartsWith(Atom, Atom),
    AttrEndsWith(Atom, Atom),
    AttrContains(Atom, Atom),

    Combinator(Combinator),

    // FirstChild // (prev_element_sibling == None)
    // LastChild // (next_element_sibling == None)
    // OnlyChild // (prev_element_sibling == None && next_element_sibling == None)

    // BTW: many are just compound shorthands and can be resolved here (:disabled is like [disabled] & input, select, ...)
    // PseudoClass(Atom) // :root, :hover, :focus, :active, :enabled, :disabled, :valid, :invalid, ...
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Combinator {
    Parent,
    Ancestor,
    Or,
    // Adjacent,
    // Sibling,
}

impl Selector {
    pub(super) fn from_parts(parts: Vec<SelectorPart>) -> Self {
        Self { parts }
    }

    pub fn unsupported() -> Self {
        Self::from_parts(vec![SelectorPart::Unsupported])
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
