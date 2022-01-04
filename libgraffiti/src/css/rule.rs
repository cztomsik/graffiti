use super::parser::{rule, tokenize, ParseError};
use super::selector::Selector;
use super::style::CssStyle;

#[derive(Debug, Clone, PartialEq)]
pub struct CssStyleRule {
    selector: Selector,
    style: CssStyle,
}

impl CssStyleRule {
    pub fn new(selector: Selector, style: CssStyle) -> Self {
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

    pub fn style(&self) -> &CssStyle {
        &self.style
    }
}
