use super::parser::{sheet, tokenize, ParseError};
use super::rule::CssStyleRule;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CssStyleSheet {
    rules: Vec<CssStyleRule>,
}

impl CssStyleSheet {
    pub fn new(rules: Vec<CssStyleRule>) -> Self {
        Self { rules }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = sheet() - pom::parser::end();

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
