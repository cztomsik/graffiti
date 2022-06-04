// TODO: stabilize
//
// note that some checks are only present in debug builds
// these might not panic but the result is undefined:
// - calling drop_node(Document::ROOT)
// - calling anything on already dropped nodes
// - calling local_name/style/...() for non-element nodes
// - calling text() for non-text nodes
// - generally, writes are checked but reads are not

use crate::css::{MatchingContext, Selector, Style};
use crate::util::Atom;
use std::collections::HashMap;
use std::fmt;
use std::slice::Iter;

pub type NodeId = usize;
pub type LocalName = Atom;
pub type AttrName = Atom;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Element = 1,
    Text = 3,
    Document = 9,
}

pub struct Document {
    node_types: Vec<NodeType>,
    parents: Vec<Option<NodeId>>,
    children: Vec<Vec<NodeId>>,
    local_names: HashMap<NodeId, LocalName>,
    attributes: HashMap<NodeId, Vec<(AttrName, String)>>,
    styles: HashMap<NodeId, Style>,
    texts: HashMap<NodeId, String>,
}

impl Document {
    pub const ROOT: NodeId = 0;

    pub fn new() -> Self {
        let mut doc = Document {
            node_types: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            local_names: HashMap::new(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
            texts: HashMap::new(),
        };

        assert_eq!(Self::ROOT, doc.create_node(NodeType::Document));

        doc
    }

    pub fn node_type(&self, node: NodeId) -> NodeType {
        self.node_types[node]
    }

    pub fn parent_node(&self, node: NodeId) -> Option<NodeId> {
        self.parents[node]
    }

    pub fn children(&self, parent: NodeId) -> &[NodeId] {
        &self.children[parent]
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        assert_ne!(self.node_type(parent), NodeType::Text);
        assert_eq!(self.parent_node(child), None);

        self.children[parent].push(child);
        self.parents[child] = Some(parent);
    }

    pub fn insert_before(&mut self, parent: NodeId, child: NodeId, before: NodeId) {
        assert_ne!(self.node_type(parent), NodeType::Text);
        assert_eq!(self.parent_node(child), None);
        assert_eq!(self.parent_node(before), Some(parent));

        let index = self.children[parent].iter().position(|ch| *ch == before).unwrap();
        self.children[parent].insert(index, child);
        self.parents[child] = Some(parent);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        assert_ne!(self.node_type(parent), NodeType::Text);
        assert_eq!(self.parent_node(child), Some(parent));

        self.children[parent].retain(|ch| *ch != child);
        self.parents[child] = None;
    }

    pub fn element_matches(&self, element: NodeId, selector: &str) -> bool {
        match Selector::parse(selector) {
            Ok(sel) => self.match_selector(&sel, element).is_some(),
            _ => false,
        }
    }

    pub fn query_selector(&self, node: NodeId, selector: &str) -> Option<NodeId> {
        self.query_selector_all(node, selector).next()
    }

    pub fn query_selector_all(&self, node: NodeId, selector: &str) -> impl Iterator<Item = NodeId> + '_ {
        todo!();
        Vec::new().iter().copied()
        // let selector = Selector::parse(selector).unwrap_or_else(|_| Selector::unsupported());
        // let descendants = Descendants {
        //     document: self,
        //     stack: vec![self.children(node).iter()],
        // };

        // descendants
        //     .filter(move |&e| self.node_type(e) == NodeType::Element && self.match_selector(&selector, node).is_some())
    }

    pub fn create_element(&mut self, local_name: impl Into<LocalName>) -> NodeId {
        let id = self.create_node(NodeType::Element);
        self.local_names.insert(id, local_name.into());
        self.attributes.insert(id, Vec::new());

        id
    }

    pub fn local_name(&self, element: NodeId) -> LocalName {
        debug_assert_eq!(self.node_type(element), NodeType::Element);

        self.local_names[&element]
    }

    pub fn attribute_names(&self, element: NodeId) -> impl Iterator<Item = AttrName> + '_ {
        debug_assert_eq!(self.node_type(element), NodeType::Element);

