// TODO:
// - shorthands (get)
// - normalize (bold -> 700)

use super::{CssDisplay, Selector, StyleProp};
use std::cell::{Ref, RefCell};
use std::fmt::Write;
use std::mem::discriminant;

// TODO: for !important we could be fine with bitflags and 1 << prop.id() as u32 to figure out the bit to flip/check
// TODO: notify Option<Box<dyn Fn()>>
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CssStyleDeclaration {
    pub(super) props: RefCell<Vec<StyleProp>>,
}

impl CssStyleDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

    // jsdom squashes longhands into one shorthand (if all are present)
    // but chrome doesn't so I think we don't have to either
    pub fn length(&self) -> usize {
        self.props.borrow().len()
    }

    pub fn item(&self, index: usize) -> Option<&str> {
        self.props.borrow().get(index).map(StyleProp::name)
    }

    pub fn property_value(&self, prop: &str) -> Option<String> {
        if let Some(prop) = self.props.borrow().iter().find(|p| p.name() == prop) {
            return Some(prop.value_as_string());
        }

        self.shorthand_value(prop)
    }

    // TODO: priority
    pub fn set_property(&self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        super::parser::parse_prop_into(prop, &tokens, self);
    }

    pub fn remove_property(&self, prop: &str) {
        self.props.borrow_mut().retain(|p| p.name() == prop);
    }

    pub fn css_text(&self) -> String {
        self.props().iter().fold(String::new(), |mut s, p| {
            write!(s, "{}: {};", p.name(), p.value_as_string()).unwrap();
            s
        })
    }

    pub fn set_css_text(&self, css_text: &str) {
        *self.props.borrow_mut() = Self::from(css_text).props.into_inner();
    }

    pub(crate) fn props(&self) -> Ref<[StyleProp]> {
        Ref::map(self.props.borrow(), Vec::as_slice)
    }

    pub(crate) fn add_prop(&self, new_prop: StyleProp) {
        let d = discriminant(&new_prop);
        let mut props = self.props.borrow_mut();

        if let Some(existing) = props.iter_mut().find(|p| d == discriminant(p)) {
            *existing = new_prop;
        } else {
            props.push(new_prop);
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
        let s = CssStyleDeclaration::new();

        s.set_css_text("display: block;");
        assert_eq!(&s.css_text(), "display: block;")
    }

    #[test]
    fn prop_overriding() {
        let s = CssStyleDeclaration::new();

        s.add_prop(StyleProp::Display(CssDisplay::None));
        s.add_prop(StyleProp::Display(CssDisplay::Block));

        assert!(s.props().eq(&vec![StyleProp::Display(CssDisplay::Block)]));
    }
}
