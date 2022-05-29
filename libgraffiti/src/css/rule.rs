use super::parsing::{sym, Parsable, ParseError, Parser};
use super::{Selector, Style};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule {
    selector: Selector,
    style: Style,
}

impl StyleRule {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Parsable::parse(input)
    }

    pub fn selector(&self) -> &Selector {
        &self.selector
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl Parsable for StyleRule {
    fn parser<'a>() -> Parser<'a, Self> {
        let rule = Selector::parser() - sym("{") + Style::parser() - sym("}");

        rule.map(|(selector, style)| StyleRule { selector, style })
    }
}

impl fmt::Display for StyleRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {{ {} }}", &self.selector, &self.style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule() -> Result<(), ParseError> {
        let selector = Selector::parse("div")?;
        let style = Style::parse("color: #fff")?;

        assert_eq!(StyleRule::parse("div { color: #fff }")?, StyleRule { selector, style });

        Ok(())
    }
}
