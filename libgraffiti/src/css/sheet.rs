use super::parser::{sheet, tokenize, ParseError};
use super::rule::StyleRule;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    pub fn new(rules: Vec<StyleRule>) -> Self {
        Self { rules }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = sheet() - pom::parser::end();

        parser.parse(&tokens)
    }

    pub fn rules(&self) -> &[StyleRule] {
        &self.rules
    }

    pub fn insert_rule(&mut self, rule: StyleRule, index: usize) {
        self.rules.insert(index, rule);
    }

    pub fn delete_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
}
