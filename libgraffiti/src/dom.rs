// document model
// x holds the data/truth (tree of nodes)
// x allows changes
// x qs(a)
// x weak data

use crate::css::{CssStyleDeclaration, MatchingContext, Selector};
use crate::util::{Atom, SlotMap};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Element = 1,
    Text = 3,
    Comment = 8,
    Document = 9,
}

// TODO: maybe we can make it private?
pub type NodeId = u32;

pub trait Node: Any + Deref<Target = NodeRef> + Debug {}
impl<T: 'static + Deref<Target = NodeRef> + Debug> Node for T {}

impl dyn Node {
    pub fn downcast<T: Node>(self: Rc<dyn Node>) -> Result<Rc<T>, Rc<dyn Node>> {
        if (*self).type_id() == TypeId::of::<T>() {
            unsafe { Ok(Rc::from_raw(Rc::into_raw(self) as *const _)) }
        } else {
            Err(self)
        }
    }
}

pub struct NodeRef {
    store: Rc<RefCell<Store>>,
    id: NodeId,
}

impl NodeRef {
    pub fn node_type(&self) -> NodeType {
        self.store.borrow().nodes[self.id].node_type
    }

    pub fn parent_node(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .parent_node
            .map(|id| self.store.borrow().refs[id].clone())
    }

    pub fn first_child(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .first_child
            .map(|id| self.store.borrow().refs[id].clone())
    }

    pub fn last_child(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .last_child
            .map(|id| self.store.borrow().refs[id].clone())
    }

    pub fn previous_sibling(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .previous_sibling
            .map(|id| self.store.borrow().refs[id].clone())
    }

    pub fn next_sibling(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .next_sibling
            .map(|id| self.store.borrow().refs[id].clone())
    }

    pub fn append_child(&self, child: Rc<dyn Node>) {
        let mut store = self.store.borrow_mut();
        let nodes = &mut store.nodes;

        if nodes[self.id].first_child == None {
            nodes[self.id].first_child = Some(child.id)
        }

        if let Some(last) = nodes[self.id].last_child {
            nodes[last].next_sibling = Some(child.id);
        }

        nodes[child.id].previous_sibling = nodes[child.id].last_child;
        nodes[self.id].last_child = Some(child.id);
        nodes[child.id].parent_node = Some(self.id);

        store.refs.put(child.id, child.clone());
    }

    // TODO: test
    pub fn insert_before(&self, child: Rc<dyn Node>, before: Rc<dyn Node>) {
        let mut store = self.store.borrow_mut();
        let nodes = &mut store.nodes;

        if nodes[self.id].first_child == Some(before.id) {
            nodes[self.id].first_child = Some(child.id)
        }

        nodes[before.id].previous_sibling = Some(child.id);

        nodes[child.id].next_sibling = Some(before.id);
        nodes[child.id].parent_node = Some(self.id);

        store.refs.put(child.id, child.clone());
    }

    pub fn remove_child(&self, child: Rc<dyn Node>) {
        let mut store = self.store.borrow_mut();
        let nodes = &mut store.nodes;

        if nodes[self.id].first_child == Some(child.id) {
            nodes[self.id].first_child = nodes[child.id].next_sibling
        }

        if nodes[self.id].last_child == Some(child.id) {
            nodes[self.id].last_child = nodes[child.id].previous_sibling
        }

        if let Some(prev) = nodes[child.id].previous_sibling {
            nodes[prev].next_sibling = nodes[child.id].next_sibling;
        }

        if let Some(next) = nodes[child.id].next_sibling {
            nodes[next].previous_sibling = nodes[child.id].previous_sibling;
        }

        nodes[child.id].parent_node = None;
        nodes[child.id].next_sibling = None;
        nodes[child.id].previous_sibling = None;

        store.refs.remove(child.id);
    }

    pub fn query_selector(&self, selector: &str) -> Option<Rc<Element>> {
        self.query_selector_all(selector).get(0).cloned()
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<Rc<Element>> {
        let selector = Selector::from(selector);
        let els = Traverse {
            store: self.store.clone(),
            next: self.store.borrow().nodes[self.id].first_child.map(NodeEdge::Start),
            }
        .filter_map(|edge| match edge {
            NodeEdge::Start(node) if self.store.borrow().nodes[node].node_type == NodeType::Element => Some(node),
            _ => None,
        });

        self.with_matching_context(|ctx| {
            els.into_iter()
                .filter(|&el| ctx.match_selector(&selector, el).is_some())
                .map(|el| self.store.borrow().refs[el].clone().downcast::<Element>().unwrap())
                .collect()
        })
    }

    // sparse data, dropped with the node
    // (canvas? parsed sheet?)
    pub fn weak_data<T: Clone + 'static>(&self) -> Option<T> {
        self.store.borrow().weak_data[self.id]
            .iter()
            .find_map(|any| any.downcast_ref())
            .cloned()
    }

