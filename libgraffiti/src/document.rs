// x minimal, real DOM should be in JS
//   x just els & text nodes
//   x no comments, fragments, ...
//   x no structural validation (<head> can be removed)
//   x get/set attrs (incl. inline style)
//   - support <style> (<head> only)
//     - insertion/removal (correct order)
//     - text node changes
//
// x part of the public API
//   x so it's high-level and can depend on layout, css, selectors
//     x but text/rendering should be elsewhere
//   x meant to be held inside of something which can do rendering (viewport)
//   x that something will make it available for changes
//   - and then it will just ask the document what needs to be updated

#![allow(unused)]

use std::collections::HashMap;

use crate::util::{Id, Lookup};
use crate::layout::{LayoutEngine, LayoutStyle, LayoutEngineImpl};

use css::{parse_props, CssStyleProp};
use selectors::{parse_selector, MatchingContext};

pub struct Document<Expando> {
    root: Option<NodeId>,
    nodes: Vec<Node>,
    parents: Vec<Option<NodeId>>,
    expandos: Vec<Expando>,
    free_list: Vec<NodeId>,

    layout_engine: LayoutEngineImpl,
    layout_nodes: Vec<<LayoutEngineImpl as LayoutEngine>::LayoutNodeId>
}

impl <Expando: Default> Document<Expando> {
    pub fn new() -> Self {
        Document {
            root: None,
            parents: Vec::new(),
            nodes: Vec::new(),
            expandos: Vec::new(),
            free_list: Vec::new(),

            layout_engine: LayoutEngineImpl::new(),
            layout_nodes: Vec::new()
        }
    }

    // only nodes with this ancestor are considered attached
    // document.ancestors(node).last() == ROOT
    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn set_root(&mut self, node: NodeId) {
        self.root = Some(node)
    }

    pub fn create_element(&mut self, tag_name: &str) -> NodeId {
        self.create_new_node(Node::Element(ElementData {
            tag_name: tag_name.to_owned(),
            attributes: HashMap::new(),
            inline_style: Vec::new(),
            child_nodes: Vec::new(),
        }))
    }

    pub fn tag_name(&self, element: NodeId) -> &str {
        &self.nodes[element].el().tag_name
    }

    pub fn attribute(&self, element: NodeId, name: &str) -> Option<String> {
        let el = self.nodes[element].el();

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
    pub fn set_attribute(&mut self, element: NodeId, name: &str, value: &str) {
        let el = self.nodes[element].el_mut();

        match name {
            // style="..." attr
            // (could be used for el.style.cssText = '...' too)
            "style" => el.inline_style = parse_props(value),

            _ => {
                self.nodes[element].el_mut().attributes.insert(name.to_owned(), value.to_owned());
            }
        }
    }

    pub fn remove_attribute(&mut self, element: NodeId, name: &str) {
        if name == "style" {
            todo!();
            return;
        }

        self.nodes[element].el_mut().attributes.remove(name);
    }

    pub fn child_nodes(&self, element: NodeId) -> &[NodeId] {
        &self.nodes[element].el().child_nodes
    }

    pub fn insert_child(&mut self, parent_element: NodeId, index: usize, child: NodeId) {
        self.nodes[parent_element].el_mut().child_nodes.insert(index, child);
        self.parents[child.index()] = Some(parent_element);

        self.layout_engine.insert_child(self.layout_nodes[parent_element.index()], index, self.layout_nodes[child.index()]);
    }

    pub fn remove_child(&mut self, parent_element: NodeId, child: NodeId) {
        self.nodes[parent_element].el_mut().child_nodes.retain(|ch| *ch != child);
        self.parents[child.index()] = None;

        self.layout_engine.remove_child(self.layout_nodes[parent_element.index()], self.layout_nodes[child.index()]);
    }

    pub fn query_selector(&self, selector: &str, element: Option<NodeId>) -> Option<NodeId> {
        self.query_selector_all(selector, element).get(0).copied()
    }

    pub fn query_selector_all(&self, selector: &str, element: Option<NodeId>) -> Vec<NodeId> {
        match parse_selector(selector) {
            Err(_) => Vec::new(),

            Ok(selector) => {
                // avoid copying
                let att = |el: NodeId, att| self.nodes[el].el().attributes.get(att).map(String::as_str);

                let ctx = MatchingContext {
                    tag_names: &|el| self.tag_name(el),
                    ids: &|el| att(el, "id"),
                    class_names: &|el| att(el, "class"),
                    ancestors: &|el| self.ancestors(el),
                };

                let els = match element {
                    None => self.root.iter().flat_map(|root| std::iter::once(*root).chain(self.descendant_children(*root))).collect(),

                    Some(el) => self.descendant_children(el)
                };

                els.into_iter().filter(|n| ctx.match_selector(&selector, *n)).collect()
            }
        }
    }

    pub fn create_text_node(&mut self, text: &str) -> NodeId {
        self.create_new_node(Node::TextNode(text.to_owned()))
    }

    pub fn text(&self, text_node: NodeId) -> &str {
        self.nodes[text_node].tn()
    }

    pub fn set_text(&mut self, text_node: NodeId, text: &str) {
        *self.nodes[text_node].tn_mut() = text.to_owned();
    }

    // shared for both node types

    pub fn expando(&self, node: NodeId) -> &Expando {
        &self.expandos[node.index()]
    }

    pub fn set_expando(&mut self, node: NodeId, v: Expando) {
        self.expandos[node.index()] = v;
    }

    pub fn parent_node(&self, node: NodeId) -> Option<NodeId> {
        self.ancestors(node).next()
    }

    pub fn free_node(&mut self, node: NodeId) {
        silly!("free node {:?}", node);

        assert!(self.parents[node.index()] == None && self.root != Some(node), "cannot free attached node");

        self.free_list.push(node);
    }

    // helpers

    fn ancestors(&self, node: NodeId) -> Ancestors {
        Ancestors {
            parents: &self.parents,
            next: self.parents[node.index()],
        }
    }

    fn children(&self, element: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.child_nodes(element).iter().copied().filter(move |n| matches!(self.nodes[*n], Node::Element(_)))
    }

    fn descendant_children(&self, element: NodeId) -> Vec<NodeId> {
        self.children(element)
            .flat_map(move |ch| std::iter::once(ch).chain(self.descendant_children(ch))).collect()
    }

    fn create_new_node(&mut self, node: Node) -> NodeId {
        let layout_node = match node {
            Node::Element(_) => self.layout_engine.create_node(&LayoutStyle::DEFAULT),
            Node::TextNode(_) => self.layout_engine.create_leaf(|_| (100., 20.))
        };

        match self.free_list.pop() {
            // try to reuse first
            Some(id) => {
                // replace prev data
                self.nodes[id] = node;
                self.parents[id.index()] = None;
                self.expandos[id.index()] = Default::default();

                self.layout_engine.free_node(self.layout_nodes[id.index()]);
                self.layout_nodes[id.index()] = layout_node;

                id
            }

            // push otherwise
            _ => {
                self.nodes.push(node);
                self.parents.push(None);
                self.expandos.push(Default::default());
                self.layout_nodes.push(layout_node);

                Id::new(self.nodes.len() - 1)
            }
        }
    }
}

pub type NodeId = Id<Node>;

// private from here
// (pubs because of Id<T>)

mod css;
mod selectors;

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
    child_nodes: Vec<NodeId>,
}

