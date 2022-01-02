use super::color::CssColor;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CssBoxShadow {
    // TODO: Dimension
    offset: (f32, f32),
    blur: f32,
    spread: f32,
    color: CssColor,
}

impl fmt::Display for CssBoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}px {}px {}px {}px {}",
            self.offset.0, self.offset.1, self.blur, self.spread, self.color
        )
    }
}
