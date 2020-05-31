// subset of CSS selectors
// - to support CSS-in-JS libs
// x no specificity
//   - should be fine, generic rules are usually first
// x no first/last/nth/siblings
// x universal
// x local name
// x id
// x class
// x child
// x descendant
// x multiple (div, span)
// x combination
// x decoupled from other systems

use crate::util::Lookup;

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    // simple & fast
    Universal,
    LocalName(String),
    Id(String),
    ClassName(String),

    // a, b, c, ...
    // (slower but not very common in CSS-in-JS)
    Multi(Vec<Selector>),

    // div.container#app.bg-primary > div span
    // (slowest)
    // TODO: rename to Compound?
    Combined(Vec<Selector>),

    // internal
    Combinator(Combinator),
}


#[derive(Debug, PartialEq)]
pub enum SelectorError {
    InvalidSelector
}

#[derive(Debug, Clone, PartialEq)]
pub enum Combinator {
    Parent,
    Ancestor,
}

// what's needed for matching
pub struct MatchingContext<'a, Item, Ancestors> {
    pub local_names: &'a dyn Lookup<Item, &'a str>,
    pub ids: &'a dyn Lookup<Item, Option<&'a str>>,
    pub class_names: &'a dyn Lookup<Item, Option<&'a str>>,
    pub ancestors: &'a dyn Lookup<Item, Ancestors>,
}

