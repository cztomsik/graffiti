// parsing utils

// notes:
// - we are using parser-combinators (for both tokenizing & parsing)
//   - see https://github.com/J-F-Liu/pom for reference
//   - tokens are just &str, there are no other token types
//   - it's probably a bit inefficient but very expressive (~350 lines)
// - repeat() for skip/discard() should be alloc-free because of zero-sized types
// - collect() creates slice from start to end regardless of the results "inside"
//   (which means (a + b).collect() only takes "super-slice" of both matches)
// - we are only parsing known/valid props, which means tokenizer can be simpler
//   and we also get correct overriding for free (only valid prop will override prev one)

use super::tokenize::tokenize;

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

pub fn float<'a>() -> Parser<'a, f32> {
    any().convert(str::parse)
}

pub fn u8<'a>() -> Parser<'a, u8> {
    any().convert(str::parse)
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

// not sure if this is a good idea but it's useful for tokenization
// (hex is only consumed if it's after `#` but `#` is a separate token)
pub fn prev<'a, I: Clone>(n: usize) -> pom::parser::Parser<'a, I, ()> {
    pom::parser::Parser::new(move |_, position: usize| {
        if position >= n {
            Ok(((), position - n))
        } else {
            Err(pom::Error::Incomplete)
        }
    })
}
