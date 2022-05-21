use super::super::parsing::{fail, Parsable, Parser};
use super::Color;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: Color,
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}px {}px {}px {}px {}",
            self.offset.0, self.offset.1, self.blur, self.spread, self.color
        )
    }
}

impl Parsable for BoxShadow {
    fn parser<'a>() -> Parser<'a, Self> {
        fail("TODO: parse box-shadow")
    }
}
