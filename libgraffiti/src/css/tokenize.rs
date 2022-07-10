use super::parsing::{alphanum_dash, decimal, space, Token};

// different from https://drafts.csswg.org/css-syntax/#tokenization
// (main purpose here is to strip comments and to keep strings together)
pub fn tokenize(input: &[u8]) -> Vec<Token> {
    Tokenizer(input, 0, false).collect()
}

struct Tokenizer<'a>(&'a [u8], usize, bool);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(input, pos, keep_space) = *self;
        let (&ch, (prev, rest)) = (input.get(pos)?, input.split_at(pos));

        let scan = |i, f: fn(u8) -> bool| rest.iter().skip(i).take_while(|&&ch| f(ch)).count() + i;
        let comment = || rest.windows(2).position(|w| w == b"*/");
        let string = |q| rest.windows(2).position(|w| w[1] == q && w[0] != b'\\');
        let mut take = |n| {
            *self = Self(input, pos + n, alphanum_dash(rest[0]) || matches!(rest[0], b'*' | b']'));
            std::str::from_utf8(&rest[0..n]).ok()
        };

        match ch {
            b'-' | b'+' if decimal(*rest.get(1)?) => take(scan(2, decimal)),
            // b'-' if alphanum_dash(*rest.get(1)?) => take(scan(2, alphanum_dash)),
            _ if alphanum_dash(ch) && prev.last() == Some(&b'#') => take(scan(1, alphanum_dash)),
            _ if decimal(ch) => take(scan(1, decimal)),
            _ if alphanum_dash(ch) => take(scan(1, alphanum_dash)),
            b';' => take(scan(1, |ch| ch == b';' || space(ch))).map(|_| ";"),
            _ if space(ch) => take(1).and_then(|_| {
                if keep_space && (alphanum_dash(*rest.get(1)?) || matches!(*rest.get(1)?, b'.' | b'#' | b'*')) {
                    Some(" ")
                } else {
                    self.next()
                }
            }),
            b'\'' | b'"' => take(2 + string(ch)?),
            _ if rest.starts_with(b"/*") => take(2 + comment()?).and_then(|_| self.next()),
            _ if rest.starts_with(b"!important") => take(10),
            _ => take(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(b""), Vec::<Token>::new());
        assert_eq!(tokenize(b" "), Vec::<Token>::new());
        assert_eq!(tokenize(b" \n \t \n "), Vec::<Token>::new());
        assert_eq!(tokenize(b"/* */"), Vec::<Token>::new());
        assert_eq!(tokenize(b" /**/ /**/ "), Vec::<Token>::new());

        assert_eq!(tokenize(b";"), vec![";"]);
        assert_eq!(tokenize(b";;"), vec![";"]);
        assert_eq!(tokenize(b";; ;;"), vec![";"]);
        assert_eq!(tokenize(b" ; ; ; ;"), vec![";"]);

        assert_eq!(tokenize(b"block"), vec!["block"]);
        assert_eq!(tokenize(b"10px"), vec!["10", "px"]);
        assert_eq!(tokenize(b"-10px"), vec!["-10", "px"]);
        assert_eq!(tokenize(b"ident2"), vec!["ident2"]);
        assert_eq!(tokenize(b"ff0"), vec!["ff0"]);
        assert_eq!(tokenize(b"00f"), vec!["00", "f"]);
        assert_eq!(tokenize(b"#00f"), vec!["#", "00f"]);
        assert_eq!(tokenize(b"0 0 10px 0"), vec!["0", " ", "0", " ", "10", "px", " ", "0"]);

        assert_eq!(tokenize(b"a b"), vec!["a", " ", "b"]);
        assert_eq!(tokenize(b".a .b"), vec![".", "a", " ", ".", "b"]);
        assert_eq!(
            tokenize(b" a .b #c *"),
            vec!["a", " ", ".", "b", " ", "#", "c", " ", "*"]
        );

        assert_eq!(tokenize(b"!important"), vec!["!important"]);
        assert_eq!(tokenize(b"! important"), vec!["!", "important"]);
        assert_eq!(tokenize(b"-webkit-xxx"), vec!["-webkit-xxx"]);
        assert_eq!(tokenize(b"--var"), vec!["--var"]);

        assert_eq!(
            tokenize(b"parent .btn { /**/ padding: 10px }"),
            vec!["parent", " ", ".", "btn", "{", "padding", ":", "10", "px", "}"]
        );

        assert_eq!(tokenize(b"'foo'"), vec!["'foo'"],);
        assert_eq!(tokenize(b"\"foo bar\""), vec!["\"foo bar\""],);
        assert_eq!(tokenize(b"'\\''"), vec!["'\\''"],);
        assert_eq!(
            tokenize(b"prop: url('foo bar')"),
            vec!["prop", ":", "url", "(", "'foo bar'", ")"],
        );
        assert_eq!(tokenize(b"[foo=\"bar\"]"), vec!["[", "foo", "=", "\"bar\"", "]"]);

        assert_eq!(
            tokenize(b"@media { a b { left: 10% } }"),
            vec!["@", "media", "{", "a", " ", "b", "{", "left", ":", "10", "%", "}", "}"]
        );

        //assert_eq!(tokenize(b"/**/ a /**/ b {}"), vec!["a", " ", "b", "{", "}"]);

        let ua = include_bytes!("../../resources/ua.css");
        let _tokens = tokenize(ua);

        // println!("{:#?}", _tokens);
    }
}
