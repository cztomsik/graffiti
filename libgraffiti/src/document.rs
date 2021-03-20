// observable model
// x holds the data/truth (tree of nodes)
// x allows changes
// x notifies listener
// x panics for invalid node types
//  (another layer on top of this should make sure it never happens)

use crate::css::{MatchingContext, Selector};
use crate::util::{Atom, IdTree};
use std::collections::HashMap;

pub type NodeId = u32;

#[derive(Debug)]
pub enum DocumentEvent {
    ParentChanged(NodeId),
    NodeDestroyed(NodeId),

    TextNodeCreated(NodeId),
    CommentCreated(NodeId),

    CharacterDataChanged(NodeId),

    ElementCreated(NodeId),
    AttributesChanged(NodeId),
    NodeInserted(NodeId, NodeId, usize),
    NodeRemoved(NodeId, NodeId),
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CdataSection = 4,
    EntityReference = 5,
    Entity = 6,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
    Notation = 12,
}

pub struct Document {
    tree: IdTree<NodeData>,
    root: NodeId,

    listener: Box<dyn Fn(DocumentEvent) + Send>,
}

// private shorthand
type Event = DocumentEvent;

impl Document {
    pub fn new(listener: impl Fn(DocumentEvent) + 'static + Send) -> Self {
        let listener = Box::new(listener);
        let mut tree = IdTree::new();

        let root = tree.create_node(NodeData::Document);

        listener(Event::ElementCreated(root));

        Self { tree, root, listener }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    // shared for all node types

    pub fn node_type(&self, node: NodeId) -> NodeType {
        match self.tree.data(node) {
            NodeData::Element(_) => NodeType::Element,
            NodeData::Text(_) => NodeType::Text,
            NodeData::Comment(_) => NodeType::Comment,
            NodeData::Document => NodeType::Document,
        }
    }

    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.tree.parent(node)
    }

    pub fn child_nodes(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.tree.children(node)
    }

    pub fn children(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.child_nodes(node)
            .filter(move |n| self.node_type(*n) == NodeType::Element)
    }

    pub fn matches(&self, el: NodeId, selector: &Selector) -> bool {
        todo!()
        //self.matching_context().match_selector(selector, el)
    }

    pub fn query_selector(&self, context_node: NodeId, selector: &Selector) -> Option<NodeId> {
        self.query_selector_all(context_node, selector).get(0).copied()
    }

    pub fn query_selector_all(&self, context_node: NodeId, selector: &Selector) -> Vec<NodeId> {
        //println!("QSA {:?}", (context_node, selector));

        // TODO: helper/macro
        let ctx = MatchingContext {
            has_local_name: &|el, name| **name == self.local_name(el),
            has_identifier: &|el, id| Some(id.as_str()) == self.attribute(el, "id"),
            has_class: &|el, cls| {
                self.attribute(el, "class")
                    .unwrap_or("")
                    .split_ascii_whitespace()
                    .any(|part| part == **cls)
            },
            parent: &|el| self.parent(el),
        };

        let els = self.descendant_children(context_node);
        //println!("els {:?}", &els);

        els.into_iter()
            .filter(|el| ctx.match_selector(&selector, *el))
            .collect()
    }

    pub fn insert_child(&mut self, parent: NodeId, child: NodeId, index: usize) {
        self.tree.insert_child(parent, child, index);

        self.emit(Event::NodeInserted(parent, child, index));
        self.emit(Event::ParentChanged(child));
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.tree.remove_child(parent, child);

        self.emit(Event::NodeRemoved(parent, child));
        self.emit(Event::ParentChanged(child));
    }

    pub fn free_node(&mut self, node: NodeId) {
        self.tree.free_node(node);

        self.emit(Event::NodeDestroyed(node));
    }

    // text node

    pub fn create_text_node(&mut self, cdata: &str) -> NodeId {
        let id = self.tree.create_node(NodeData::Text(cdata.to_owned()));

        self.emit(Event::TextNodeCreated(id));

        id
    }

    // comment

    pub fn create_comment(&mut self, cdata: &str) -> NodeId {
        let id = self.tree.create_node(NodeData::Comment(cdata.to_owned()));

        self.emit(Event::CommentCreated(id));

        id
    }

