use super::parsing::{alphanum, alphanum_dash, any, is_a, none_of, one_of, prev, seq, skip, sym, Token};

// different from https://drafts.csswg.org/css-syntax/#tokenization
// (main purpose here is to strip comments and to keep strings together)
pub fn tokenize(input: &[u8]) -> Vec<Token> {
    let comment = seq(b"/*") * (!seq(b"*/") * skip(1)).repeat(0..) - seq(b"*/");
    let space = one_of(b" \t\r\n").discard().repeat(1..).map(|_| &b" "[..]);
    let hex_or_id = prev(1) * sym(b'#') * is_a(alphanum).repeat(1..).collect();
    let num = (sym(b'-').opt() + one_of(b".0123456789").repeat(1..)).collect();
    let ident = is_a(alphanum_dash).repeat(1..).collect();
    let string1 = (sym(b'\'') + none_of(b"'").repeat(0..) + sym(b'\'')).collect();
    let string2 = (sym(b'"') + none_of(b"\"").repeat(0..) + sym(b'"')).collect();
    let other = any().collect();

    // spaces are "normalized" but they still can appear multiple times because of stripped comments
    let token = comment.opt() * (space | hex_or_id | num | ident | string1 | string2 | other);
    let tokens = token.convert(std::str::from_utf8).repeat(0..).parse(input).unwrap();

    // strip whitespace except for selectors & multi-values
    // TODO: this was easier than combinators
    let (mut res, mut keep_space) = (Vec::new(), false);
    for (i, &t) in tokens.iter().enumerate() {
        if t == " " {
            if !keep_space {
                continue;
            }

            if let Some(&next) = tokens.get(i + 1) {
                if !(alphanum_dash(next.as_bytes()[0]) || next == "." || next == "#" || next == "*") {
                    continue;
                }
            }
        }

        res.push(t);
        keep_space = alphanum_dash(t.as_bytes()[0]) || t == "*" || t == "]";
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(b""), Vec::<Token>::new());
        assert_eq!(tokenize(b" "), Vec::<Token>::new());
        assert_eq!(tokenize(b" /**/ /**/ "), Vec::<Token>::new());

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

        assert_eq!(tokenize(b"-webkit-xxx"), vec!["-webkit-xxx"]);
        assert_eq!(tokenize(b"--var"), vec!["--var"]);

        assert_eq!(
            tokenize(b"parent .btn { /**/ padding: 10px }"),
            vec!["parent", " ", ".", "btn", "{", "padding", ":", "10", "px", "}"]
        );

        assert_eq!(
            tokenize(b"prop: url('foo bar')"),
            vec!["prop", ":", "url", "(", "'foo bar'", ")"],
        );
        assert_eq!(tokenize(b"[foo=\"bar\"]"), vec!["[", "foo", "=", "\"bar\"", "]"]);

        assert_eq!(
            tokenize(b"@media { a b { left: 10% } }"),
            vec!["@", "media", "{", "a", " ", "b", "{", "left", ":", "10", "%", "}", "}"]
        );

        assert_eq!(tokenize(b"/**/ a /**/ b {}"), vec!["a", " ", "b", "{", "}"]);

        let ua = include_bytes!("../../resources/ua.css");
        let _tokens = tokenize(ua);

        // println!("{:#?}", _tokens);
    }
}
