// observable document model
// x OO-like api (auto-upcast, on-demand downcast)
// x holds the data/truth (tree of nodes)
// x allows changes
// x notifies listener
// x provides query_selector()

use crate::css::{CssStyleDeclaration, CssStyleSheet, Selector};
use crate::util::{Atom, Edge, IdTree};
use fnv::FnvHashMap;
use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Error, Formatter};
use std::num::NonZeroU32;
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

pub type NodeId = NonZeroU32;

#[derive(Debug)]
pub enum DomEvent<'a> {
    NodeCreated(&'a NodeRef),
    AppendChild(&'a NodeRef, &'a NodeRef),
    InsertBefore(&'a NodeRef, &'a NodeRef, &'a NodeRef),
    RemoveChild(&'a NodeRef, &'a NodeRef),
    NodeDestroyed(NodeId),
}

pub struct NodeRef {
    store: Rc<Store>,
    id: NodeId,
}

impl NodeRef {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn node_type(&self) -> NodeType {
        self.store.tree.borrow().data(self.id).node_type()
    }

    pub fn parent_node(&self) -> Option<NodeRef> {
        self.find_node(self.store.tree.borrow().parent_node(self.id)?)
    }

    pub fn first_child(&self) -> Option<NodeRef> {
        self.find_node(self.store.tree.borrow().first_child(self.id)?)
    }

    pub fn last_child(&self) -> Option<NodeRef> {
        self.find_node(self.store.tree.borrow().last_child(self.id)?)
    }

    pub fn previous_sibling(&self) -> Option<NodeRef> {
        self.find_node(self.store.tree.borrow().previous_sibling(self.id)?)
    }

    pub fn next_sibling(&self) -> Option<NodeRef> {
        self.find_node(self.store.tree.borrow().next_sibling(self.id)?)
    }

    // TODO: avoid collect(), return impl Iterator<Item = NodeRef>
    pub fn child_nodes(&self) -> Vec<NodeRef> {
        self.store
            .tree
            .borrow()
            .children(self.id)
            .map(move |id| self.store.node_ref(id))
            .collect()
    }

    pub fn append_child(&self, child: &NodeRef) {
        self.store.tree.borrow_mut().append_child(self.id, child.id);
        child.inc_count();

        self.store.emit(DomEvent::AppendChild(self, child));
    }

    pub fn insert_before(&self, child: &NodeRef, before: &NodeRef) {
        self.store.tree.borrow_mut().insert_before(self.id, child.id, before.id);
        child.inc_count();

        self.store.emit(DomEvent::InsertBefore(self, child, before));
    }

    pub fn remove_child(&self, child: &NodeRef) {
        self.store.tree.borrow_mut().remove_child(self.id, child.id);
        child.dec_count();

        self.store.emit(DomEvent::RemoveChild(self, child));
    }

    pub fn query_selector(&self, selector: &str) -> Option<ElementRef> {
        self.query_selector_all(selector).get(0).cloned()
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<ElementRef> {
        let selector = Selector::parse(selector).unwrap_or(Selector::unsupported());
        let tree = self.store.tree.borrow();
        let els = tree.traverse(self.id).skip(1).filter_map(|edge| match edge {
            Edge::Start(node) => self.store.node_ref(node).downcast::<ElementRef>(),
            _ => None,
        });

        els.filter(|el| selector.match_element(el).is_some()).collect()
    }

    pub fn as_node(&self) -> NodeRef {
        self.store.node_ref(self.id)
    }

    // TODO: as_document(), as_element(), as_character_data() -> Option<XxRef>
    //       but the problem currently is that Index<> in ffi.rs needs to return &Xxx
    //       and we also cannot return borrow of newly-created value so this is TODO
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let node_type = self.node_type();

        if (type_id == TypeId::of::<ElementRef>() && node_type == NodeType::Element)
            || (type_id == TypeId::of::<CharacterDataRef>()
                && (node_type == NodeType::Text || node_type == NodeType::Comment))
            || (type_id == TypeId::of::<DocumentRef>() && node_type == NodeType::Document)
            || (type_id == TypeId::of::<NodeRef>())
        {
            unsafe { std::mem::transmute(self) }
        } else {
            None
        }
    }

    pub fn as_document(&self) -> Option<DocumentRef> {
        self.downcast()
    }

    pub fn as_element(&self) -> Option<ElementRef> {
        self.downcast()
    }

    pub fn as_character_data(&self) -> Option<CharacterDataRef> {
        self.downcast()
    }

    pub fn downcast<T: Clone + 'static>(&self) -> Option<T> {
        self.downcast_ref().cloned()
    }

    // helpers

    // TODO: use nodes.get()
    pub fn find_node(&self, id: NodeId) -> Option<NodeRef> {
        Some(self.store.node_ref(id))
    }

    fn inc_count(&self) {
        self.store.inc_count(self.id);
    }

    fn dec_count(&self) {
        self.store.dec_count(self.id);
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.store, &other.store) && self.id == other.id
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        self.dec_count()
    }
}