    pub fn set_weak_data<T: 'static>(&self, data: T) {
        let mut store = self.store.borrow_mut();

        if let Some(v) = store.weak_data[self.id].iter_mut().find(|any| any.is::<T>()) {
            *v = Box::new(data);
        } else {
            store.weak_data[self.id].push(Box::new(data));
        }
    }

    pub fn remove_weak_data<T: 'static>(&self) {
        self.store.borrow_mut().weak_data[self.id].retain(|any| !any.is::<T>());
    }

    // helpers

    pub(crate) fn with_matching_context<R, F: FnOnce(MatchingContext<'_, NodeId>) -> R>(&self, f: F) -> R {
        let store = self.store.borrow();
        let els = &store.elements;

        f(MatchingContext {
            has_local_name: &|el, name| name == &els[&el].local_name,
            has_identifier: &|el, id| Some(id) == els[&el].identifier.as_ref(),
            has_class: &|el, cls| match &els[&el].class_name {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                None => false,
            },
            parent: &|el| store.nodes[el].parent_node,
        })
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store) && self.id == other.id
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        let mut s = self.store.borrow_mut();
        s.nodes.remove(self.id);

        s.elements.remove(&self.id);
        s.cdata.remove(&self.id);

        s.weak_data.remove(self.id);
        s.free_ids.push(self.id);

        // TODO: we need to do something about (strong) refs to child nodes
    }
}

impl Debug for NodeRef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.debug_tuple("NodeRef").field(&self.id).finish()
    }
}

#[derive(Debug, PartialEq)]
pub struct Document(NodeRef);

#[derive(Debug, PartialEq)]
pub struct Element(NodeRef);

#[derive(Debug, PartialEq)]
pub struct CharacterData(NodeRef);

// TODO: move to util?
// (we might use it for CSS too)
macro_rules! impl_deref_node { ($($struct:ident),*) => { $(impl Deref for $struct { type Target = NodeRef; fn deref(&self) -> &NodeRef { &self.0 } } )* } }
impl_deref_node!(Document, Element, CharacterData);

impl Document {
    pub fn new() -> Rc<Document> {
        let doc = Rc::new(Document(create_node(&Default::default(), NodeType::Document)));
        doc.store.borrow_mut().refs.put(doc.id, doc.clone());
        doc
    }

    pub fn create_element(&self, local_name: &str) -> Rc<Element> {
        let res = Rc::new(Element(create_node(&self.store, NodeType::Element)));
        self.store.borrow_mut().elements.insert(
            res.id,
            ElementData {
                local_name: local_name.into(),
                identifier: None,
                class_name: None,
                style: CssStyleDeclaration::new(),
                attributes: Vec::new(),
            },
        );
        res
    }

    pub fn create_text_node(&self, data: &str) -> Rc<CharacterData> {
        let res = Rc::new(CharacterData(create_node(&self.store, NodeType::Text)));
        res.set_data(data);
        res
    }

    pub fn create_comment(&self, data: &str) -> Rc<CharacterData> {
        let res = Rc::new(CharacterData(create_node(&self.store, NodeType::Comment)));
        res.set_data(data);
        res
    }
}

impl Element {
    pub fn local_name(&self) -> Atom<String> {
        self.store.borrow().elements.get(&self.id).unwrap().local_name.clone()
    }

    pub fn attribute_names(&self) -> Vec<String> {
        let store = self.store.borrow();
        let el_data = store.elements.get(&self.id).unwrap();
        let mut names = Vec::new();

        if el_data.identifier.is_some() {
            names.push("id".to_owned());
        }

        if el_data.class_name.is_some() {
            names.push("class".to_owned());
        }

        for (k, _) in &el_data.attributes {
            names.push(k.to_string());
        }

        names
    }

