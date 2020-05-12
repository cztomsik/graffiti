// x super-simple subset of HTML
// x meant for markdown (inner_html) & testing/prototyping
// x el/text node only
//
// x no end tag matching (later)
// x no self-closing (later)
// x no bool/num attrs (later)
// x no entities/quoting (later)

#![allow(unused)]

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Element {
        tag_name: String,
        attributes: HashMap<String, String>,
        children: Vec<HtmlNode>,
    },

    TextNode(String),
}

impl FromStr for HtmlNode {
    type Err = pom::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::node().parse(s.as_bytes())
    }
}

pub fn parse_html(html: &str) -> Result<Vec<HtmlNode>, pom::Error> {
    (parse::doctype().opt() * parse::node().repeat(1..).name("nodes")).parse(html.as_bytes())
}

mod parse {
    use super::*;
    use pom::char_class::{alphanum, space};
    use pom::parser::*;

    pub fn doctype<'a>() -> Parser<'a, u8, ()> {
        seq(b"<!DOCTYPE") * none_of(b">").repeat(0..).discard() - sym(b'>')
    }

    pub fn node<'a>() -> Parser<'a, u8, HtmlNode> {
        let el_open = sym(b'<') * ident().convert(String::from_utf8) - is_a(space).repeat(0..);
        let el_close = seq(b"</") * ident() * sym(b'>');
        let el = el_open + attributes() - sym(b'>') + children() - el_close;

        let element = el.map(|((tag_name, attributes), children)| HtmlNode::Element { tag_name, attributes, children });
        let text_node = none_of(b"<>").repeat(1..).convert(String::from_utf8).map(HtmlNode::TextNode);

        element | text_node
    }

    fn children<'a>() -> Parser<'a, u8, Vec<HtmlNode>> {
        call(node).repeat(0..)
    }

    fn attributes<'a>() -> Parser<'a, u8, HashMap<String, String>> {
        let name = ident().convert(String::from_utf8);
        // TODO: entities/quoting
        // (replace in result could be enough)
        let value = (sym(b'"') * none_of(b"\"").repeat(0..) - sym(b'"')).convert(String::from_utf8);
        let attr = name - sym(b'=') + value;

        list(attr, sym(b' ').repeat(0..)).map(|entries| entries.into_iter().collect())
    }

    fn ident<'a>() -> Parser<'a, u8, Vec<u8>> {
        is_a(alphanum_dash).repeat(1..)
    }

    fn alphanum_dash(b: u8) -> bool {
        alphanum(b) || b == b'-'
    }

    #[cfg(test)]
    mod tests {
        use super::HtmlNode::*;
        use super::*;

        #[test]
        fn parse_attributes() {
            assert_eq!(attributes().parse(b""), Ok(HashMap::new()));

            // one attr
            assert_eq!(attributes().parse(b"class=\"btn\""), Ok(vec![("class".to_string(), "btn".to_string())].into_iter().collect()));

            // many
            assert_eq!(
                attributes().parse(b"id=\"app\" class=\"container\""),
                Ok(vec![("id".to_string(), "app".to_string()), ("class".to_string(), "container".to_string())].into_iter().collect())
            );
        }

        #[test]
        fn parse_node() {
            assert_eq!("foo".parse(), Ok(TextNode("foo".to_string())));

            assert_eq!(
                "<div></div>".parse(),
                Ok(Element {
                    tag_name: "div".to_string(),
                    attributes: HashMap::new(),
                    children: Vec::new(),
                })
            );

            // alphanum
            assert_eq!(
                "<h1></h1>".parse(),
                Ok(Element {
                    tag_name: "h1".to_string(),
                    attributes: HashMap::new(),
                    children: Vec::new(),
                })
            );
        }

        #[test]
        fn parse_simple() {
            assert_eq!(
                "<div><h1>Hello</h1></div>".parse(),
                Ok(Element {
                    tag_name: "div".to_string(),
                    attributes: HashMap::new(),
                    children: vec![Element {
                        tag_name: "h1".to_string(),
                        attributes: HashMap::new(),
                        children: vec![TextNode("Hello".to_string())]
                    }]
                })
            );
        }

        #[test]
        fn parse_complex() {
            let html = r#"
                <div id="app" class="container">
                    <button class="btn">button</button>
                </div>
            "#;

            assert_eq!(
                html.trim().parse(),
                Ok(Element {
                    tag_name: "div".to_string(),
                    attributes: vec![("id".to_string(), "app".to_string()), ("class".to_string(), "container".to_string())].into_iter().collect(),
                    children: vec![
                        TextNode("\n                    ".to_string()),
                        Element {
                            tag_name: "button".to_string(),
                            attributes: vec![("class".to_string(), "btn".to_string())].into_iter().collect(),
                            children: vec![TextNode("button".to_string())]
                        },
                        TextNode("\n                ".to_string())
                    ],
                })
            );
        }

        #[test]
        fn parse_doctype() {
            assert!(doctype().parse(b"<!DOCTYPE html>").is_ok())
        }

        #[test]
        fn parse_html() {
            assert_eq!(
                super::parse_html(" <div></div>"),
                Ok(vec![
                    TextNode(" ".to_string()),
                    Element {
                        tag_name: "div".to_string(),
                        attributes: HashMap::new(),
                        children: Vec::new(),
                    }
                ])
            );
        }
    }
}
