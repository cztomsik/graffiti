// observable model
// x holds the data/truth (tree of nodes)
// x allows changes
// x notifies listener
// x panics for invalid node types
//  (another layer on top of this should make sure it never happens)

use crate::css::{MatchingContext, Selector, Style};
use crate::util::{Atom, IdTree, SlotMap};
use std::any::Any;
use std::convert::TryFrom;

pub type NodeId = u32;

#[derive(Debug)]
pub enum DocumentEvent {
    Create(NodeId, NodeType),
    Insert(NodeId, NodeId, usize),
    Remove(NodeId, NodeId),

    // TODO: call during Document::Drop, probably in document order (children first)
    Drop(NodeId),
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

    listeners: Vec<Box<dyn Fn(&Document, &Event)>>,

    // SlotMap + Vec because node freeing has to be fast
    weak_data: SlotMap<NodeId, Vec<Box<dyn Any>>>,
}

// private shorthand
type Event = DocumentEvent;

impl Document {
    pub fn new() -> Self {
        let mut doc = Self {
            tree: IdTree::new(),
            root: 0,
            listeners: Vec::new(),
            weak_data: SlotMap::new(),
        };

        let root = doc.create_node(NodeData::Document);
        doc.root = root;

        doc
    }

    pub fn add_listener(&mut self, listener: impl Fn(&Document, &Event) + 'static) {
        self.listeners.push(Box::new(listener));
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

    pub fn matches(&self, el: NodeId, selector: &str) -> bool {
        todo!()
        //self.matching_context().match_selector(selector, el)
    }

    pub fn query_selector(&self, context_node: NodeId, selector: &str) -> Option<NodeId> {
        self.query_selector_all(context_node, selector).get(0).copied()
    }

    pub fn query_selector_all(&self, context_node: NodeId, selector: &str) -> Vec<NodeId> {
        let selector = match Selector::try_from(selector) {
            Ok(s) => s,
            _ => return Vec::new(),
        };

        // TODO: helper/macro
        // TODO: avoid allocs, try different layouts
        let ctx = MatchingContext {
            has_local_name: &|el, name| **name == self.local_name(el),
            has_identifier: &|el, id| Some(id.to_string()) == self.attribute(el, "id"),
            has_class: &|el, cls| match self.attribute(el, "class") {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                None => false,
            },
            parent: &|el| self.parent(el),
        };

        let els = self.descendant_children(context_node);

        els.into_iter()
            .filter(|el| ctx.match_selector(&selector, *el))
            .collect()
    }

    pub fn insert_child(&mut self, parent: NodeId, child: NodeId, index: usize) {
        self.tree.insert_child(parent, child, index);

        self.emit(Event::Insert(parent, child, index));
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.tree.remove_child(parent, child);

        self.emit(Event::Remove(parent, child));
    }

    // meant for sparse, any-shape data like attaching StyleSheet to <style>
    pub fn weak_data<T: 'static>(&self, node: NodeId) -> Option<&T> {
        self.weak_data[node].iter().find_map(|any| any.downcast_ref())
    }

