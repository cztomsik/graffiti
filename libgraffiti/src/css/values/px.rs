// some props are currently limited to pixels

use super::super::parsing::{sym, Parsable, Parser};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Px(pub f32);

impl fmt::Display for Px {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

impl Parsable for Px {
    fn parser<'a>() -> Parser<'a, Self> {
        let zero = sym("0").map(|_| Self(0.));
        let px = f32::parser().map(Self) - sym("px");

        px | zero
    }
}