impl Debug for NodeRef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self.node_type() {
            NodeType::Element => write!(fmt, "<{}>", self.downcast_ref::<ElementRef>().unwrap().local_name()),
            NodeType::Text => Debug::fmt(&self.downcast_ref::<CharacterDataRef>().unwrap().data(), fmt),
            t => Debug::fmt(&t, fmt),
        }
    }
}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        self.inc_count();

        Self {
            store: self.store.clone(),
            id: self.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentRef(NodeRef);

#[derive(Debug, Clone, PartialEq)]
pub struct ElementRef(NodeRef);

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterDataRef(NodeRef);

macro_rules! impl_deref {
    ($struct:ident, $target: ident) => {
        impl Deref for $struct {
            type Target = $target;
            fn deref(&self) -> &$target {
                &self.0
            }
        }
    };
}
impl_deref!(DocumentRef, NodeRef);
impl_deref!(ElementRef, NodeRef);
impl_deref!(CharacterDataRef, NodeRef);

impl DocumentRef {
    pub fn new() -> DocumentRef {
        DocumentRef(Rc::<Store>::default().create_node(NodeData::Document))
    }

    pub fn add_listener(&self, listener: Rc<dyn Fn(&DomEvent)>) {
        self.store.listeners.borrow_mut().push(listener);
    }

    pub fn remove_listener(&self, listener: &Rc<dyn Fn(&DomEvent)>) {
        self.store.listeners.borrow_mut().retain(|l| Rc::ptr_eq(l, listener));
    }

    pub fn create_element(&self, local_name: &str) -> ElementRef {
        ElementRef(self.store.create_node(NodeData::Element(ElementData {
            local_name: local_name.into(),
            attributes: Vec::new(),
            style: Rc::new(CssStyleDeclaration::new()),
        })))
    }

    pub fn create_text_node(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(self.store.create_node(NodeData::Text(data.to_owned())))
    }

    pub fn create_comment(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(self.store.create_node(NodeData::Comment(data.to_owned())))
    }

    pub fn all_nodes(&self) -> Vec<NodeRef> {
        self.store
            .tree
            .borrow()
            .iter()
            .map(|(id, _data)| self.store.node_ref(id))
            .collect()
    }

    pub fn style_sheet(&self, style_element: &ElementRef) -> Option<Rc<CssStyleSheet>> {
        self.store.style_sheets.borrow().get(&style_element.id()).cloned()
    }

    pub fn style_sheets(&self) -> Vec<Rc<CssStyleSheet>> {
        self.query_selector_all("style")
            .iter()
            .filter_map(|el| self.style_sheet(el))
            .collect()
    }
}

impl ElementRef {
    pub fn local_name(&self) -> Atom<String> {
        self.store.tree.borrow().data(self.id).el().local_name.clone()
    }

    pub fn attribute_names(&self) -> Vec<String> {
        let tree = self.store.tree.borrow();
        let el = tree.data(self.id).el();
        let mut names = Vec::new();

        if el.style.length() > 0 {
            names.push("style".to_owned());
        }

        for (k, _) in &el.attributes {
            names.push(k.to_string());
        }

        names
    }

    pub fn attribute(&self, attr: &str) -> Option<String> {
        let tree = self.store.tree.borrow();
        let el = tree.data(self.id).el();

        match attr {
            "style" => Some(el.style.css_text()),
            _ => el
                .attributes
                .iter()
                .find(|(a, _)| attr == **a)
                .map(|(_, v)| v.to_string()),
        }
    }

    pub fn set_attribute(&self, attr: &str, value: &str) {
        let mut tree = self.store.tree.borrow_mut();
        let mut el = tree.data_mut(self.id).el_mut();

        match attr {
            "style" => el.style.set_css_text(value),
            _ => {
                if let Some(a) = el.attributes.iter_mut().find(|(a, _)| attr == **a) {
                    a.1 = value.into();
                } else {
                    el.attributes.push((attr.into(), value.into()));
                }
            }
        }
    }

    pub fn remove_attribute(&self, attr: &str) {
        let mut tree = self.store.tree.borrow_mut();
        let mut el = tree.data_mut(self.id).el_mut();

        match attr {
            "style" => el.style.set_css_text(""),
            _ => el.attributes.retain(|(a, _)| attr != **a),
        };
    }

    pub fn matches(&self, selector: &str) -> bool {
        match Selector::parse(selector) {
            Ok(sel) => sel.match_element(self).is_some(),
            _ => false,
        }
    }

    pub fn style(&self) -> Rc<CssStyleDeclaration> {
        self.store.tree.borrow().data(self.id).el().style.clone()
    }
}

impl crate::css::Element for ElementRef {
    fn parent(&self) -> Option<Self> {
        self.parent_node()?.downcast::<ElementRef>()
    }

    fn local_name(&self) -> Atom<String> {
        ElementRef::local_name(self)
    }

    fn attribute(&self, name: &str) -> Option<Atom<String>> {
        // TODO: maybe "style" should not be attribute and then we could just delegate to self.attribute()
        for (a, v) in &self.store.tree.borrow().data(self.id).el().attributes {
            if a == name {
                return Some(v.clone());
            }
        }

        None
    }
}

impl CharacterDataRef {
    pub fn data(&self) -> String {
        self.store.tree.borrow().data(self.id).cdata().clone()
    }

    pub fn set_data(&self, data: &str) {
        *self.store.tree.borrow_mut().data_mut(self.id).cdata_mut() = data.to_owned()
    }
}

#[derive(Default)]
pub struct Store {
    tree: RefCell<IdTree<Node>>,
    style_sheets: RefCell<FnvHashMap<NodeId, Rc<CssStyleSheet>>>,
    listeners: RefCell<Vec<Rc<dyn Fn(&DomEvent)>>>,
}

impl Store {
    fn create_node(self: &Rc<Self>, data: NodeData) -> NodeRef {
        let id = self.tree.borrow_mut().create_node(Node {
            ref_count: Cell::new(1),
            data,
        });

        let node = NodeRef {
            store: Rc::clone(self),
            id,
        };

        self.emit(DomEvent::NodeCreated(&node));

        node
    }

    fn node_ref(self: &Rc<Self>, id: NodeId) -> NodeRef {
        self.inc_count(id);

        NodeRef {
            store: self.clone(),
            id,
        }
    }

    fn emit(&self, event: DomEvent) {
        for listener in &*self.listeners.borrow() {
            listener(&event);
        }
    }

    fn inc_count(&self, id: NodeId) {
        let tree = self.tree.borrow();
        tree.data(id).ref_count.set(tree.data(id).ref_count.get() + 1);
    }

    fn dec_count(&self, id: NodeId) {
        let prev = self.tree.borrow().data(id).ref_count.get();
        self.tree.borrow().data(id).ref_count.set(prev - 1);

        if prev == 1 {
            self.drop_node(id);
        }
    }

    fn drop_node(&self, id: NodeId) {
        // potentially drop whole subtree (first)
        let mut next = self.tree.borrow().first_child(id);
        while let Some(child) = next {
            next = self.tree.borrow().next_sibling(child);
            self.dec_count(child);
        }

        self.tree.borrow_mut().drop_node(id);

        self.emit(DomEvent::NodeDestroyed(id));
    }
}

struct Node {
    ref_count: Cell<u32>,
    data: NodeData,
}

enum NodeData {
    Document,
    Text(String),
    Comment(String),
    Element(ElementData),
}

struct ElementData {
    local_name: Atom<String>,
    attributes: Vec<(Atom<String>, Atom<String>)>,
    style: Rc<CssStyleDeclaration>,
}

impl Node {
    fn node_type(&self) -> NodeType {
        match self.data {
            NodeData::Element(_) => NodeType::Element,
            NodeData::Text(_) => NodeType::Text,
            NodeData::Comment(_) => NodeType::Comment,
            NodeData::Document => NodeType::Document,
        }
    }

    fn cdata(&self) -> &String {
        match &self.data {
            NodeData::Text(data) => data,
            NodeData::Comment(data) => data,
            _ => panic!("not cdata node"),
        }
    }

    fn cdata_mut(&mut self) -> &mut String {
        match &mut self.data {
            NodeData::Text(data) => data,
            NodeData::Comment(data) => data,
            _ => panic!("not cdata node"),
        }
    }

    fn el(&self) -> &ElementData {
        match &self.data {
            NodeData::Element(data) => data,
            _ => panic!("not el node"),
        }
    }

    fn el_mut(&mut self) -> &mut ElementData {
        match &mut self.data {
            NodeData::Element(data) => data,
            _ => panic!("not el node"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let doc = DocumentRef::new();
        assert_eq!(doc.node_type(), NodeType::Document);
        assert_eq!(doc.first_child(), None);

        let div = doc.create_element("div");
        assert_eq!(div.node_type(), NodeType::Element);
        assert_eq!(*div.local_name(), "div");
        assert_eq!(div.first_child(), None);

        let hello = doc.create_text_node("hello");
        assert_eq!(hello.node_type(), NodeType::Text);
        assert_eq!(hello.data(), "hello");
        hello.set_data("hello world");

        let comment = doc.create_comment("test");
        assert_eq!(comment.node_type(), NodeType::Comment);
        assert_eq!(comment.data(), "test");
        comment.set_data("test2");

        div.append_child(&hello);
        assert_eq!(div.first_child(), Some(hello.as_node()));

        div.append_child(&comment);
        assert_eq!(div.first_child(), Some(hello.as_node()));
        assert_eq!(hello.next_sibling(), Some(comment.as_node()));

        div.remove_child(&comment);
        assert_eq!(div.first_child(), Some(hello.as_node()));
        assert_eq!(div.next_sibling(), None);
    }

    #[test]
    fn qsa() {
        let doc = DocumentRef::new();
        let div = doc.create_element("div");

        div.set_attribute("id", "panel");
        assert_eq!(div.attribute("id").as_deref(), Some("panel"));

        // even before connecting, browsers do the same
        assert!(div.matches("div#panel"));

        doc.append_child(&div);
        assert_eq!(doc.query_selector("div#panel"), Some(div));
    }

    #[test]
    fn inline_style() {
        let doc = DocumentRef::new();
        let div = doc.create_element("div");

        div.set_attribute("style", "display: block");
        assert_eq!(div.style().css_text(), "display:block;");

        div.style().set_property("width", "100px");
        assert_eq!(div.attribute("style").as_deref(), Some("display:block;width:100px;"));

        div.remove_attribute("style");
        assert_eq!(div.style().css_text(), "");
    }

    #[test]
    fn downcast() {
        let doc = DocumentRef::new();
        let div = doc.create_element("div");

        let node = doc.as_node();
        assert!(node.downcast::<DocumentRef>().is_some());

        let node = div.as_node();
        assert!(node.downcast::<DocumentRef>().is_none());

        let node = div.as_node();
        assert!(node.downcast::<ElementRef>().is_some());
    }
}
