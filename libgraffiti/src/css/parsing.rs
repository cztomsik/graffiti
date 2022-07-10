// parsing utils

// notes:
// - we are using parser-combinators (with tokenizer)
//   - see https://github.com/J-F-Liu/pom for reference
//   - tokens are just &str, there are no other token types
//   - it's probably a bit inefficient but very expressive (~350 lines)
// - repeat() for skip/discard() should be alloc-free because of zero-sized types
// - collect() creates slice from start to end regardless of the results "inside"
//   (which means (a + b).collect() only takes "super-slice" of both matches)
// - we are only parsing known/valid props, which means tokenizer can be simpler
//   and we also get correct overriding for free (only valid prop will override prev one)

use super::tokenize::tokenize;
use std::fmt::Debug;
use std::str::FromStr;

pub use pom::char_class::alphanum;
pub use pom::parser::{any, empty, is_a, list, none_of, one_of, seq, skip, sym};

pub type Parser<'a, T> = pom::parser::Parser<'a, Token<'a>, T>;

// TODO: maybe we could have a struct (with row/col), it just needs to be Deref<str>
//       but on the other hand, &str contains offset so we can compute row/col easily anyway
pub type Token<'a> = &'a str;

pub type ParseError = pom::Error;

pub(super) trait Parsable: Sized {
    fn parser<'a>() -> Parser<'a, Self>;

    fn parse(input: &str) -> Result<Self, ParseError> {
        let tokens = tokenize(input.as_bytes());
        let parser = Self::parser() - pom::parser::end();

        parser.parse(&tokens)
    }
}

impl<T: 'static + FromStr> Parsable for T
where
    <T as FromStr>::Err: Debug,
{
    fn parser<'a>() -> Parser<'a, Self> {
        any().convert(str::parse)
    }
}

pub fn ident<'a>() -> Parser<'a, &'a str> {
    is_a(|t: &str| alphanum_dash(t.as_bytes()[0]))
}

pub fn fail<'a, T: 'static>(msg: &'static str) -> Parser<'a, T> {
    empty().convert(move |_| Err(msg))
}

pub fn alphanum_dash(b: u8) -> bool {
    alphanum(b) || b == b'-'
}

pub fn decimal(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'.')
}

pub fn space(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t' | b'\r')
}
