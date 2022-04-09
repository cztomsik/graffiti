use super::parser::{rule, tokenize, ParseError};
use super::selector::Selector;
use super::style::Style;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    selector: Selector,
    style: Style,
}

impl StyleRule {
    pub fn new(selector: Selector, style: Style) -> Self {
        Self { selector, style }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = rule() - pom::parser::end();

        parser.parse(&tokens)
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}
