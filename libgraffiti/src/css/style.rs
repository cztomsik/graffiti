// TODO:
// - shorthands
// - normalize (bold -> 700)

use super::StyleProp;
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

// TODO: I thought it would be useful to keep resolved styles but
//       it looks like changes are not that frequent and we can
//       apply props immediately to layout, etc.
//
//       if anything, it will be array of refs to styles or something like that
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedStyle {}

impl ResolvedStyle {
    pub const INITIAL: Self = Self {};
}

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
