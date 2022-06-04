use super::parsing::{seq, skip, sym, Parsable, ParseError, Parser};
use super::StyleRule;
use std::fmt;

#[derive(Debug, Default)]
pub struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    fn new(rules: Vec<StyleRule>) -> Self {
        Self { rules }
    }

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Parsable::parse(input)
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

impl Parsable for StyleSheet {
    fn parser<'a>() -> Parser<'a, Self> {
        // anything until next "}}" (empty media is matched with unknown)
        let media =
            sym("@") * sym("media") * (!seq(&["}", "}"]) * skip(1)).repeat(1..).map(|_| None) - sym("}") - sym("}");
        // anything until next "}"
        let unknown = (!sym("}") * skip(1)).repeat(1..).map(|_| None) - sym("}").opt();

        (StyleRule::parser().map(Option::Some) | media | unknown)
            .repeat(0..)
            .map(|maybe_rules| Self::new(maybe_rules.into_iter().flatten().collect()))
    }
}

impl fmt::Display for StyleSheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.rules {
            write!(f, "{}\n", r)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Selector, Style};
    use super::*;

    #[test]
    fn parse_sheet() -> Result<(), ParseError> {
        let sheet = StyleSheet::parse("div { color: #fff }")?;

        assert_eq!(sheet.rules()[0].selector(), &Selector::parse("div")?);
        assert_eq!(sheet.rules()[0].style(), &Style::parse("color: #fff")?);
        assert_eq!(sheet.rules()[0].style().to_string(), "color: rgba(255, 255, 255, 255)");

        // white-space
        assert_eq!(StyleSheet::parse(" *{}")?.rules().len(), 1);
        assert_eq!(StyleSheet::parse("\n*{\n}\n")?.rules().len(), 1);

        // forgiving/future-compatibility
        assert_eq!(StyleSheet::parse(":root {} a { v: 0 }")?.rules().len(), 2);
        assert_eq!(StyleSheet::parse("a {} @media { a { v: 0 } } b {}")?.rules().len(), 2);
        assert_eq!(StyleSheet::parse("@media { a { v: 0 } } a {} b {}")?.rules().len(), 2);

        Ok(())
    }
}
