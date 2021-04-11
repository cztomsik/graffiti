use super::*;

// just a fn for now
pub(crate) fn matching_style<'a, E: Copy>(
    ctx: &MatchingContext<'a, E>,
    sheets: &'a [StyleSheet],
    el: E
) -> Style {
    let mut matching_rules: Vec<_> = sheets
        .iter()
        .flat_map(|s| &s.rules)
        .filter_map(|r| ctx.match_selector(&r.selector, el).map(move |spec| (spec, r)))
        .collect();

    matching_rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut style = Style::new();

    for (_, r) in &matching_rules {
        for p in &r.style.props {
            style.add_prop(p.clone());
        }
    }

    style
}

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
