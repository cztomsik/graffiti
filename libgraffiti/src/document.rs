// observable model
// x holds the data/truth (tree of nodes)
// x allows changes
// x notifies listener
// x panics for invalid node types
//  (another layer on top of this should make sure it never happens)

use crate::css::{MatchingContext, Selector, Style};
use crate::util::{Atom, SlotMap};
use std::any::Any;
use std::borrow::Cow;
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
    nodes: SlotMap<NodeId, Node>,
    root: NodeId,

    listeners: Vec<Box<dyn Fn(&Document, &Event)>>,

    // SlotMap + Vec because node freeing has to be fast
    weak_data: SlotMap<NodeId, Vec<Box<dyn Any>>>,

    free_ids: Vec<NodeId>,
}

// private shorthand
type Event = DocumentEvent;

impl Document {
    pub fn new() -> Self {
        let mut doc = Self {
            nodes: SlotMap::new(),
            root: 0,
            listeners: Vec::new(),
            weak_data: SlotMap::new(),
            free_ids: Vec::new(),
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
        match self.nodes[node].data {
            NodeData::Element(_) => NodeType::Element,
            NodeData::Text(_) => NodeType::Text,
            NodeData::Comment(_) => NodeType::Comment,
            NodeData::Document => NodeType::Document,
        }
    }

    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].parent
    }

    pub fn first_child(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].first_child
    }

    pub fn prev_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.child_nodes(self.parent(node)?)
            .find(|n| self.nodes[*n].next_sibling == Some(node))
    }

    pub fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].next_sibling
    }

    pub fn child_nodes(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        ChildNodes {
            doc: self,
            next: self.nodes[node].first_child,
        }
    }

    pub fn children(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.child_nodes(node)
            .filter(move |n| self.node_type(*n) == NodeType::Element)
    }

    // doesn't allocate if there's 0-1 child_nodes
    pub fn text_content(&self, node: NodeId) -> Cow<str> {
        match self.node_type(node) {
            NodeType::Text => Cow::Borrowed(self.cdata(node)),
            _ => match self.nodes[node].first_child {
                None => Cow::Borrowed(""),
                Some(ch) => match self.nodes[ch].next_sibling {
                    None => self.text_content(ch),
                    Some(_) => {
                        let string = self
                            .child_nodes(node)
                            .fold(String::new(), |res, ch| res + &self.text_content(ch));
                        Cow::Owned(string)
                    }
                },
            },
        }
    }

    pub fn matches(&self, el: NodeId, selector: &str) -> bool {
        self.with_matching_context(|ctx| ctx.match_selector(&Selector::from(selector), el).is_some())
    }

    pub fn query_selector(&self, context_node: NodeId, selector: &str) -> Option<NodeId> {
        self.query_selector_all(context_node, selector).get(0).copied()
    }

    pub fn query_selector_all(&self, context_node: NodeId, selector: &str) -> Vec<NodeId> {
        let selector = Selector::from(selector);
        let els = self.descendant_children(context_node);

        self.with_matching_context(|ctx| {
            els.into_iter()
                .filter(|el| ctx.match_selector(&selector, *el).is_some())
                .collect()
        })
    }

    pub fn insert_child(&mut self, parent: NodeId, child: NodeId, index: usize) {
        debug_assert_eq!(self.nodes[child].parent, None);

        if index == 0 {
            self.nodes[child].next_sibling = self.first_child(parent);
            self.nodes[parent].first_child = Some(child);
        } else {
            let prev = (1..index)
                .fold(self.first_child(parent), |n, _| self.next_sibling(n?))
                .expect("out of bounds");

            self.nodes[child].next_sibling = self.next_sibling(prev);
            self.nodes[prev].next_sibling = Some(child);
        }

        self.nodes[child].parent = Some(parent);

        self.emit(Event::Insert(parent, child, index));
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        debug_assert_eq!(self.nodes[child].parent, Some(parent));

        if let Some(prev) = self.prev_sibling(child) {
            self.nodes[prev].next_sibling = self.next_sibling(child);
        } else {
            self.nodes[parent].first_child = self.next_sibling(child);
        }

        self.nodes[child].next_sibling = None;
        self.nodes[child].parent = None;

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
        drop(self.nodes.remove(node));
        self.weak_data.remove(node);
        self.free_ids.push(node);

        // TODO: emit
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
        if let NodeData::Text(data) | NodeData::Comment(data) = &self.nodes[cdata_node].data {
            data
        } else {
            panic!("not a cdata node")
        }
    }

    pub fn set_cdata(&mut self, cdata_node: NodeId, cdata: &str) {
        if let NodeData::Text(data) | NodeData::Comment(data) = &mut self.nodes[cdata_node].data {
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
        // TODO: id reusing

        let id = self.nodes.insert(Node {
            parent: None,
            first_child: None,
            next_sibling: None,
            data,
        });

        let weak_id = self.weak_data.insert(Vec::new());

        debug_assert_eq!(id, weak_id);

        self.emit(Event::Create(id, self.node_type(id)));

        id
    }

    pub(crate) fn descendant_children(&self, element: NodeId) -> Vec<NodeId> {
        self.children(element)
            .flat_map(move |ch| std::iter::once(ch).chain(self.descendant_children(ch)))
            .collect()
    }

    pub(crate) fn with_matching_context<R, F: FnOnce(MatchingContext<'_, NodeId>) -> R>(&self, f: F) -> R {
        f(MatchingContext {
            has_local_name: &|el, name| **name == self.local_name(el),
            has_identifier: &|el, id| Some(id.to_string()) == self.attribute(el, "id"),
            has_class: &|el, cls| match self.attribute(el, "class") {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                None => false,
            },
            parent: &|el| self.parent(el),
        })
    }

    fn el(&self, el: NodeId) -> &ElementData {
        if let NodeData::Element(data) = &self.nodes[el].data {
            data
        } else {
            panic!("not an element")
        }
    }

    fn el_mut(&mut self, el: NodeId) -> &mut ElementData {
        if let NodeData::Element(data) = &mut self.nodes[el].data {
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

struct Node {
    parent: Option<NodeId>,
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
    data: NodeData,
}

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

pub struct ChildNodes<'a> {
    doc: &'a Document,
    next: Option<NodeId>,
}

impl<'a> Iterator for ChildNodes<'a> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(next) => {
                self.next = self.doc.nodes[next].next_sibling;
                Some(next)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut doc = Document::new();
        assert_eq!(doc.node_type(doc.root()), NodeType::Document);

        let div = doc.create_element("div");
        assert_eq!(doc.local_name(div), "div");

        let hello = doc.create_text_node("hello");
        assert_eq!(doc.cdata(hello), "hello");

        doc.insert_child(doc.root(), div, 0);
        doc.insert_child(div, hello, 0);

        doc.set_attribute(div, "id", "panel");
        assert_eq!(doc.attribute(div, "id").as_deref(), Some("panel"));

        assert_eq!(doc.query_selector(doc.root(), "div#panel"), Some(div));
    }

    #[test]
    fn tree() {
        let mut doc = Document::new();
        let root = doc.root();
        assert_eq!(doc.parent(root), None);
        assert_eq!(doc.first_child(root), None);
        assert_eq!(doc.next_sibling(root), None);
        assert_eq!(doc.prev_sibling(root), None);

        let ch1 = doc.create_text_node("ch1");
        let ch2 = doc.create_text_node("ch2");
        let ch3 = doc.create_text_node("ch3");

        doc.insert_child(root, ch1, 0);
        assert_eq!(doc.first_child(root), Some(ch1));
        assert_eq!(doc.parent(ch1), Some(root));
        assert_eq!(doc.next_sibling(ch1), None);
        assert_eq!(doc.prev_sibling(ch1), None);

        doc.insert_child(root, ch2, 1);
        assert_eq!(doc.first_child(root), Some(ch1));
        assert_eq!(doc.next_sibling(ch1), Some(ch2));
        assert_eq!(doc.prev_sibling(ch2), Some(ch1));

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2]);

        doc.insert_child(root, ch3, 0);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3, ch1, ch2]);

        doc.remove_child(root, ch1);
        doc.remove_child(root, ch2);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3]);

        doc.insert_child(root, ch2, 0);
        doc.insert_child(root, ch1, 0);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2, ch3]);
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
