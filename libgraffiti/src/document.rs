// x separate subsystem, useful by itself
//
// x thin subset of native DOM just to implement real DOM in JS
//   - similar naming/types if possible
//   - no structural validation
//     - it's possible to mess the tree with set_inner_html
//   - get/set attrs, inline style
//   - support <style>
//     - only in <head>
//     - insertion/removal (correct order)
//     - text node changes
//
// x part of the public API
//   x so it's high-level and can depend on html, css, selectors
//     x but layout/text/rendering should be elsewhere
//   x meant to be held inside of something which can do layout & rendering (viewport)
//   x that something will make it available for changes
//   - and then it will just ask the document what needs to be updated

// x just els & text nodes
// x no comments, fragments, ...
// - swallow parse error?

#![allow(unused)]

use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::util::{Id, Lookup};

use css::{parse_props, CssStyleProp};
use html::{parse_html, HtmlNode};
use selectors::{parse_selector, MatchingContext};

pub struct Document {
    document_element: Option<NodeRef>,
    nodes: Vec<Node>,
    parents: Vec<Option<NodeRef>>,
    free_list: Rc<RefCell<Vec<NodeId>>>,
}

impl Document {
    pub fn empty_html() -> Self {
        Document::from_html("<html><head></head><body></body></html>").unwrap()
    }

    pub fn from_html(html: &str) -> Result<Self, &str> {
        let mut html_nodes = parse_html(html);

        if html_nodes.len() != 1 {
            return Err("expected exactly 1 root node");
        }

        let mut d = Document {
            document_element: None,
            parents: Vec::new(),
            nodes: Vec::new(),
            free_list: Rc::new(RefCell::new(Vec::new())),
        };

        d.document_element = Some(d.create_node_from_html_node(html_nodes.remove(0)));

        Ok(d)
    }

    // only nodes with this ancestor are considered attached
    // document.ancestors(node).last() == ROOT
    pub fn document_element(&self) -> &NodeRef {
        self.document_element.as_ref().expect("unitialized")
    }

    pub fn create_element(&mut self, tag_name: &str) -> NodeRef {
        self.create_new_node(Node::Element(ElementData {
            tag_name: tag_name.to_owned(),
            attributes: HashMap::new(),
            inline_style: Vec::new(),
            child_nodes: Vec::new(),
        }))
    }

    pub fn tag_name(&self, element: &NodeRef) -> &str {
        &self.nodes[element.id].el().tag_name
    }

    pub fn attribute(&self, element: &NodeRef, name: &str) -> Option<String> {
        let el = self.nodes[element.id].el();

        match name {
            "style" => {
                // TODO: inline styles
                // - empty -> None
                // - otherwise -> stringify
                None
            }

            _ => el.attributes.get(name).map(String::to_owned),
        }
    }

    // TODO: it's now possible to set attribute with invalid name
    // we should either ignore it or return Result<>
    pub fn set_attribute(&mut self, element: &NodeRef, name: &str, value: &str) {
        let el = self.nodes[element.id].el_mut();

        match name {
            // style="..." attr
            // (could be used for el.style.cssText = '...' too)
            "style" => el.inline_style = parse_props(value),

            _ => {
                self.nodes[element.id].el_mut().attributes.insert(name.to_owned(), value.to_owned());
            }
        }
    }

    pub fn remove_attribute(&mut self, element: &NodeRef, name: &str) {
        self.nodes[element.id].el_mut().attributes.remove(name);
    }

    pub fn child_nodes(&self, element: &NodeRef) -> &[NodeRef] {
        &self.nodes[element.id].el().child_nodes
    }

    pub fn insert_child(&mut self, parent_element: &NodeRef, index: usize, child: &NodeRef) {
        self.nodes[parent_element.id].el_mut().child_nodes.insert(index, child.clone());
        self.parents[child.id.0] = Some(parent_element.clone());
    }

    pub fn remove_child(&mut self, parent_element: &NodeRef, child: &NodeRef) {
        self.nodes[parent_element.id].el_mut().child_nodes.retain(|ch| ch != child);
        self.parents[child.id.0] = None;
    }

    pub fn query_selector(&self, selector: &str) -> Option<&NodeRef> {
        self.query_selector_all(selector).get(0).map(|n| *n)
    }

