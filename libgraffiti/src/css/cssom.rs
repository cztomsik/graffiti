// TODO:
// - shorthands (get)
// - normalize (bold -> 700)

use super::{CssDisplay, Selector, StyleProp};
use once_cell::sync::Lazy;
use std::fmt::Write;
use std::mem::discriminant;

#[derive(Debug, PartialEq)]
pub struct CssStyleSheet {
    pub(super) rules: Vec<CssStyleRule>,
}

impl CssStyleSheet {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn insert_rule(&mut self, rule: CssStyleRule, index: usize) {
        self.rules.insert(index, rule);
    }

    pub fn delete_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
}

// never fails
impl From<&str> for CssStyleSheet {
    fn from(sheet: &str) -> Self {
        let tokens = super::parser::tokenize(sheet.as_bytes());
        let parser = super::parser::sheet();

        parser.parse(&tokens).unwrap_or_else(|_| Self::new())
    }
}

#[derive(Debug, PartialEq)]
pub struct CssStyleRule {
    pub(crate) selector: Selector,
    style: CssStyleDeclaration,
}

impl CssStyleRule {
    pub fn new(selector: Selector, style: CssStyleDeclaration) -> Self {
        Self { selector, style }
    }

    pub fn style(&self) -> &CssStyleDeclaration {
        &self.style
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssStyleDeclaration {
    pub(super) props: Vec<StyleProp>,
}

impl CssStyleDeclaration {
    pub const EMPTY: Self = Self::new();
    pub const HIDDEN: Lazy<Self> = Lazy::new(|| Self::from("display: none"));

    pub const fn new() -> Self {
        Self { props: Vec::new() }
    }

    // jsdom squashes longhands into one shorthand (if all are present)
    // but chrome doesn't so I think we don't have to either
    pub fn length(&self) -> usize {
        self.props.len()
    }

    pub fn item(&self, index: usize) -> Option<&str> {
        self.props.get(index).map(StyleProp::name)
    }

    pub fn property_value(&self, prop: &str) -> Option<String> {
        if let Some(prop) = self.find_prop_by_name(prop) {
            return Some(prop.value_as_string());
        }

        self.shorthand_value(prop)
    }

    pub(super) fn find_prop_by_name(&self, prop: &str) -> Option<&StyleProp> {
        self.props.iter().find(|p| p.name() == prop)
    }

    // TODO: priority
    pub fn set_property(&mut self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        super::parser::parse_prop_into(prop, &tokens, self);
    }

    pub fn remove_property(&mut self, prop: &str) {
        self.props.retain(|p| p.name() == prop);
    }

    pub fn css_text(&self) -> String {
        self.props().fold(String::new(), |mut s, p| {
            write!(s, "{}: {};", p.name(), p.value_as_string()).unwrap();
            s
        })
    }

    pub fn set_css_text(&mut self, css_text: &str) {
        *self = Self::from(css_text);
    }

    pub(crate) fn props(&self) -> impl Iterator<Item = &StyleProp> + '_ {
        self.props.iter()
    }

    pub(crate) fn add_prop(&mut self, new_prop: StyleProp) {
        let d = discriminant(&new_prop);

        if let Some(existing) = self.props.iter_mut().find(|p| d == discriminant(p)) {
            *existing = new_prop;
        } else {
            self.props.push(new_prop);
        }
    }
}

// never fails
impl From<&str> for CssStyleDeclaration {
    fn from(style: &str) -> Self {
        let tokens = super::parser::tokenize(style.as_bytes());
        let parser = super::parser::style();

        parser.parse(&tokens).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn css_text() {
        let mut s = CssStyleDeclaration::new();

        s.set_css_text("display: block;");
        assert_eq!(&s.css_text(), "display: block;")
    }

    #[test]
    fn prop_overriding() {
        let mut s = CssStyleDeclaration::new();

        s.add_prop(StyleProp::Display(CssDisplay::None));
        s.add_prop(StyleProp::Display(CssDisplay::Block));

        assert!(Iterator::eq(s.props(), &vec![StyleProp::Display(CssDisplay::Block)]));
    }
}