    pub fn weak_data_mut<T: 'static>(&mut self, node: NodeId) -> Option<&mut T> {
        self.weak_data[node].iter_mut().find_map(|any| any.downcast_mut())
    }

    pub fn set_weak_data<T: 'static>(&mut self, node: NodeId, data: T) {
        if let Some(v) = self.weak_data[node].iter_mut().find(|any| any.is::<T>()) {
            *v = Box::new(data);
        } else {
            self.weak_data[node].push(Box::new(data));
        }
    }

    pub fn remove_weak_data<T: 'static>(&mut self, node: NodeId) {
        self.weak_data[node].retain(|any| !any.is::<T>());
    }

    pub fn drop_node(&mut self, node: NodeId) {
        self.tree.drop_node(node);
        self.weak_data.remove(node);
    }

    // text node

    pub fn create_text_node(&mut self, cdata: &str) -> NodeId {
        let id = self.create_node(NodeData::Text(cdata.to_owned()));

        id
    }

    // comment

    pub fn create_comment(&mut self, cdata: &str) -> NodeId {
        let id = self.create_node(NodeData::Comment(cdata.to_owned()));

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
        } else {
            panic!("not a cdata node")
        }
    }

    // element

    pub fn create_element(&mut self, local_name: &str) -> NodeId {
        let id = self.create_node(NodeData::Element(ElementData {
            local_name: local_name.into(),
            identifier: None,
            class_name: None,
            style: Style::EMPTY,
            attrs: Vec::new(),
        }));

        id
    }

    pub fn local_name(&self, element: NodeId) -> &str {
        &self.el(element).local_name
    }

    pub fn attribute(&self, element: NodeId, att_name: &str) -> Option<String> {
        let el_data = self.el(element);

        match att_name {
            "id" => el_data.identifier.as_deref().cloned(),
            "class" => el_data.class_name.as_deref().cloned(),
            "style" => Some(el_data.style.css_text()),
            _ => el_data
                .attrs
                .iter()
                .find(|(a, _)| att_name == **a)
                .map(|(_, v)| v.to_string()),
        }
    }

    pub fn set_attribute(&mut self, element: NodeId, att_name: &str, value: &str) {
        let el_data = self.el_mut(element);

        match att_name {
            "id" => el_data.identifier = Some(value.into()),
            "class" => el_data.class_name = Some(value.into()),
            "style" => el_data.style.set_css_text(value),
            _ => {
                if let Some(a) = el_data.attrs.iter_mut().find(|(a, _)| att_name == **a) {
                    a.1 = value.into();
                } else {
                    el_data.attrs.push((att_name.into(), value.into()));
                }
            }
        }
    }

    pub fn remove_attribute(&mut self, element: NodeId, att_name: &str) {
        let el_data = self.el_mut(element);

        match att_name {
            "id" => drop(el_data.identifier.take()),
            "class" => drop(el_data.identifier.take()),
            "style" => el_data.style = Style::EMPTY,
            _ => el_data.attrs.retain(|(a, _)| att_name != **a),
        };
    }

    pub fn attribute_names(&self, element: NodeId) -> Vec<String> {
        let el_data = self.el(element);
        let mut names = Vec::new();

        if el_data.identifier.is_some() {
            names.push("id".to_owned());
        }

        if el_data.class_name.is_some() {
            names.push("class".to_owned());
        }

        for (k, _) in &el_data.attrs {
            names.push(k.to_string());
        }

        names
    }

    /*
    pub fn style(&self, element: NodeId) -> &Style {
        &self.el(element).style
    }
    */

    // helpers

    fn create_node(&mut self, data: NodeData) -> NodeId {
        let id = self.tree.create_node(data);
        let weak_id = self.weak_data.insert(Vec::new());

        debug_assert_eq!(id, weak_id);

        self.emit(Event::Create(id, self.node_type(id)));

        id
    }

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
        for listener in &self.listeners {
            listener(self, &event);
        }
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
    identifier: Option<Atom<String>>,
    class_name: Option<Atom<String>>,
    style: Style,
    attrs: Vec<(Atom<String>, Atom<String>)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut d = Document::new();
        assert_eq!(d.node_type(d.root()), NodeType::Document);

        let div = d.create_element("div");
        assert_eq!(d.local_name(div), "div");

        let hello = d.create_text_node("hello");
        assert_eq!(d.cdata(hello), "hello");

        d.insert_child(d.root(), div, 0);
        d.insert_child(div, hello, 0);

        d.set_attribute(div, "id", "panel");
        assert_eq!(d.attribute(div, "id"), Some("panel"));

        assert_eq!(d.query_selector(d.root(), "div#panel"), Some(div));
    }

    /*
    #[test]
    fn inline_style() {
        use crate::css::{Display, StyleProp, Value};

        let mut d = Document::new();
        let div = d.create_element("div");

        d.set_attribute(div, "style", "display: block");
        assert_eq!(
            d.style(div).props().next().unwrap(),
            &StyleProp::Display(Value::Specified(Display::Block))
        );

        // TODO: change style prop

        // TODO: check attr("style")

        d.remove_attribute(div, "style");
        assert_eq!(d.style(div), &Style::EMPTY);
    }
    */

    #[test]
    fn weak_data() {
        let mut d = Document::new();
        let root = d.root();

        assert_eq!(d.weak_data(root), None::<&usize>);
        d.set_weak_data(root, 1);
        assert_eq!(d.weak_data(root), Some(&1));
        *(d.weak_data_mut(root).unwrap()) = 2;
        assert_eq!(d.weak_data(root), Some(&2));
        d.remove_weak_data::<usize>(root);
        assert_eq!(d.weak_data(root), None::<&usize>);
    }
}