    // TODO: context node for subtree queries (later)
    //       most tricky part will be API because context version
    //       works only on descendants
    //
    //       we could also support `:scope` then (replace with tag name)
    //       https://www.w3.org/TR/selectors-4/#the-scope-pseudo
    pub fn query_selector_all(&self, selector: &str) -> Vec<&NodeRef> {
        match parse_selector(selector) {
            Err(_) => Vec::new(),
            Ok(selector) => {
                let att = |el: &NodeRef, att| self.nodes[el.id].el().attributes.get(att).map(String::as_str);

                let ctx = MatchingContext {
                    tag_names: &|el| self.tag_name(el),
                    ids: &|el| att(el, "id"),
                    class_names: &|el| att(el, "class"),
                    ancestors: &|el| self.ancestors(el),
                };

                let root = self.document_element();

                std::iter::once(root).chain(self.descendants(root)).filter(|n| ctx.match_selector(&selector, n)).collect()
            }
        }
    }

    /*
    pub fn get_inner_html(&self, element: NodeRef) -> String {}
    */

    pub fn set_inner_html(&mut self, element: &NodeRef, html: &str) {
        // free some space first
        self.nodes[element.id].el_mut().child_nodes = Vec::new();

        // create nodes
        let child_nodes: Vec<NodeRef> = parse_html(html).into_iter().map(|n| self.create_node_from_html_node(n)).collect();

        // insert
        self.append_child_nodes(element, &child_nodes);
    }

    pub fn create_text_node(&mut self, data: &str) -> NodeRef {
        self.create_new_node(Node::TextNode(data.to_owned()))
    }

    pub fn data(&self, text_node: &NodeRef) -> &str {
        self.nodes[text_node.id].tn()
    }

    pub fn set_data(&mut self, text_node: &NodeRef, data: &str) {
        *self.nodes[text_node.id].tn_mut() = data.to_owned();
    }

    // shared for both node types

    pub fn parent_node(&self, node: &NodeRef) -> Option<&NodeRef> {
        self.ancestors(node).next()
    }

    pub fn ancestors(&self, node: &NodeRef) -> Ancestors {
        Ancestors {
            parents: &self.parents,
            next: self.parents[node.id.0].as_ref(),
        }
    }

    // helpers

    fn descendants(&self, element: &NodeRef) -> impl Iterator<Item = &NodeRef> + '_ {
        // TODO: compute & cache whole tree and just subslice into it
        //       that way we can avoid linked list

        self.child_nodes(element)
            .iter()
            .flat_map(move |ch| std::iter::once(ch).chain(self.descendants(ch)).collect::<Vec<&NodeRef>>())
    }

    fn create_new_node(&mut self, node: Node) -> NodeRef {
        let id = match self.free_list.try_borrow_mut() {
            // try to reuse first
            Ok(mut ids) if !ids.is_empty() => {
                let id = ids.pop().unwrap();

                // replace prev data
                self.nodes[id] = node;
                self.parents[id.0] = None;

                id
            }

            // push otherwise
            _ => {
                self.nodes.push(node);
                self.parents.push(None);

                Id::new(self.nodes.len() - 1)
            }
        };

        Rc::new(NodeHandle {
            id,
            free_list: self.free_list.clone(),
        })
    }

    fn create_node_from_html_node(&mut self, html_node: HtmlNode) -> NodeRef {
        match html_node {
            HtmlNode::TextNode(data) => self.create_text_node(&data),

            HtmlNode::Element { tag_name, attributes, children } => {
                let el = self.create_element(&tag_name);
                let child_nodes: Vec<NodeRef> = children.into_iter().map(|ch| self.create_node_from_html_node(ch)).collect();

                self.append_child_nodes(&el, &child_nodes);

                // needed because of style attr
                for (k, v) in attributes {
                    self.set_attribute(&el, &k, &v);
                }

                el
            }
        }
    }

    fn append_child_nodes(&mut self, el: &NodeRef, child_nodes: &[NodeRef]) {
        // TODO: append_child?
        for (i, ch) in child_nodes.iter().enumerate() {
            self.insert_child(&el, i, &ch);
        }
    }
}

pub type NodeRef = Rc<NodeHandle>;