        self.attributes[&element].iter().map(|(k, _)| *k)
    }

    pub fn attribute(&self, element: NodeId, attr: impl Into<AttrName>) -> Option<&str> {
        debug_assert_eq!(self.node_type(element), NodeType::Element);

        let attr = attr.into();
        self.attributes[&element]
            .iter()
            .find(|(a, _)| attr == *a)
            .map(|(_, v)| &**v)
    }

    pub fn set_attribute(&mut self, element: NodeId, attr: impl Into<AttrName>, value: &str) {
        assert_eq!(self.node_type(element), NodeType::Element);

        let attr = attr.into();
        let attrs = self.attributes.get_mut(&element).unwrap();

        if let Some(a) = attrs.iter_mut().find(|(a, _)| attr == *a) {
            a.1 = value.into();
        } else {
            attrs.push((attr.into(), value.into()));
        }
    }

    pub fn remove_attribute(&mut self, element: NodeId, attr: impl Into<AttrName>) {
        assert_eq!(self.node_type(element), NodeType::Element);

        let attr = attr.into();
        let attrs = self.attributes.get_mut(&element).unwrap();

        attrs.retain(|(a, _)| attr != *a);
    }

    pub fn style(&self, element: NodeId) -> Option<&Style> {
        debug_assert_eq!(self.node_type(element), NodeType::Element);

        self.styles.get(&element)
    }

    // TODO: Into<Option<Style>>
    pub fn set_style(&mut self, element: NodeId, style: &str) {
        assert_eq!(self.node_type(element), NodeType::Element);

        self.styles.insert(element, Style::parse(style).unwrap_or_default());
    }

    pub fn create_text_node(&mut self, text: &str) -> NodeId {
        let id = self.create_node(NodeType::Text);
        self.texts.insert(id, text.to_owned());

        id
    }

    pub fn text(&self, text_node: NodeId) -> &str {
        debug_assert_eq!(self.node_type(text_node), NodeType::Text);

        &self.texts[&text_node]
    }

    pub fn set_text(&mut self, text_node: NodeId, text: &str) {
        assert_eq!(self.node_type(text_node), NodeType::Text);

        self.texts.insert(text_node, text.to_owned());
    }

    pub fn drop_node(&mut self, node: NodeId) {
        todo!()
    }

    // helpers
    fn create_node(&mut self, node_type: NodeType) -> NodeId {
        let id = self.node_types.len();

        self.node_types.push(node_type);
        self.parents.push(None);
        self.children.push(Vec::new());

        id
    }
}

struct Descendants<'a> {
    document: &'a Document,
    stack: Vec<Iter<'a, NodeId>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        match self.stack.last_mut()?.next() {
            Some(&node) => {
                self.stack.push(self.document.children(node).iter());
                Some(node)
            }
            None => {
                self.stack.pop();
                self.next()
            }
        }
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Document").finish()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchingContext for Document {
    type ElementRef = NodeId;

    fn parent_element(&self, element: NodeId) -> Option<NodeId> {
        match self.parent_node(element) {
            Some(p) if self.node_type(p) == NodeType::Element => Some(p),
            _ => None,
        }
    }

    fn local_name(&self, element: NodeId) -> &str {
        todo!()

        // TODO: ref to temporarily created value
        //       maybe extra atom.to_str() with static lifetime?
        // &*self.local_name(element)
    }

    fn attribute(&self, element: NodeId, attr: &str) -> Option<&str> {
        self.attribute(element, attr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut doc = Document::new();
        assert_eq!(doc.node_type(Document::ROOT), NodeType::Document);
        assert_eq!(doc.children(Document::ROOT), &[]);

        let div = doc.create_element("div");
        assert_eq!(doc.node_type(div), NodeType::Element);
        assert_eq!(doc.style(div), None);
        assert_eq!(doc.local_name(div), "div");
        assert_eq!(doc.children(div), &[]);

        let hello = doc.create_text_node("hello");
        assert_eq!(doc.node_type(hello), NodeType::Text);
        assert_eq!(doc.text(hello), "hello");
        doc.set_text(hello, "hello world");
        assert_eq!(doc.text(hello), "hello world");

        let other = doc.create_text_node("test");

        doc.append_child(div, hello);
        assert_eq!(doc.children(div), &[hello]);

        doc.append_child(div, other);
        assert_eq!(doc.children(div), &[hello, other]);

        doc.remove_child(div, other);
        assert_eq!(doc.children(div), &[hello]);
    }

    #[test]
    fn qsa() {
        let mut doc = Document::new();
        let div = doc.create_element("div");

        assert_eq!(doc.attribute(div, "id"), None);

        doc.set_attribute(div, "id", "panel");
        assert_eq!(doc.attribute(div, "id").as_deref(), Some("panel"));

        // even before connecting, browsers do the same
        assert!(doc.element_matches(div, "div#panel"));

        doc.append_child(Document::ROOT, div);
        assert_eq!(doc.query_selector(Document::ROOT, "div#panel"), Some(div));
    }

    /*
    #[test]
    fn inline_style() {
        let mut doc = Document::new();
        let div = doc.create_element("div");

        doc[div].el_mut().set_style("display: block");
        assert_eq!(doc[div].el().style().to_string(), "display:block;");

        // doc[div].el_mut().style_mut().set_property("width", "100px");
        // assert_eq!(
        //     doc[div].el().attribute("style").as_deref(),
        //     Some("display:block;width:100px;")
        // );

        doc[div].el_mut().set_style("");
        assert_eq!(doc[div].el().style().to_string(), "");
    }
    */
}
