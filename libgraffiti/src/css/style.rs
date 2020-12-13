// TODO:
// - shorthands
// - normalize (bold -> 700)

use super::{Dimension, StyleProp, Value};
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub(super) props: Vec<StyleProp>,
}

impl Style {
    pub const EMPTY: Self = Self::new();

    pub const fn new() -> Self {
        Self { props: Vec::new() }
    }

    pub fn props(&self) -> impl Iterator<Item = &StyleProp> + '_ {
        self.props.iter()
    }

    pub fn add_prop(&mut self, new_prop: StyleProp) {
        let d = discriminant(&new_prop);

        if let Some(existing) = self.props.iter_mut().find(|p| d == discriminant(p)) {
            *existing = new_prop;
        } else {
            self.props.push(new_prop);
        }
    }
}

impl From<&str> for Style {
    fn from(style: &str) -> Style {
        super::parser::style().parse(style.as_bytes()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedStyle {
    pub width: Dim,
    pub height: Dim,
    pub min_width: Dim,
    pub min_height: Dim,
    pub max_width: Dim,
    pub max_height: Dim,

    pub padding: [Dim; 4],
    pub margin: [Dim; 4],

    pub top: Dim,
    pub right: Dim,
    pub bottom: Dim,
    pub left: Dim,
}

impl ResolvedStyle {
    pub const INITIAL: ResolvedStyle = ResolvedStyle {
        width: Dim::Auto,
        height: Dim::Auto,
        min_width: Dim::Auto,
        min_height: Dim::Auto,
        max_width: Dim::Px(f32::INFINITY),  // none
        max_height: Dim::Px(f32::INFINITY), // none

        padding: [Dim::ZERO, Dim::ZERO, Dim::ZERO, Dim::ZERO],
        margin: [Dim::ZERO, Dim::ZERO, Dim::ZERO, Dim::ZERO],

        top: Dim::Auto,
        right: Dim::Auto,
        bottom: Dim::Auto,
        left: Dim::Auto,
    };
}

// private shorthand
type Dim = Dimension;

#[cfg(test)]
mod tests {
    use super::super::Display;
    use super::*;

    #[test]
    fn prop_overriding() {
        let mut s = Style::new();

        s.add_prop(StyleProp::Display(Value::Specified(Display::None)));
        s.add_prop(StyleProp::Display(Value::Specified(Display::Block)));

        assert!(Iterator::eq(
            s.props(),
            &vec![StyleProp::Display(Value::Specified(Display::Block))]
        ));
    }
}
