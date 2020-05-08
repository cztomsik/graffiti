// x part of the public API
//   x so it's high-level and can depend on html, css, selectors
//     x but layout/text/rendering should be elsewhere
//   x meant to be held inside of something which can do layout & rendering (viewport)
//   x that something will make it available for changes
//   - and then it will just ask the document what needs to be updated

// x just els & text nodes
// x no comments, fragments, ...
// x only id/class attributes
// - no special treatment for <style>
//   - use `insert_style_sheet(index, sheet_str)` instead
//   - that way we don't need to observe for:
//     - element insertion/removal
//     - correct order
//     - text node changes

#![allow(unused)]

use crate::commons::{Id, Lookup};
use crate::html::{parse_html, HtmlNode};
use crate::selectors::MatchingContext;

pub struct Document {
    nodes: Vec<Node>,
    parents: Vec<Option<NodeId>>,
}

impl Document {
    // only nodes with ROOT ancestor are considered attached
    // document.ancestors(node).last() == ROOT
    pub const ROOT: NodeId = Id::new(0);

    pub fn new() -> Self {
        let mut d = Document {
            nodes: Vec::new(),
            parents: Vec::new(),
        };

        // not 100% sure but all of this is html-like and it's not fun
        // to write `Document::new("html")` everywhere
        // so maybe later
        d.create_element("html");

        d
    }

    pub fn create_element(&mut self, tag_name: &str) -> NodeId {
        self.push_node(Node::Element(ElementData {
            tag_name: tag_name.to_string(),
            identifier: String::new(),
            class_name: String::new(),
            children: Vec::new(),
        }))
    }

    pub fn tag_name(&self, element: NodeId) -> &str {
        &self.nodes[element].el().tag_name
    }

    pub fn identifier(&self, element: NodeId) -> &str {
        &self.nodes[element].el().identifier
    }

    pub fn set_identifier(&mut self, element: NodeId, identifier: &str) {
        self.nodes[element].el_mut().identifier = identifier.to_string()
    }

    pub fn class_name(&self, element: NodeId) -> &str {
        &self.nodes[element].el().class_name
    }

    pub fn set_class_name(&mut self, element: NodeId, class_name: &str) {
        self.nodes[element].el_mut().class_name = class_name.to_string()
    }

    pub fn children(&self, element: NodeId) -> &[NodeId] {
        &self.nodes[element].el().children
    }

    pub fn insert_child(&mut self, parent: NodeId, index: usize, child: NodeId) {
        self.nodes[parent].el_mut().children.insert(index, child);
        self.parents[child.0] = Some(parent);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.nodes[parent].el_mut().children.retain(|ch| *ch != child);
        self.parents[child.0] = None;
    }

    // TODO: maybe it could just insert the html as text node if it cannot be parsed
    // (this is what browsers do but it probably should be elsewhere)
    pub fn set_inner_html(&mut self, parent: NodeId, html: &str) -> Result<(), pom::Error> {
        self.nodes[parent].el_mut().children = Vec::new();

        self.push_html_nodes(parent, &parse_html(html)?);

        Ok(())
    }

    fn push_html_nodes(&mut self, parent: NodeId, html_nodes: &[HtmlNode]) -> Vec<NodeId> {
        // TODO: append_child?
        html_nodes
            .iter()
            .rev()
            .map(|n| {
                let node = match n {
                    HtmlNode::TextNode(s) => self.create_text_node(&s),
                    HtmlNode::Element { tag_name, attributes, children } => {
                        let el = self.create_element(&tag_name);

                        self.set_identifier(el, attributes.get("id").unwrap_or(&String::new()));
                        self.set_class_name(el, attributes.get("class").unwrap_or(&String::new()));

                        self.push_html_nodes(el, &children);

                        el
                    }
                };

                self.insert_child(parent, 0, node);

                node
            })
            .collect()
    }

    pub fn create_text_node(&mut self, text: &str) -> NodeId {
        self.push_node(Node::TextNode(text.to_string()))
    }

    pub fn text(&self, text_node: NodeId) -> &str {
        self.nodes[text_node].tn()
    }

