// subset of CSS selectors
// x to support CSS-in-JS libs
// x no specificity for now
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

use crate::util::Atom;
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tag {
    LocalName(String),
    Identifier(String),
    ClassNamePart(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selector {
    parts: Vec<SelectorPart>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SelectorPart {
    Combinator(Combinator),
    Tag(Atom<Tag>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Combinator {
    Universal,
    Parent,
    Ancestor,
    Or,
}

impl Selector {
    // mask of all tail tags
    // can be stored somewhere and used for short-circuiting:
    // (s.tail_mask().includes(SelectorMask::from(&[Tag::LocalName(..), Tag::ClassNamePart(..), ...])))
    pub fn tail_mask(&self) -> SelectorMask {
        // TODO: split by Combinator::Or and get tail (first because we are reversed)
        //SelectorMask::from(self.parts.iter().filter_map(|p| match p {
        //    SelectorPart::Tag(tag) => Some(tag),
        //    _ => None,
        //}))

        todo!()
    }

    pub fn matches<'a>(&'a self, tags_stack: &[Vec<Atom<Tag>>]) -> bool {
        debug_assert!(tags_stack.len() > 0);

        // useful for reset
        let initial_pos = tags_stack.len() - 1;

        // so we can fast-forward to next OR
        let mut parts_iter = self.parts.iter();

        // state
        let mut pos = initial_pos;
        let mut parent = false;
        let mut ancestors = false;

        // we are always going forward
        'next_part: while let Some(p) = parts_iter.next() {
            match p {
                SelectorPart::Tag(t) => {
                    loop {
                        if parent || ancestors {
                            parent = false;

                            // nothing left to match
                            if pos == 0 {
                                break;
                            }

                            // go up
                            pos -= 1;
                        }

                        if tags_stack[pos].contains(t) {
                            ancestors = false;
                            continue 'next_part;
                        }

                        if !ancestors {
                            break;
                        }
                    }

                    // no match, fast-forward to next OR
                    while let Some(p) = parts_iter.next() {
                        if p == &SelectorPart::Combinator(Combinator::Or) {
                            // reset stack
                            pos = initial_pos;
                            continue 'next_part;
                        }
                    }

                    // or fail otherwise
                    return false;
                }

                // state changes
                SelectorPart::Combinator(Combinator::Parent) => parent = true,
                SelectorPart::Combinator(Combinator::Ancestor) => ancestors = true,

                // no-op
                SelectorPart::Combinator(Combinator::Universal) => {}

                // we still have a match, no need to check others
                SelectorPart::Combinator(Combinator::Or) => return true,
            }
        }

        // everything was fine
        true
    }
}

#[derive(Debug)]
pub struct InvalidSelector;

impl TryFrom<&str> for Selector {
    type Error = InvalidSelector;

    fn try_from(selector: &str) -> Result<Self, Self::Error> {
        (parse::selector() - pom::parser::end())
            .parse(selector.as_bytes())
            .map_err(|_| InvalidSelector)
    }
}

pub struct SelectorMask(u32);

impl SelectorMask {
    const BITS: usize = core::mem::size_of::<usize>() * 8;

    pub fn includes(&self, other: SelectorMask) -> bool {
        (self.0 & other.0) != 0
    }
}

impl<'a, T: IntoIterator<Item = &'a Atom<Tag>>> From<T> for SelectorMask {
    fn from(tags: T) -> Self {
        use std::hash::{Hash, Hasher};

        // TODO: maybe it could be oneliner too (fold)
        let hash = |tag: &Tag| {
            let mut hasher = fnv::FnvHasher::default();
            tag.hash(&mut hasher);
            hasher.finish()
        };

        // TODO: test
        // TODO: zero?
        Self(
            tags.into_iter()
                .fold(0, |res, t| res | 1 << (hash(t) as usize - 1) % Self::BITS),
        )
    }
}

pub(crate) mod parse {
    use super::*;
    use pom::char_class::alphanum;
    use pom::parser::*;

    pub(crate) fn selector<'a>() -> Parser<'a, u8, Selector> {
        let tag = || {
            let ident = || ident().map(str::to_owned);
            let local_name = ident().map(Tag::LocalName);
            let id = sym(b'#') * ident().map(Tag::Identifier);
            let class_name = sym(b'.') * ident().map(Tag::ClassNamePart);
            let universal = sym(b'*').map(|_| SelectorPart::Combinator(Combinator::Universal));

            universal | (local_name | id | class_name).map(Atom::new).map(SelectorPart::Tag)
        };

        // note we parse child/descendant but we flip the final order so it's parent/ancestor
        let child = space() * sym(b'>') * space().map(|_| Combinator::Parent);
        let descendant = sym(b' ').repeat(1..).map(|_| Combinator::Ancestor);
        let or = space() * sym(b',') * space().map(|_| Combinator::Or);
        let comb = (child | descendant | or).map(SelectorPart::Combinator);

        let selector = tag() + (comb.opt() + tag()).repeat(0..);

        selector.map(|(head, tail)| {
            let mut parts = Vec::with_capacity(tail.len() + 1);

            // reversed (child/descendant -> parent/ancestor)
            for (comb, tag) in tail.into_iter().rev() {
                parts.push(tag);

                if let Some(comb) = comb {
                    parts.push(comb);
                }
            }

            parts.push(head);

            Selector { parts }
        })
    }

    fn space<'a>() -> Parser<'a, u8, ()> {
        sym(b' ').repeat(0..).discard()
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

    // TODO: it should be possible (tag is nonzero) but maybe it's easier to make Atom shorter
    #[ignore]
    #[test]
    fn part_has_same_size_as_tag() {
        use std::mem::size_of;

        assert_eq!(size_of::<SelectorPart>(), size_of::<Atom<Tag>>())
    }

    #[test]
    fn parsing() {
        use Combinator as C;
        use SelectorPart as P;
        use Tag as T;

        let s = |s| Selector::try_from(s).unwrap().parts;
        #[allow(non_snake_case)]
        let A = Atom::new;

        // simple
        assert_eq!(s("*"), &[P::Combinator(C::Universal)]);
        assert_eq!(s("body"), &[P::Tag(A(T::LocalName("body".into())))]);
        assert_eq!(s("h2"), &[P::Tag(A(T::LocalName("h2".into())))]);
        assert_eq!(s("#app"), &[P::Tag(A(T::Identifier("app".into())))]);
        assert_eq!(s(".btn"), &[P::Tag(A(T::ClassNamePart("btn".into())))]);

        // combined
        assert_eq!(
            s(".btn.btn-primary"),
            &[
                P::Tag(A(T::ClassNamePart("btn-primary".into()))),
                P::Tag(A(T::ClassNamePart("btn".into())))
            ]
        );
        assert_eq!(
            s("*.test"),
            &[
                P::Tag(A(T::ClassNamePart("test".into()))),
                P::Combinator(Combinator::Universal)
            ]
        );
        assert_eq!(
            s("div#app.test"),
            &[
                P::Tag(A(T::ClassNamePart("test".into()))),
                P::Tag(A(T::Identifier("app".into()))),
                P::Tag(A(T::LocalName("div".into())))
            ]
        );

        // combined with combinators
        assert_eq!(
            s("body > div.test div#test"),
            &[
                P::Tag(A(T::Identifier("test".into()))),
                P::Tag(A(T::LocalName("div".into()))),
                P::Combinator(C::Ancestor),
                P::Tag(A(T::ClassNamePart("test".into()))),
                P::Tag(A(T::LocalName("div".into()))),
                P::Combinator(C::Parent),
                P::Tag(A(T::LocalName("body".into())))
            ]
        );

        // multi
        assert_eq!(
            s("html, body"),
            &[
                P::Tag(A(T::LocalName("body".into()))),
                P::Combinator(C::Or),
                P::Tag(A(T::LocalName("html".into())))
            ]
        );
        assert_eq!(
            s("body > div, div button span"),
            &[
                P::Tag(A(T::LocalName("span".into()))),
                P::Combinator(C::Ancestor),
                P::Tag(A(T::LocalName("button".into()))),
                P::Combinator(C::Ancestor),
                P::Tag(A(T::LocalName("div".into()))),
                P::Combinator(C::Or),
                P::Tag(A(T::LocalName("div".into()))),
                P::Combinator(C::Parent),
                P::Tag(A(T::LocalName("body".into()))),
            ]
        );

        // invalid
        assert!(Selector::try_from("").is_err());
        assert!(Selector::try_from(" ").is_err());
        assert!(Selector::try_from("a,,b").is_err());
        assert!(Selector::try_from("a>>b").is_err());
    }

    #[test]
    fn matching() {
        use Tag as T;

        let s = |s| Selector::try_from(s).unwrap();

        let stack = vec![
            vec![T::LocalName("html".into())],
            vec![T::LocalName("body".into()), T::Identifier("app".into())],
            vec![T::LocalName("div".into()), T::Identifier("panel".into())],
            vec![T::LocalName("button".into()), T::ClassNamePart("btn".into())],
            vec![T::LocalName("span".into())],
        ]
        .iter()
        .map(|tags| tags.iter().cloned().map(Atom::new).collect())
        .collect::<Vec<Vec<_>>>();

        // basic
        assert!(s("*").matches(&stack));
        assert!(s("html").matches(&stack[0..1]));
        assert!(s("body").matches(&stack[1..2]));
        assert!(s("div").matches(&stack[2..3]));
        assert!(s("button").matches(&stack[3..4]));
        assert!(s("span").matches(&stack[4..5]));

        // combined
        assert!(s("#app").matches(&stack[1..2]));
        assert!(s("div#panel").matches(&stack[2..3]));
        assert!(s(".btn").matches(&stack[3..4]));

        // parent
        assert!(s("button > span").matches(&stack));
        assert!(s("div#panel > button.btn > span").matches(&stack));

        // ancestor
        assert!(s("button span").matches(&stack));
        assert!(s("div#panel span").matches(&stack));
        assert!(s("body div .btn span").matches(&stack));

        // OR
        assert!(s("div, span").matches(&stack));
        assert!(s("a, b, c, span, d").matches(&stack));
        assert!(s("html, body").matches(&stack[1..2]));

        // complex
        assert!(s("div, span.foo, #panel span").matches(&stack));
        assert!(s("a b c d e f g, span").matches(&stack));
    }
}