impl<'a, Item: Copy, Ancestors: IntoIterator<Item = Item>> MatchingContext<'a, Item, Ancestors> {
    pub fn match_selector<'b>(&self, selector: &'b Selector, item: Item) -> bool {
        match selector {
            Selector::Universal => true,
            Selector::LocalName(tn) => self.local_names.lookup(item) == tn,
            Selector::Id(id) => self.ids.lookup(item) == Some(id),
            Selector::ClassName(cn) => self.class_name_contains(item, cn),

            Selector::Multi(sels) => sels.iter().any(|s| self.match_selector(s, item)),
            Selector::Combined(parts) => self.match_rest(parts.iter(), item),

            // internal part of Combined(...)
            Selector::Combinator(_) => panic!("unexpected combinator"),
        }
    }

    fn match_rest<'b>(&self, mut iter: impl Iterator<Item = &'b Selector> + Clone, item: Item) -> bool {
        while let Some(s) = iter.next() {
            match s {
                // take the rest of components & evaluate them in different context(s)
                Selector::Combinator(c) => {
                    let mut ancestors = self.ancestors.lookup(item).into_iter();

                    match c {
                        Combinator::Parent => return ancestors.next().map(|parent| self.match_rest(iter, parent)).unwrap_or(false),
                        Combinator::Ancestor => return ancestors.any(|ancestor| self.match_rest(iter.clone(), ancestor)),
                    }
                }

                _ => {
                    if !self.match_selector(s, item) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn class_name_contains(&self, item: Item, cn: &str) -> bool {
        self.class_names.lookup(item).map(|s| s.split_ascii_whitespace().find(|part| part == &cn)).is_some()
    }
}

pub fn parse_selector(s: &str) -> Result<Selector, SelectorError> {
    parse::selector().parse(s.trim().as_bytes()).map_err(|_| SelectorError::InvalidSelector)
}

mod parse {
    use super::*;
    use pom::char_class::alphanum;
    use pom::parser::*;

    pub fn selector<'a>() -> Parser<'a, u8, Selector> {
        let comma = sym(b' ').repeat(0..) * sym(b',') * sym(b' ').repeat(0..);
        let selectors = list(call(single_selector), comma);

        selectors.convert(|mut parts| {
            if parts.is_empty() {
                return Err("expected at least one selector");
            }

            if parts.len() == 1 {
                return Ok(parts.remove(0));
            }

            Ok(Selector::Multi(parts))
        })
    }

    pub fn single_selector<'a>() -> Parser<'a, u8, Selector> {
        let local_name = ident().map(|s| Selector::LocalName(s.to_string()));
        let id = sym(b'#') * ident().map(|s| Selector::Id(s.to_string()));
        let class_name = sym(b'.') * ident().map(|s| Selector::ClassName(s.to_string()));
        let universal = sym(b'*').map(|_| Selector::Universal);

        // note we parse Parent/Ancestor & flip the final order
        let child = sym(b' ').repeat(0..) * sym(b'>') * sym(b' ').repeat(0..).map(|_| Selector::Combinator(Combinator::Parent));
        let descendant = sym(b' ').repeat(1..).map(|_| Selector::Combinator(Combinator::Ancestor));

        (local_name | id | class_name | universal | child | descendant).repeat(1..).map(|mut components| {
            if components.len() == 1 {
                return components.remove(0);
            }

            components.reverse();

            Selector::Combined(components)
        })
    }

    fn ident<'a>() -> Parser<'a, u8, &'a str> {
        is_a(alphanum_dash).repeat(1..).collect().convert(std::str::from_utf8)
    }

    fn alphanum_dash(b: u8) -> bool {
        alphanum(b) || b == b'-'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(selector: &str) -> Selector {
        parse_selector(selector).unwrap()
    }

    #[test]
    fn parsing() {
        use super::Selector::*;

        // simple
        assert_eq!(s("*"), Universal);
        assert_eq!(s("body"), LocalName("body".to_string()));
        assert_eq!(s("h2"), LocalName("h2".to_string()));
        assert_eq!(s("#app"), Id("app".to_string()));
        assert_eq!(s(".btn"), ClassName("btn".to_string()));

        // combined
        assert_eq!(s(".btn.btn-primary"), Combined(vec![ClassName("btn-primary".to_string()), ClassName("btn".to_string())]));
        assert_eq!(s("*.test"), Combined(vec![ClassName("test".to_string()), Universal]));
        assert_eq!(s("div#app.test"), Combined(vec![ClassName("test".to_string()), Id("app".to_string()), LocalName("div".to_string())]));

        // combined with combinators
        assert_eq!(
            s("body > div.test div#test"),
            Combined(vec![
                Id("test".to_string()),
                LocalName("div".to_string()),
                Combinator(super::Combinator::Ancestor),
                ClassName("test".to_string()),
                LocalName("div".to_string()),
                Combinator(super::Combinator::Parent),
                LocalName("body".to_string()),
            ])
        );

        // multi
        assert_eq!(s("html, body"), Multi(vec![LocalName("html".to_string()), LocalName("body".to_string())]));
        assert_eq!(
            s("body > div, div div"),
            Multi(vec![
                Combined(vec![LocalName("div".to_string()), Combinator(super::Combinator::Parent), LocalName("body".to_string()),]),
                Combined(vec![LocalName("div".to_string()), Combinator(super::Combinator::Ancestor), LocalName("div".to_string()),])
            ])
        );
    }

    #[test]
    fn matching() {
        let local_names = &vec!["body", "div", "button"];
        let ids = &vec![Some("app"), Some("panel"), None];
        let class_names = &vec![None, None, Some("btn")];
        let ancestors = &vec![vec![], vec![0], vec![1, 0]];

        let ctx = MatchingContext {
            local_names,
            ids,
            class_names,
            ancestors,
        };

        assert!(ctx.match_selector(&s("*"), 0));
        assert!(ctx.match_selector(&s("body"), 0));
        assert!(ctx.match_selector(&s("div"), 1));
        assert!(ctx.match_selector(&s("button"), 2));

        assert!(ctx.match_selector(&s("#app"), 0));
        assert!(ctx.match_selector(&s("div#panel"), 1));
        assert!(ctx.match_selector(&s(".btn"), 2));

        assert!(ctx.match_selector(&s("body > div"), 1));
        assert!(ctx.match_selector(&s("body div"), 1));

        assert!(ctx.match_selector(&s("div > button"), 2));
        assert!(ctx.match_selector(&s("div button"), 2));
        assert!(ctx.match_selector(&s("body > div > button"), 2));
        assert!(ctx.match_selector(&s("body div button"), 2));

        assert!([0, 1, 2].iter().all(|i| ctx.match_selector(&s("body, div, button"), *i)));
    }
}