pub struct Ancestors<'a> {
    parents: &'a [Option<NodeId>],
    next: Option<NodeId>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        let next = self.next;
        self.next = self.parents[next?.index()];

        next
    }
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
        let d = Document::<()>::new();

        assert_eq!(d.root(), None);
    }

    #[test]
    fn element() {
        let mut d = Document::<()>::new();

        let el = d.create_element("div");
        assert_eq!(d.tag_name(el), "div");
        assert_eq!(d.attribute(el, "id"), None);
        assert_eq!(d.attribute(el, "class"), None);
        assert_eq!(d.child_nodes(el), &[]);

        assert_eq!(d.expando(el), &());
        assert_eq!(d.parent_node(el), None);

        d.set_attribute(el, "id", "app");
        assert_eq!(d.attribute(el, "id"), Some("app".to_string()));

        d.set_attribute(el, "class", "container");
        assert_eq!(d.attribute(el, "class"), Some("container".to_string()));
    }

    #[test]
    fn text_node() {
        let mut d = Document::<()>::new();

        let tn = d.create_text_node("foo");
        assert_eq!(d.text(tn), "foo");

        assert_eq!(d.expando(tn), &());
        assert_eq!(d.parent_node(tn), None);

        d.set_text(tn, "bar");
        assert_eq!(d.text(tn), "bar");
    }

    #[test]
    fn tree() {
        let mut d = Document::<()>::new();

        let html = d.create_element("html");
        let body = d.create_element("body");
        let header = d.create_element("header");
        let div = d.create_element("div");
        let button = d.create_element("button");

        d.insert_child(html, 0, body);
        d.insert_child(div, 0, button);
        d.insert_child(header, 0, div);
        d.insert_child(body, 0, header);

        assert_eq!(d.child_nodes(body), &[header]);
        assert_eq!(d.child_nodes(header), &[div]);
        assert_eq!(d.child_nodes(div), [button]);
    }

    #[test]
    fn inline_style() {
        let mut d = Document::<()>::new();
        let div = d.create_element("div");

        // TODO: css shorthands
        //d.set_attribute(d.div(), "style", "padding: 0");

        d.set_attribute(div, "style", "display: flex");

        // TODO: check inline style
    }

    // TODO: test for resolve_style() - at least inline styles

    // TODO: test for d.attribute("style")
}
