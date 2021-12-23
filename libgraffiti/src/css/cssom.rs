// TODO:
// - shorthands (get)
// - normalize (bold -> 700)

use super::{selector::Selector, ParseError, StyleProp};
use std::cell::{Ref, RefCell};
use std::fmt::Write;
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CssStyleSheet {
    pub(super) rules: Vec<CssStyleRule>,
}

impl CssStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_ua_sheet() -> Self {
        CssStyleSheet::parse(include_str!("../../resources/ua.css")).expect("invalid ua.css")
    }

    pub fn parse(source: &str) -> Result<Self, ParseError> {
        let tokens = super::parser::tokenize(source.as_bytes());
        let parser = super::parser::sheet() - pom::parser::end();

        parser.parse(&tokens)
    }

    pub fn rules(&self) -> &[CssStyleRule] {
        &self.rules
    }

    pub fn insert_rule(&mut self, rule: CssStyleRule, index: usize) {
        self.rules.insert(index, rule);
    }

    pub fn delete_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
}

#[derive(Debug, Clone, PartialEq)]
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

// TODO: for !important we could be fine with bitflags and 1 << prop.id() as u32 to figure out the bit to flip/check
// TODO: notify Option<Box<dyn Fn()>>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CssStyleDeclaration {
    pub(super) props: RefCell<Vec<StyleProp>>,
}

impl CssStyleDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(source: &str) -> Result<Self, ParseError> {
        let tokens = super::parser::tokenize(source.as_bytes());
        let parser = super::parser::style() - pom::parser::end();

        parser.parse(&tokens)
    }

    // jsdom squashes longhands into one shorthand (if all are present)
    // but chrome doesn't so I think we don't have to either
    pub fn length(&self) -> usize {
        self.props.borrow().len()
    }

    pub fn item(&self, index: usize) -> Option<&str> {
        self.props.borrow().get(index).map(StyleProp::css_name)
    }

    pub fn property_value(&self, prop: &str) -> Option<String> {
        if let Some(prop) = self.props.borrow().iter().find(|p| p.css_name() == prop) {
            return Some(prop.css_value());
        }

        self.shorthand_value(prop)
    }

    // TODO: priority
    pub fn set_property(&self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        super::parser::parse_prop_into(prop, &tokens, self);
    }

    pub fn remove_property(&self, prop: &str) {
        self.props.borrow_mut().retain(|p| p.css_name() == prop);
    }

    pub fn css_text(&self) -> String {
        self.props().iter().fold(String::new(), |mut s, p| {
            write!(s, "{}:{};", p.css_name(), p.css_value()).unwrap();
            s
        })
    }

    pub fn set_css_text(&self, css_text: &str) {
        let style = CssStyleDeclaration::parse(css_text).unwrap_or_default();
        *self.props.borrow_mut() = style.props.into_inner();
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

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn css_text() {
        let s = CssStyleDeclaration::new();

        s.set_css_text("display: block;");
        assert_eq!(&s.css_text(), "display:block;");
    }

    #[test]
    fn prop_overriding() {
        let s = CssStyleDeclaration::new();

        s.add_prop(StyleProp::Display(CssDisplay::None));
        s.add_prop(StyleProp::Display(CssDisplay::Block));

        assert!(s.props().eq(&vec![StyleProp::Display(CssDisplay::Block)]));
    }
}
