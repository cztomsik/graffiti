use super::*;

// just a fn for now
pub(crate) fn matching_rules<'a, E: Copy>(ctx: &MatchingContext<'a, E>, sheets: &'a [CssStyleSheet], el: E) -> impl Iterator<Item = &'a CssStyleRule> + 'a {
    let mut rules: Vec<_> = sheets
        .iter()
        .flat_map(|s| &s.rules)
        .filter_map(|r| ctx.match_selector(&r.selector, el).map(move |spec| (spec, r)))
        .collect();

    rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    rules.into_iter().map(|(_, r)| r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn matching_style() {
        let sheet = CssStyleDeclaration::from(
            ".a { display: block }
             .b { display: none }",
        );

        //let mut res = Style::new();
        //sheet.matching_rules(&mut res, |s| s == &sheet.rules[0].selector);

        //assert_eq!(res.css_text(), "display: none");
    }
}
