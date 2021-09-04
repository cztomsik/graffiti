#![allow(unused)]

// observable model
// x holds the data/truth (tree of nodes)
// x allows changes
// - notifies listener
// - qs(a)
// - weak data?

use crate::css::{CssStyleDeclaration, MatchingContext, Selector};
use crate::util::{Atom, SlotMap};
use std::any::{type_name, Any, TypeId};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub type NodeId = u32;

pub trait Node: Debug {
    fn id(&self) -> NodeId;
    fn node_type(&self) -> NodeType;
    fn parent_node(&self) -> Option<Rc<dyn Node>>;
    fn first_child(&self) -> Option<Rc<dyn Node>>;
    fn last_child(&self) -> Option<Rc<dyn Node>>;
    fn next_sibling(&self) -> Option<Rc<dyn Node>>;
    fn prev_sibling(&self) -> Option<Rc<dyn Node>>;
    fn append_child(&self, chidl: Rc<dyn Node>);
    fn insert_before(&self, child: Rc<dyn Node>, before: Rc<dyn Node>);
    fn remove_child(&self, child: Rc<dyn Node>);
    fn query_selector(&self, selector: &str) -> Option<Rc<Element>>;
    fn query_selector_all(&self, selector: &str) -> Vec<Rc<Element>>;
}

pub struct NodeRef {
    store: Rc<RefCell<Store>>,
    id: NodeId,
}

impl<T: 'static + Debug + Deref<Target = NodeRef>> Node for T {
    fn id(&self) -> NodeId {
        self.id
    }

    fn node_type(&self) -> NodeType {
        self.store.borrow().nodes[self.id].node_type
    }

    fn parent_node(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .parent_node
            .map(|id| self.store.borrow().refs[id].clone())
    }

    fn first_child(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .first_child
            .map(|id| self.store.borrow().refs[id].clone())
    }

    fn last_child(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .last_child
            .map(|id| self.store.borrow().refs[id].clone())
    }

    fn next_sibling(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .next_sibling
            .map(|id| self.store.borrow().refs[id].clone())
    }

    fn prev_sibling(&self) -> Option<Rc<dyn Node>> {
        self.store.borrow().nodes[self.id]
            .prev_sibling
            .map(|id| self.store.borrow().refs[id].clone())
    }

    fn append_child(&self, child: Rc<dyn Node>) {
        let mut store = self.store.borrow_mut();

        if store.nodes[self.id].first_child == None {
            store.nodes[self.id].first_child = Some(child.id())
        }

        if let Some(last) = store.nodes[self.id].last_child {
            store.nodes[last].next_sibling = Some(child.id());
        }

        store.nodes[self.id].last_child = Some(child.id());

        store.nodes[child.id()].parent_node = Some(self.id);

        store.refs.put(child.id(), child.clone());
        // TODO: emit
    }

    fn insert_before(&self, child: Rc<dyn Node>, before: Rc<dyn Node>) {
        todo!()
    }

    fn remove_child(&self, child: Rc<dyn Node>) {
        let mut store = self.store.borrow_mut();

        if store.nodes[self.id].last_child == Some(child.id()) {
            store.nodes[self.id].last_child = store.nodes[child.id()].prev_sibling
        }

        if store.nodes[self.id].first_child == Some(child.id()) {
            store.nodes[self.id].first_child = store.nodes[child.id()].next_sibling
        }

        if let Some(prev) = store.nodes[child.id()].prev_sibling {
            store.nodes[prev].next_sibling = store.nodes[child.id()].next_sibling;
        }

        if let Some(next) = store.nodes[child.id()].next_sibling {
            store.nodes[next].prev_sibling = store.nodes[child.id()].prev_sibling;
        }

        store.nodes[child.id()].parent_node = None;
        store.nodes[child.id()].next_sibling = None;
        store.nodes[child.id()].prev_sibling = None;

        store.refs.remove(child.id());
        //self.emit(Event::Remove(parent, child));
    }

    fn query_selector(&self, selector: &str) -> Option<Rc<Element>> {
        todo!()
    }

    fn query_selector_all(&self, selector: &str) -> Vec<Rc<Element>> {
        todo!()
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        let mut s = self.store.borrow_mut();
        s.nodes.remove(self.id);
        s.elements.remove(&self.id);
        s.cdata.remove(&self.id);
    }
}

impl Debug for NodeRef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.debug_tuple("NodeRef").field(&self.id).finish()
    }
}

#[derive(Debug)]
pub struct Document(NodeRef);