    pub fn attribute(&self, attr: &str) -> Option<String> {
        let store = self.store.borrow();
        let el_data = store.elements.get(&self.id).unwrap();

        match attr {
            "id" => el_data.identifier.as_deref().cloned(),
            "class" => el_data.class_name.as_deref().cloned(),
            "style" => Some(el_data.style.css_text()),
            _ => el_data
                .attributes
                .iter()
                .find(|(a, _)| attr == **a)
                .map(|(_, v)| v.to_string()),
        }
    }

    pub fn set_attribute(&self, attr: &str, value: &str) {
        let mut store = self.store.borrow_mut();
        let el_data = store.elements.get_mut(&self.id).unwrap();

        match attr {
            "id" => el_data.identifier = Some(value.into()),
            "class" => el_data.class_name = Some(value.into()),
            "style" => el_data.style.set_css_text(value),
            _ => {
                if let Some(a) = el_data.attributes.iter_mut().find(|(a, _)| attr == **a) {
                    a.1 = value.into();
                } else {
                    el_data.attributes.push((attr.into(), value.into()));
                }
            }
        }
    }

    pub fn remove_attribute(&self, attr: &str) {
        let mut store = self.store.borrow_mut();
        let el_data = store.elements.get_mut(&self.id).unwrap();

        match attr {
            "id" => drop(el_data.identifier.take()),
            "class" => drop(el_data.identifier.take()),
            "style" => el_data.style = CssStyleDeclaration::EMPTY,
            _ => el_data.attributes.retain(|(a, _)| attr != **a),
        };
    }

    pub fn matches(&self, selector: &str) -> bool {
        self.with_matching_context(|ctx| ctx.match_selector(&Selector::from(selector), self.id).is_some())
    }

    //pub fn style() -> Rc<CssStyleDeclaration> { todo!() }
}

impl CharacterData {
    pub fn data(&self) -> String {
        self.store.borrow().cdata[&self.id].clone()
    }

    pub fn set_data(&self, data: &str) {
        self.store.borrow_mut().cdata.insert(self.id, data.to_owned());
    }
}

#[derive(Default)]
pub struct Store {
    refs: SlotMap<NodeId, Rc<dyn Node>>,
    nodes: SlotMap<NodeId, NodeData>,
    elements: HashMap<NodeId, ElementData>,
    cdata: HashMap<NodeId, String>,

    // TODO: move to slotmap?
    free_ids: Vec<NodeId>,

    // SlotMap + Vec because node freeing has to be fast
    weak_data: SlotMap<NodeId, Vec<Box<dyn Any>>>,
}

struct NodeData {
    node_type: NodeType,
    parent_node: Option<NodeId>,
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    last_child: Option<NodeId>,
}

struct ElementData {
    local_name: Atom<String>,
    identifier: Option<Atom<String>>,
    class_name: Option<Atom<String>>,
    // TODO: Rc<>?
    style: CssStyleDeclaration,
    attributes: Vec<(Atom<String>, Atom<String>)>,
}

fn create_node(store: &Rc<RefCell<Store>>, node_type: NodeType) -> NodeRef {
    let store = Rc::clone(store);
    let mut store_mut = store.borrow_mut();

    let node = NodeData {
        node_type,
        parent_node: None,
        first_child: None,
        next_sibling: None,
        previous_sibling: None,
        last_child: None,
    };

    let id = if let Some(id) = store_mut.free_ids.pop() {
        store_mut.nodes.put(id, node);
        store_mut.weak_data.put(id, Vec::new());
        id
    } else {
        let id = store_mut.nodes.insert(node);
        let weak_id = store_mut.weak_data.insert(Vec::new());
        debug_assert_eq!(id, weak_id);

        id
    };
    drop(store_mut);

    NodeRef { store, id }
}

#[derive(Clone, Debug)]
enum NodeEdge {
    Start(NodeId),
    End(NodeId),
}

#[derive(Clone)]
struct Traverse {
    // TODO: so it's going to borrow every time?
    //       or is this going to be Ref<> and it might panic on change?
    store: Rc<RefCell<Store>>,
    next: Option<NodeEdge>,
}

impl Iterator for Traverse {
    type Item = NodeEdge;

