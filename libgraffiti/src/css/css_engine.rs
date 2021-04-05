use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct StyleSheet {
    pub(super) rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn insert_rule(&mut self, rule: Rule, index: usize) {
        self.rules.insert(index, rule);
    }

    pub fn delete_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
}

// should never fail
impl From<&str> for StyleSheet {
    fn from(s: &str) -> Self {
        super::parser::sheet().parse(s.as_bytes()).unwrap_or(Default::default())
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub(super) selector: Selector,
    pub(super) style: Style,
}

// TODO
pub struct CssEngine {}
impl CssEngine {
    pub fn new() -> Self {
        Self {}
    }
}