    pub fn set_text(&mut self, text_node: NodeId, text: &str) {
        *self.nodes[text_node].tn_mut() = text.to_string();
    }

    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.ancestors(node).next()
    }

    pub fn ancestors(&self, node: NodeId) -> Ancestors {
        Ancestors(&self.parents, self.parents[node.0])
    }

    pub fn query_selector(&self, selector: &str) -> Option<NodeId> {
        self.query_selector_all(selector).first().map(|it| *it)
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<NodeId> {
        match selector.parse() {
            Err(_) => Vec::new(),
            Ok(selector) => {
                let ctx = MatchingContext {
                    tag_names: &|el| self.tag_name(el),
                    ids: &|el| self.identifier(el),
                    class_names: &|el| self.class_name(el),
                    ancestors: &|el| self.ancestors(el),
                };

                // TODO: attached only
                self.nodes
                    .iter()
                    .enumerate()
                    .filter_map(|(index, n)| {
                        if let Node::Element(_) = n {
                            let id = Id::new(index);

                            if ctx.match_selector(&selector, id) {
                                Some(id)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        }
    }

    fn push_node(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        self.parents.push(None);

        Id::new(self.nodes.len() - 1)
    }
}

pub type NodeId = Id<Node>;

pub struct Ancestors<'a>(&'a [Option<NodeId>], Option<NodeId>);

impl<'a> Iterator for Ancestors<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        let next = self.1;
        self.1 = next.and_then(|_| self.0[self.1.unwrap().0]);

        next
    }
}

// private from here
// (pubs because of Id<T>)

pub enum Node {
    Element(ElementData),
    TextNode(String),
}

pub struct ElementData {
    tag_name: String,
    identifier: String,
    class_name: String,
    children: Vec<NodeId>,
}

// TODO: macro?
impl Node {
    fn el(&self) -> &ElementData {
        if let Node::Element(data) = self {
            data
        } else {
            panic!("not an element")
        }
    }

    fn el_mut(&mut self) -> &mut ElementData {
        if let Node::Element(data) = self {
            data
        } else {
            panic!("not an element")
        }
    }

    fn tn(&self) -> &str {
        if let Node::TextNode(data) = self {
            data
        } else {
            panic!("not a text node")
        }
    }

    fn tn_mut(&mut self) -> &mut String {
        if let Node::TextNode(data) = self {
            data
        } else {
            panic!("not a text node")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut d = Document::new();

        assert_eq!(d.tag_name(Document::ROOT), "html");
    }

    #[test]
    fn element() {
        let mut d = Document::new();

        let el = d.create_element("div");
        assert_eq!(d.tag_name(el), "div");
        assert_eq!(d.identifier(el), "");
        assert_eq!(d.class_name(el), "");
        assert_eq!(d.children(el), &[]);

        assert_eq!(d.parent(el), None);
        assert_eq!(d.ancestors(el).next(), None);

        d.set_identifier(el, "test");
        assert_eq!(d.identifier(el), "test");

        d.set_class_name(el, "test");
        assert_eq!(d.class_name(el), "test");
    }

    #[test]
    fn text_node() {
        let mut d = Document::new();

        let tn = d.create_text_node("foo");
        assert_eq!(d.text(tn), "foo");

        assert_eq!(d.parent(tn), None);
        assert_eq!(d.ancestors(tn).next(), None);

        d.set_text(tn, "bar");
        assert_eq!(d.text(tn), "bar");
    }

    #[test]
    fn tree() {
        let mut d = Document::new();

        let header = d.create_element("header");
        let div = d.create_element("div");
        let button = d.create_element("button");

        d.insert_child(div, 0, button);
        d.insert_child(header, 0, div);
        d.insert_child(Document::ROOT, 0, header);

        assert_eq!(d.children(Document::ROOT), &[header]);
        assert_eq!(d.children(header), &[div]);
        assert_eq!(d.children(div), &[button]);

        assert_eq!(d.ancestors(button).collect::<Vec<_>>(), &[div, header, Document::ROOT]);
    }
}
