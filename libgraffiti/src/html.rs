// TODO: single-quote attributes, entities, self-closing, implicit closing? (meta/link)

use pom::char_class::{alphanum, multispace};
use pom::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode<'a> {
    Element {
        local_name: &'a str,
        attributes: Vec<(&'a str, &'a str)>,
        children: Vec<HtmlNode<'a>>,
    },
    Text(&'a str),
    Comment(&'a str),
}

pub type ParseError = pom::Error;

pub fn parse_html<'a>(html: &'a str) -> Result<Vec<HtmlNode<'a>>, ParseError> {
    (junk() * nodes() - pom::parser::end()).parse(html.as_bytes())
}

type Parser<'a, T> = pom::parser::Parser<'a, u8, T>;

fn junk<'a>() -> Parser<'a, ()> {
    space() * (seq(b"<!") * none_of(b">").repeat(0..) * sym(b'>')).opt() * space().discard()
}

fn nodes<'a>() -> Parser<'a, Vec<HtmlNode<'a>>> {
    node().repeat(0..)
}

pub fn node<'a>() -> Parser<'a, HtmlNode<'a>> {
    comment() | element() | text_node()
}

pub fn comment<'a>() -> Parser<'a, HtmlNode<'a>> {
    seq(b"<!--")
        * (!seq(b"-->") * take(1))
            .repeat(0..)
            .collect()
            .convert(std::str::from_utf8)
            .map(HtmlNode::Comment)
        - seq(b"-->")
}

fn element<'a>() -> Parser<'a, HtmlNode<'a>> {
    let el_open = sym(b'<') * ident() - space();

    el_open
        >> |local_name| {
            (attributes() - sym(b'>') + nodes() - seq(b"</") - seq(local_name.as_bytes()) - sym(b'>')).map(
                move |(attributes, children)| HtmlNode::Element {
                    local_name,
                    attributes,
                    children,
                },
            )
        }
}

fn text_node<'a>() -> Parser<'a, HtmlNode<'a>> {
    none_of(b"<>")
        .repeat(1..)
        .collect()
        .convert(std::str::from_utf8)
        .map(HtmlNode::Text)
}

fn attributes<'a>() -> Parser<'a, Vec<(&'a str, &'a str)>> {
    // TODO: entities/quoting
    let string = sym(b'"') * none_of(b"\"").repeat(0..).collect().convert(std::str::from_utf8) - sym(b'"');
    let attr = ident() + (sym(b'=') * string).opt().map(Option::unwrap_or_default);

    list(attr, is_a(multispace).discard().repeat(1..))
}

fn ident<'a>() -> Parser<'a, &'a str> {
    is_a(alphanum_dash).repeat(1..).collect().convert(std::str::from_utf8)
}

fn space<'a>() -> Parser<'a, &'a [u8]> {
    is_a(multispace).repeat(0..).collect()
}

fn alphanum_dash(b: u8) -> bool {
    alphanum(b) || b == b'-'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_comment() {
        assert_eq!(comment().parse(b"<!---->"), Ok(HtmlNode::Comment("")));
        assert_eq!(comment().parse(b"<!-- foo -->"), Ok(HtmlNode::Comment(" foo ")));
    }

    #[test]
    fn parse_element() {
        assert_eq!(
            element().parse(b"<div><div></div></div>"),
            Ok(HtmlNode::Element {
                local_name: "div",
                attributes: vec![],
                children: vec![HtmlNode::Element {
                    local_name: "div",
                    attributes: vec![],
                    children: vec![]
                }],
            })
        );

        assert_eq!(
            element().parse(br#"<my-app foo="bar" baz></my-app>"#),
            Ok(HtmlNode::Element {
                local_name: "my-app",
                attributes: vec![("foo", "bar"), ("baz", "")],
                children: vec![]
            })
        );

        assert!(element().parse(br#"<a>"#).is_err());
        assert!(element().parse(br#"<a></b>"#).is_err());
    }

    #[test]
    fn parse_attributes() {
        assert_eq!(attributes().parse(b""), Ok(vec![]));
        assert_eq!(attributes().parse(b"disabled"), Ok(vec![("disabled", "")]));
        assert_eq!(attributes().parse(b"class=\"btn\""), Ok(vec![("class", "btn")]));
        assert_eq!(
            attributes().parse(b"id=\"app\" class=\"container\""),
            Ok(vec![("id", "app"), ("class", "container")])
        );
    }

    #[test]
    fn parse_node() {
        assert_eq!(node().parse(b"foo"), Ok(HtmlNode::Text("foo")));

        assert_eq!(node().parse(b"<!-- foo -->"), Ok(HtmlNode::Comment(" foo ")));

        assert_eq!(
            node().parse(b"<div></div>"),
            Ok(HtmlNode::Element {
                local_name: "div",
                attributes: vec![],
                children: vec![],
            })
        );
    }

    #[test]
    fn parse_html() {
        assert_eq!(
            super::parse_html(" <div></div>"),
            Ok(vec![HtmlNode::Element {
                local_name: "div",
                attributes: vec![],
                children: Vec::new(),
            }])
        );

        assert_eq!(
            super::parse_html(
                r#"
                <div id="app" class="container">
                    <button class="btn">button</button>
                </div>
            "#
            ),
            Ok(vec![
                HtmlNode::Element {
                    local_name: "div",
                    attributes: vec![("id", "app"), ("class", "container")].into_iter().collect(),
                    children: vec![
                        HtmlNode::Text("\n                    "),
                        HtmlNode::Element {
                            local_name: "button",
                            attributes: vec![("class", "btn")].into_iter().collect(),
                            children: vec![HtmlNode::Text("button")]
                        },
                        HtmlNode::Text("\n                ")
                    ],
                },
                HtmlNode::Text("\n            ")
            ])
        );

        assert!(super::parse_html(
            r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <meta charset="utf-8"></meta>
                    <meta http-equiv="X-UA-Compatible" content="IE=edge"></meta>
                    <title>Hello</title>
                    <meta name="viewport" content="width=device-width, initial-scale=1"></meta>
                    <link rel="stylesheet" type="text/css" media="screen" href="style.css"></link>
                    <script src="main.js"></script>
                </head>
                <body>
                    Hello                
                </body>
                </html>
            "#
        )
        .is_ok());
    }
}