    fn next(&mut self) -> Option<NodeEdge> {
        let nodes = &self.store.borrow().nodes;

        match self.next.take() {
            Some(next) => {
                self.next = match next {
                    NodeEdge::Start(node) => match nodes[node].first_child {
                        Some(first_child) => Some(NodeEdge::Start(first_child)),
                        None => Some(NodeEdge::End(node)),
                    },
                    NodeEdge::End(node) => match nodes[node].next_sibling {
                        Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                        None => match nodes[node].parent_node {
                            Some(parent) => Some(NodeEdge::End(parent)),
                            None => None,
                        },
                    },
                };
                Some(next)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for dyn Node {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn test() {
        let doc = Document::new();
        assert_eq!(doc.node_type(), NodeType::Document);
        assert_eq!(doc.first_child(), None);

        let div = doc.create_element("div");
        assert_eq!(div.node_type(), NodeType::Element);
        assert_eq!(*div.local_name(), "div");
        assert_eq!(doc.first_child(), None);

        let hello = doc.create_text_node("hello");
        assert_eq!(hello.node_type(), NodeType::Text);
        assert_eq!(hello.data(), "hello");
        hello.set_data("hello world");

        let comment = doc.create_comment("test");
        assert_eq!(comment.node_type(), NodeType::Comment);
        assert_eq!(comment.data(), "test");
        comment.set_data("test2");

        div.append_child(hello.clone());
        assert_eq!(div.first_child(), Some(hello.clone() as _));

        div.append_child(comment.clone());
        assert_eq!(div.first_child(), Some(hello.clone() as _));
        assert_eq!(hello.next_sibling(), Some(comment.clone() as _));

        div.remove_child(comment.clone());
        assert_eq!(div.first_child(), Some(hello.clone() as _));
        assert_eq!(div.next_sibling(), None);
    }

    #[test]
    fn qsa() {
        let doc = Document::new();
        let div = doc.create_element("div");

        div.set_attribute("id", "panel");
        assert_eq!(div.attribute("id").as_deref(), Some("panel"));

        // even before connecting, browsers do the same
        assert!(div.matches("div#panel"));

        doc.append_child(div.clone());
        assert_eq!(doc.query_selector("div#panel"), Some(div));
    }

    #[test]
    fn tree() {
        let doc = Document::new();
        assert_eq!(doc.parent_node(), None);
        assert_eq!(doc.first_child(), None);
        assert_eq!(doc.next_sibling(), None);
        assert_eq!(doc.previous_sibling(), None);

        let ch1 = doc.create_text_node("ch1");
        let ch2 = doc.create_text_node("ch2");
        let ch3 = doc.create_text_node("ch3");

        doc.append_child(ch1.clone());
        assert_eq!(doc.first_child(), Some(ch1.clone() as _));
        assert_eq!(ch1.parent_node(), Some(doc.clone() as _));
        assert_eq!(ch1.next_sibling(), None);
        assert_eq!(ch1.previous_sibling(), None);

        /*
        doc.append_child(root, ch2);
        assert_eq!(doc.first_child(root), Some(ch1));
        assert_eq!(doc.next_sibling(ch1), Some(ch2));
        assert_eq!(doc.previous_sibling(ch2), Some(ch1));

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2]);

        doc.insert_child(root, ch3, 0);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3, ch1, ch2]);

        doc.remove_child(ch1);
        doc.remove_child(ch2);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3]);

        doc.insert_child(ch2, 0);
        doc.insert_child(ch1, 0);

        assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2, ch3]);
        */
    }

    #[test]
    fn inline_style() {
        let doc = Document::new();
        let div = doc.create_element("div");

        div.set_attribute("style", "display: block");
        //assert_eq!(div.style().css_text(), "display: block");

        //div.style().set_property("width", "100px");
        assert_eq!(div.attribute("style").as_deref(), Some("display: block; width: 100px"));

        div.remove_attribute("style");
        //assert_eq!(div.style().css_text(), "");
    }

    #[test]
    fn weak_data() {
        let doc = Document::new();

        assert_eq!(doc.weak_data(), None::<usize>);
        doc.set_weak_data(1);
        assert_eq!(doc.weak_data(), Some(1));
        doc.remove_weak_data::<usize>();
        assert_eq!(doc.weak_data(), None::<usize>);
    }

    #[test]
    fn downcast() {
        let doc = Document::new();
        let div = doc.create_element("div");

        let node: Rc<dyn Node> = doc.clone();
        assert!(node.downcast::<Document>().is_ok());

        let node: Rc<dyn Node> = div.clone();
        assert!(node.downcast::<Document>().is_err());

        let node: Rc<dyn Node> = div.clone();
        assert!(node.downcast::<Element>().is_ok());
    }
}
