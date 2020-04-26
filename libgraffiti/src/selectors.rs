// subset of CSS selectors
// - to support CSS-in-JS libs
// x no specificity
//   - should be fine, generic rules are usually first
// x no first/last/nth/siblings
// x tagName
// x id
// - className
// x direct descendant
// - descendant
// - combination
// - decoupled from other systems

use crate::commons::Lookup;

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    TagName(String),
    Id(String),
    ClassName(String),

    // - not common in CSS-in-JS so Box is fine
    Parent(Box<(Selector, Selector)>),
    Ancestor(Box<(Selector, Selector)>),
}

pub struct MatchingContext<'a, Item> {
    tag_names: &'a dyn Lookup<Item, &'a str>,
    ids: &'a dyn Lookup<Item, &'a str>,
    class_names: &'a dyn Lookup<Item, &'a str>,

    // root has no parent
    parents: &'a dyn Lookup<Item, Option<Item>>,
}

impl<'a, Item: Copy> MatchingContext<'a, Item> {
    pub fn match_selector(&self, selector: &Selector, item: Item) -> bool {
        match selector {
            Selector::TagName(tn) => self.tag_names.lookup(item) == *tn,
            Selector::Id(id) => self.ids.lookup(item) == *id,
            Selector::ClassName(cn) => self.class_names.lookup(item) == *cn,

            Selector::Parent(data) => {
                let (parent_sel, child_sel) = (&data.0, &data.1);

                if let Some(parent) = self.parents.lookup(item) {
                    self.match_selector(child_sel, item) && self.match_selector(parent_sel, parent)
                } else {
                    false
                }
            }

            Selector::Ancestor(data) => {
                let (ancestor_sel, descendant_sel) = (&data.0, &data.1);

                self.match_selector(descendant_sel, item) && self.ancestors(item).any(|a| self.match_selector(ancestor_sel, a))
            }
        }
    }

    fn ancestors(&self, item: Item) -> Ancestors<Item> {
        Ancestors(self.parents, Some(item))
    }
}

pub fn parse(input: &str) -> Selector {
    let mut parser = Parser { iter: input.chars().peekable() };

    parser.parse_selector().expect("invalid selector")
}

struct Parser<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Parser<'_> {
    fn parse_selector(&mut self) -> Option<Selector> {
        let mut res = None;

        // TODO: compound, parent, ancestor
        while let Some(c) = self.iter.peek() {
            match c {
                c if word_char(c) => {
                    res = Some(Selector::TagName(self.parse_tag_name()));
                }

                '#' => {
                    self.iter.next();
                    return Some(Selector::Id(self.parse_id()));
                }

                '.' => {
                    self.iter.next();
                    return Some(Selector::ClassName(self.parse_class_name()));
                }

                ' ' if res.is_some() => {
                    self.iter.next();
                    res = Some(Selector::Ancestor(Box::new((res.unwrap(), self.parse_selector().expect("invalid descendant selector")))));
                }

                _ => panic!("unexpected {:?}", c),
            }
        }

        res
    }

    fn parse_id(&mut self) -> String {
        self.parse_word()
    }

    fn parse_tag_name(&mut self) -> String {
        self.parse_word()
    }

    fn parse_class_name(&mut self) -> String {
        self.parse_word()
    }

    fn parse_word(&mut self) -> String {
        self.take_while(word_char)
    }

    // built-in take_while consumes the value
    fn take_while(&mut self, f: impl FnMut(&char) -> bool) -> String {
        let res: String = self.iter.clone().take_while(f).collect();

        for _ in 0..res.chars().count() {
            self.iter.next();
        }

        res
    }
}

fn word_char(c: &char) -> bool {
    ('a'..='z').contains(c)
}

pub struct Ancestors<'a, Item>(&'a dyn Lookup<Item, Option<Item>>, Option<Item>);

impl<'a, Item: Copy> Iterator for Ancestors<'a, Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.1;
        self.1 = next.and_then(|_| self.0.lookup(self.1.unwrap()));

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse("body"), Selector::TagName("body".to_string()));
        assert_eq!(parse("#app"), Selector::Id("app".to_string()));
        assert_eq!(parse(".btn"), Selector::ClassName("btn".to_string()));
        assert_eq!(
            parse("body div"),
            Selector::Ancestor(Box::new((Selector::TagName("body".to_string()), Selector::TagName("div".to_string()))))
        );
    }

    #[test]
    fn matching() {
        let tag_names = vec!["body", "div", "button"];
        let ids = vec!["", "app", ""];
        let class_names = vec!["", "", "btn"];
        let parents = vec![None, Some(0), Some(0)];

        let ctx = MatchingContext {
            tag_names: &tag_names,
            ids: &ids,
            class_names: &class_names,
            parents: &parents,
        };

        assert!(ctx.match_selector(&parse("body"), 0));
        assert!(ctx.match_selector(&parse("div"), 1));
        assert!(ctx.match_selector(&parse("button"), 2));

        assert!(ctx.match_selector(&parse("#app"), 1));

        assert!(ctx.match_selector(&parse(".btn"), 2));

        assert!(ctx.match_selector(&Selector::Parent(Box::new((Selector::TagName("body".to_string()), Selector::TagName("div".to_string())))), 1));
    }
}