    // text/comment node

    pub fn cdata(&self, cdata_node: NodeId) -> &str {
        if let NodeData::Text(data) | NodeData::Comment(data) = self.tree.data(cdata_node) {
            data
        } else {
            panic!("not a cdata node")
        }
    }

    pub fn set_cdata(&mut self, cdata_node: NodeId, cdata: &str) {
        if let NodeData::Text(data) | NodeData::Comment(data) = self.tree.data_mut(cdata_node) {
            *data = cdata.to_owned();

            self.emit(Event::CharacterDataChanged(cdata_node));
        } else {
            panic!("not a cdata node")
        }
    }

    // element

    pub fn create_element(&mut self, local_name: &str) -> NodeId {
        let id = self.tree.create_node(NodeData::Element(ElementData {
            local_name: local_name.into(),
            attributes: AttrMap::default(),
        }));

        self.emit(Event::ElementCreated(id));

        id
    }

    pub fn local_name(&self, element: NodeId) -> &str {
        &self.el(element).local_name
    }

    pub fn attribute(&self, element: NodeId, att_name: &str) -> Option<&str> {
        self.el(element).attributes.get(att_name)
    }

    pub fn set_attribute(&mut self, element: NodeId, att_name: &str, value: &str) {
        self.el_mut(element).attributes.set(att_name, value);

        self.emit(Event::AttributesChanged(element));
    }

    pub fn remove_attribute(&mut self, element: NodeId, att_name: &str) {
        self.el_mut(element).attributes.remove(att_name);

        self.emit(Event::AttributesChanged(element));
    }

    // helpers

    fn descendant_children(&self, element: NodeId) -> Vec<NodeId> {
        self.children(element)
            .flat_map(move |ch| std::iter::once(ch).chain(self.descendant_children(ch)))
            .collect()
    }

    fn el(&self, el: NodeId) -> &ElementData {
        if let NodeData::Element(data) = self.tree.data(el) {
            data
        } else {
            panic!("not an element")
        }
    }

    fn el_mut(&mut self, el: NodeId) -> &mut ElementData {
        if let NodeData::Element(data) = self.tree.data_mut(el) {
            data
        } else {
            panic!("not an element")
        }
    }

    fn emit(&self, event: Event) {
        (self.listener)(event);
    }
}

// private from here

enum NodeData {
    Document,
    Element(ElementData),
    Text(String),
    Comment(String),
}

struct ElementData {
    local_name: Atom<String>,
    attributes: AttrMap,
}

#[derive(Default)]
struct AttrMap {
    identifier: Option<Atom<String>>,
    class_name: Option<Atom<String>>,
    attrs: Vec<(Atom<String>, Atom<String>)>,
}

// note that document is using the same &'static strs
// so that matching should be quick
impl AttrMap {
    fn get(&self, att: &str) -> Option<&str> {
        let opt = match att {
            "id" => self.identifier.as_deref(),
            "class" => self.class_name.as_deref(),
            _ => self.attrs.iter().find(|(a, _)| att == **a).map(|(_, v)| &**v),
        };

        opt.map(String::as_str)
    }

    fn set(&mut self, att: &str, v: &str) {
        match att {
            "id" => self.identifier = Some(v.into()),
            "class" => self.class_name = Some(v.into()),
            _ => {
                if let Some(a) = self.attrs.iter_mut().find(|(a, _)| att == **a) {
                    a.1 = v.into();
                } else {
                    self.attrs.push((att.into(), v.into()));
                }
            }
        }
    }

    fn remove(&mut self, att: &str) {
        match att {
            "id" => self.identifier.take(),
            "class" => self.identifier.take(),
            _ => return self.attrs.retain(|(a, _)| att != **a),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut d = Document::new(|_| {});

        let div = d.create_element("div");
        let hello = d.create_text_node("hello");

        d.insert_child(d.root(), div, 0);
        d.insert_child(div, hello, 0);

        // TODO: impl from(&'static str)?
        use std::convert::TryFrom;
        assert_eq!(
            d.query_selector(d.root(), &Selector::try_from("div").unwrap()),
            Some(div)
        );
    }
}
