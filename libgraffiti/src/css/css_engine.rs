use super::*;

// just a fn for now
pub(crate) fn matching_rules<'a, E: Copy>(ctx: &MatchingContext<'a, E>, sheets: &'a [StyleSheet], el: E) -> impl Iterator<Item = &'a Rule> + 'a {
    let mut rules: Vec<_> = sheets
        .iter()
        .flat_map(|s| &s.rules)
        .filter_map(|r| ctx.match_selector(&r.selector, el).map(move |spec| (spec, r)))
        .collect();

    rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    rules.into_iter().map(|(_, r)| r)
}

#[derive(Debug, PartialEq)]
pub struct StyleSheet {
    pub(super) rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn insert_rule(&mut self, rule: Rule, index: usize) {
        self.rules.insert(index, rule);
    }

    pub fn delete_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
}

// should never fail
impl From<&str> for StyleSheet {
    fn from(sheet: &str) -> Self {
        let tokens = super::parser::tokenize(sheet.as_bytes());
        let parser = super::parser::sheet();

        parser.parse(&tokens).unwrap_or_else(|_| Self::new())
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    selector: Selector,
    style: Style,
}

impl Rule {
    pub fn new(selector: Selector, style: Style) -> Self {
        Self { selector, style }
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn matching_style() {
        let sheet = StyleSheet::from(
            ".a { display: block }
             .b { display: none }",
        );

        //let mut res = Style::new();
        //sheet.matching_rules(&mut res, |s| s == &sheet.rules[0].selector);

        //assert_eq!(res.css_text(), "display: none");
    }
}
