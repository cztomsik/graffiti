use super::{CssStyleSheet, Element, StyleProp};
use crate::util::Bloom;
use fnv::FnvHashMap;
use std::rc::Rc;

pub struct StyleResolver {
    sheets: Vec<Rc<CssStyleSheet>>,

    // (sheet, rule) indices grouped by rule.selector().tail_mask()
    groups: Vec<(Bloom<()>, Vec<(usize, usize)>)>,
}

impl StyleResolver {
    pub fn new(sheets: Vec<Rc<CssStyleSheet>>) -> Self {
        let mut groups = FnvHashMap::default();

        for (i, sheet) in sheets.iter().enumerate() {
            for (j, rule) in sheet.rules().iter().enumerate() {
                groups
                    .entry(rule.selector.tail_mask())
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }

        Self {
            sheets,
            groups: groups.into_iter().collect(),
        }
    }

    pub fn resolve_style<R: Default>(&self, element: &impl Element, apply_prop_fn: impl Fn(&mut R, &StyleProp)) -> R {
        let mut res = R::default();

        for sheet in &self.sheets {
            for rule in sheet.rules() {
                if let Some(_spec) = rule.selector.match_element(element) {
                    for p in rule.style().props().iter() {
                        apply_prop_fn(&mut res, p);
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sheet = CssStyleSheet::default_ua_sheet();
        let resolver = StyleResolver::new(vec![Rc::new(sheet)]);

        // TODO
        // assert_eq!(resolver.resolve(el, Vec::push), vec![...])
    }
}

/*

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
*/