pub struct Ancestors<'a> {
    parents: &'a [Option<NodeRef>],
    next: Option<&'a NodeRef>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = &'a NodeRef;

    fn next(&mut self) -> Option<&'a NodeRef> {
        let next = self.next;
        self.next = next.and_then(|_| self.parents[next.unwrap().id.0].as_ref());

        next
    }
}

// private from here
// (pubs because of Id<T>)

mod css;
mod html;
mod selectors;

#[derive(Debug, PartialEq, Eq)]
pub struct NodeHandle {
    id: NodeId,
    free_list: Rc<RefCell<Vec<NodeId>>>,
}

impl std::ops::Drop for NodeHandle {
    fn drop(&mut self) {
        silly!("drop node {:?}", &self.id);

        self.free_list.borrow_mut().push(self.id)
    }
}

type NodeId = Id<Node>;

#[derive(Debug)]
pub enum Node {
    Element(ElementData),
    TextNode(String),
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: HashMap<String, String>,
    inline_style: Vec<CssStyleProp>,
    child_nodes: Vec<NodeRef>,
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

// TODO: test for resolve_style() - at least inline styles

// TODO: test for d.attribute("style")

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_html() {
        let mut d = Document::empty_html();

        let root = d.document_element();
        assert_eq!(d.tag_name(root), "html");

        match d.child_nodes(root) {
            [head, body] => {
                assert_eq!(d.tag_name(head), "head");
                assert_eq!(d.tag_name(body), "body");
            }

            _ => panic!("expected 2 child nodes"),
        }
    }

    #[test]
    fn element() {
        let mut d = Document::empty_html();

        let el = d.create_element("div");
        assert_eq!(d.tag_name(&el), "div");
        assert_eq!(d.attribute(&el, "id"), None);
        assert_eq!(d.attribute(&el, "class"), None);
        assert_eq!(d.child_nodes(&el), &[]);

        assert_eq!(d.parent_node(&el), None);
        assert_eq!(d.ancestors(&el).next(), None);

        d.set_attribute(&el, "id", "app");
        assert_eq!(d.attribute(&el, "id"), Some("app".to_string()));

        d.set_attribute(&el, "class", "container");
        assert_eq!(d.attribute(&el, "class"), Some("container".to_string()));
    }

    #[test]
    fn text_node() {
        let mut d = Document::empty_html();

        let tn = d.create_text_node("foo");
        assert_eq!(d.data(&tn), "foo");

        assert_eq!(d.parent_node(&tn), None);
        assert_eq!(d.ancestors(&tn).next(), None);

        d.set_data(&tn, "bar");
        assert_eq!(d.data(&tn), "bar");
    }

    #[test]
    fn query_selector() {
        let mut d = Document::empty_html();
        let root = d.document_element();

        let head = d.query_selector("head").expect("no head");
        let body = d.query_selector("body").expect("no body");

        assert_eq!(d.tag_name(head), "head");
        assert_eq!(d.tag_name(body), "body");

        //assert_eq!(d.query_selector_all(root, "html > *"), vec![head, body]);

        //assert_eq!()
    }

    #[test]
    fn tree() {
        let mut d = Document::empty_html();

        let html = d.document_element().clone();
        let body = d.query_selector("body").expect("no body").clone();
        let header = d.create_element("header");
        let div = d.create_element("div");
        let button = d.create_element("button");

        d.insert_child(&div, 0, &button);
        d.insert_child(&header, 0, &div);
        d.insert_child(&body, 0, &header);

        println!("{:#?}", d.nodes);

        assert_eq!(d.child_nodes(&body), &[header.clone()]);
        assert_eq!(d.child_nodes(&header), &[div.clone()]);
        assert_eq!(d.child_nodes(&div), [button.clone()]);

        assert_eq!(d.ancestors(&button).collect::<Vec<_>>(), vec![&div, &header, &body, &html]);
    }

    #[test]
    fn inline_style() {
        let mut d = Document::empty_html();
        let root = d.document_element().clone();

        d.insert_child(div, 0, button);
        d.insert_child(header, 0, div);
        d.insert_child(Document::ROOT, 0, header);

        d.set_attribute(&root, "style", "display: flex");

        //assert_eq!(d.child_nodes(&root), &[header.clone()]);
    }
}