#[derive(Debug)]
pub struct Element(NodeRef);

#[derive(Debug)]
pub struct CharacterData(NodeRef);

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
                style: CssStyleDeclaration::new(),
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

    pub fn add_listener(&mut self, listener: impl Fn() + 'static) {
        self.store.borrow_mut().listeners.push(Box::new(listener));
    }
}

impl Element {
    pub fn local_name(&self) -> Atom<String> {
        self.store.borrow().elements.get(&self.id).unwrap().local_name.clone()
    }

    pub fn attribute_names(&self) -> Vec<String> {
        vec!["TODO: el.attribute_names()".to_string()]
    }

    pub fn attribute(&self, att: &str) -> String {
        "TODO: el.attribute()".to_string()
    }

    pub fn set_attribute(&self, att: &str, val: &str) {
        println!("TODO: el.set_attribute()")
    }

    pub fn remove_attribute(&self, att: &str) {
        println!("TODO: el.remove_attribute()")
    }

    pub fn matches(&self, selector: &str) -> bool {
        todo!()
    }

    //pub fn style() -> Rc<CssStyleDeclaration> { todo!() }

    pub fn style_property_value(&self, prop: &str) -> Option<String> {
        self.store
            .borrow()
            .elements
            .get(&self.id)
            .unwrap()
            .style
            .property_value(prop)
    }

    pub fn style_set_property(&self, prop: &str, value: &str) {
        self.store
            .borrow_mut()
            .elements
            .get_mut(&self.id)
            .unwrap()
            .style
            .set_property(prop, value)
    }
}

impl CharacterData {
    pub fn data(&self) -> String {
        self.store.borrow().cdata.get(&self.id).unwrap().clone()
    }

    pub fn set_data(&self, data: &str) {
        self.store.borrow_mut().cdata.insert(self.id, data.to_owned());
        // TODO: self.emit(Event::Cdata(cdata_node, cdata));
    }
}

#[derive(Default)]
pub struct Store {
    refs: SlotMap<NodeId, Rc<dyn Node>>,
    nodes: SlotMap<NodeId, NodeData>,
    elements: HashMap<NodeId, ElementData>,
    cdata: HashMap<NodeId, String>,
    listeners: Vec<Box<dyn Fn()>>,

    // SlotMap + Vec because node freeing has to be fast
    weak_data: SlotMap<NodeId, Vec<Box<dyn Any>>>,
}

struct NodeData {
    node_type: NodeType,
    parent_node: Option<NodeId>,
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    last_child: Option<NodeId>,
}

struct ElementData {
    local_name: Atom<String>,
    style: CssStyleDeclaration,
}

fn create_node(store: &Rc<RefCell<Store>>, node_type: NodeType) -> NodeRef {
    let store = Rc::clone(store);
    let id = store.borrow_mut().nodes.insert(NodeData {
        node_type,
        parent_node: None,
        first_child: None,
        next_sibling: None,
        prev_sibling: None,
        last_child: None,
    });

    NodeRef { store, id }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for dyn Node {
        fn eq(&self, other: &Self) -> bool {
            self.id() == other.id()
        }
    }

    /*
    #[test]
    fn test() {
        doc.insert_child(doc.root(), div, 0);
        doc.insert_child(div, hello, 0);

        doc.set_attribute(div, "id", "panel");
        assert_eq!(doc.attribute(div, "id").as_deref(), Some("panel"));

        assert_eq!(doc.query_selector(doc.root(), "div#panel"), Some(div));
    }
    */

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
    fn tree() {
        let doc = Document::new();
        assert_eq!(doc.parent_node(), None);
        assert_eq!(doc.first_child(), None);
        assert_eq!(doc.next_sibling(), None);
        assert_eq!(doc.prev_sibling(), None);

        let ch1 = doc.create_text_node("ch1");
        let ch2 = doc.create_text_node("ch2");
        let ch3 = doc.create_text_node("ch3");

        doc.append_child(ch1.clone());
        assert_eq!(doc.first_child(), Some(ch1.clone() as _));
        assert_eq!(ch1.parent_node(), Some(doc.clone() as _));
        assert_eq!(ch1.next_sibling(), None);
        assert_eq!(ch1.prev_sibling(), None);

        /*
        doc.append_child(root, ch2);
        assert_eq!(doc.first_child(root), Some(ch1));
        assert_eq!(doc.next_sibling(ch1), Some(ch2));
        assert_eq!(doc.prev_sibling(ch2), Some(ch1));

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

    /*
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
    */
}
