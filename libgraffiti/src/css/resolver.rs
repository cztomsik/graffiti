use super::{MatchingContext, Style, StyleSheet};

pub struct StyleResolver<'a, M> {
    // TODO: it might be interesting to opt-out from specificity sorting
    // TODO: media (for matching media rules)
    ua_sheet: &'a StyleSheet,
    sheets: &'a [StyleSheet],
    context: &'a M,
}

impl<'a, M: MatchingContext> StyleResolver<'a, M> {
    pub fn new(context: &'a M, ua_sheet: &'a StyleSheet, sheets: &'a [StyleSheet]) -> Self {
        Self {
            ua_sheet,
            sheets,
            context,
        }
    }

    pub fn resolve_style(
        &self,
        element: M::ElementRef,
        inline_style: Option<&Style>,
        _parent_style: Option<&Style>,
    ) -> Style {
        let mut res = Style::default();

        let sheets = std::iter::once(self.ua_sheet).chain(self.sheets.iter());

        let mut rules: Vec<_> = sheets
            .flat_map(|s| s.rules())
            .filter_map(|r| {
                r.selector()
                    .match_element(element, self.context)
                    .map(move |spec| (spec, r))
            })
            .collect();
        rules.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (_, r) in rules {
            res.apply(r.style());
        }

        if let Some(s) = inline_style {
            res.apply(s);
        }

        // TODO: inherit, css-vars

        res
    }
}
