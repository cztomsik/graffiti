// TODO:
// - shorthands
// - normalize (bold -> 700)

use super::StyleProp;
use std::fmt::Write;
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub(super) props: Vec<StyleProp>,
}

impl Style {
    pub const EMPTY: Self = Self::new();
    pub const HIDDEN: Self = Self::new();

    pub const fn new() -> Self {
        Self { props: Vec::new() }
    }

    pub fn length(&self) -> usize {
        self.props.len()
    }

    pub fn item(&self, index: usize) -> Option<&str> {
        self.props.get(index).map(StyleProp::name)
    }

    pub fn property_value(&self, prop: &str) -> Option<String> {
        self.props.iter().find(|p| p.name() == prop).map(StyleProp::value)
    }

    pub fn set_property(&mut self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        if let Ok(prop) = super::parser::parse_style_prop(prop, &tokens) {
            self.add_prop(prop);
        }
    }

    // TODO: should return previous value
    pub fn remove_property(&mut self, prop: &str) {
        self.props.retain(|p| p.name() == prop);
    }

    pub fn css_text(&self) -> String {
        self.props().fold(String::new(), |mut s, p| {
            write!(s, "{}", p);
            s
        })
    }

    pub fn set_css_text(&mut self, css_text: &str) {
        *self = Self::from(css_text);
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

// never fails
impl From<&str> for Style {
    fn from(style: &str) -> Style {
        let tokens = super::parser::tokenize(style.as_bytes());
        let parser = super::parser::style();

        parser.parse(&tokens).unwrap()
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
    use super::super::*;
    use super::*;

    #[test]
    fn css_text() {
        let mut s = Style::new();

        s.set_css_text("display: block;");
        assert_eq!(&s.css_text(), "display: block;")
    }

    #[test]
    fn prop_overriding() {
        let mut s = Style::new();

        s.add_prop(StyleProp::Display(CssValue::Specified(CssDisplay::None)));
        s.add_prop(StyleProp::Display(CssValue::Specified(CssDisplay::Block)));

        assert!(Iterator::eq(
            s.props(),
            &vec![StyleProp::Display(CssValue::Specified(CssDisplay::Block))]
        ));
    }
}
