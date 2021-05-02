// TODO:
// - shorthands
// - normalize (bold -> 700)

use super::StyleProp;
use once_cell::sync::Lazy;
use std::fmt::Write;
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub(super) props: Vec<StyleProp>,
}

impl Style {
    pub const EMPTY: Self = Self::new();

    pub const HIDDEN: Lazy<Self> = Lazy::new(|| Self::from("display: none"));

    pub const INITIAL: Lazy<Self> = Lazy::new(|| {
        Self::from(
            "
                width: auto;
                height: auto;
                padding: 0;
                margin: 0;
                background: rgba(0, 0, 0, 0);
                border: 0 none rgba(0, 0, 0, 0);
                border-radius: 0;
                flex: 0 1 auto;
                flex-wrap: nowrap;
                flex-direction: row;
                align-content: stretch;
                align-items: stretch;
                align-self: auto;
                justify-content: flex-start;
                font: normal normal 400 16px/20px sans-serif;
                text-align: left;
                color: #000;
                outline: 3px none #000;
                overflow: visible;
                position: static;
                top: auto;
                right: auto;
                bottom: auto;
                left: auto;
                display: inline;
                opacity: 1;
                visibility: visible;
            ",
        )
    });

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
        self.find_prop_by_name(prop).map(StyleProp::value_as_string)
    }

    pub(super) fn find_prop_by_name(&self, prop: &str) -> Option<&StyleProp> {
        self.props.iter().find(|p| p.name() == prop)
    }

    // TODO: priority
    pub fn set_property(&mut self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        if let Ok(prop) = super::parser::parse_style_prop(prop, &tokens) {
            self.add_prop(prop)
        }
    }

    // TODO: should return previous value
    pub fn remove_property(&mut self, prop: &str) {
        self.props.retain(|p| p.name() == prop);
    }

    pub fn css_text(&self) -> String {
        self.props().fold(String::new(), |mut s, p| {
            write!(s, "{}: {};", p.name(), p.value_as_string());
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

        s.add_prop(StyleProp::Display(CssDisplay::None));
        s.add_prop(StyleProp::Display(CssDisplay::Block));

        assert!(Iterator::eq(s.props(), &vec![StyleProp::Display(CssDisplay::Block)]));
    }
}
