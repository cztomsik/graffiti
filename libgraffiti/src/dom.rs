// document model
// x holds the data/truth (tree of nodes)
// x allows changes
// x qs(a)
// x weak data

use crate::css::{CssStyleDeclaration, MatchingContext, Selector};
use crate::util::{Atom, Bloom, Edge, IdTree};
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
        self.store
            .tree
            .borrow()
            .parent_node(self.id)
            .map(|id| self.store.node_ref(id))
    }

    pub fn first_child(&self) -> Option<NodeRef> {
        self.store
            .tree
            .borrow()
            .first_child(self.id)
            .map(|id| self.store.node_ref(id))
    }

    pub fn last_child(&self) -> Option<NodeRef> {
        self.store
            .tree
            .borrow()
            .last_child(self.id)
            .map(|id| self.store.node_ref(id))
    }

    pub fn previous_sibling(&self) -> Option<NodeRef> {
        self.store
            .tree
            .borrow()
            .previous_sibling(self.id)
            .map(|id| self.store.node_ref(id))
    }

    pub fn next_sibling(&self) -> Option<NodeRef> {
        self.store
            .tree
            .borrow()
            .next_sibling(self.id)
            .map(|id| self.store.node_ref(id))
    }

    pub fn append_child(&self, child: &NodeRef) {
        self.store.tree.borrow_mut().append_child(self.id, child.id);

        // TODO: add to damage
        child.update_ancestors();
        child.inc_count();
    }

    pub fn insert_before(&self, child: &NodeRef, before: &NodeRef) {
        self.store.tree.borrow_mut().insert_before(self.id, child.id, before.id);

        // TODO: add to damage
        child.update_ancestors();
        child.inc_count();
    }

    pub fn remove_child(&self, child: &NodeRef) {
        self.store.tree.borrow_mut().remove_child(self.id, child.id);

        // TODO: add to damage
        child.clear_ancestors();
        child.dec_count();
    }

    pub fn query_selector(&self, selector: &str) -> Option<ElementRef> {
        self.query_selector_all(selector).get(0).cloned()
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<ElementRef> {
        let selector = Selector::from(selector);
        let tree = self.store.tree.borrow();
        let els = tree.traverse(self.id).skip(1).filter_map(|edge| match edge {
            Edge::Start(node) if self.store.tree.borrow().data(node).node_type() == NodeType::Element => Some(node),
            _ => None,
        });

        self.with_matching_context(|ctx| {
            els.into_iter()
                .filter(|&el| ctx.match_selector(&selector, el).is_some())
                .map(|el| self.store.node_ref(el).downcast::<ElementRef>().unwrap())
                .collect()
        })
    }

    pub fn as_node(&self) -> NodeRef {
        self.store.node_ref(self.id)
    }

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

    pub fn downcast<T: Clone + 'static>(self) -> Option<T> {
        self.downcast_ref().cloned()
    }

    // helpers

    fn update_ancestors(&self) {
        for edge in self.store.tree.borrow().traverse(self.id) {
            if let Edge::Start(node) = edge {
                let parent = self.store.tree.borrow().parent_node(node).unwrap();
                self.store
                    .tree
                    .borrow()
                    .data(node)
                    .ancestors
                    .set(self.store.tree.borrow().data(parent).ancestors.get().with(&parent));
            }
        }
    }

    fn clear_ancestors(&self) {
        for edge in self.store.tree.borrow().traverse(self.id) {
            if let Edge::Start(node) = edge {
                self.store.tree.borrow().data(node).ancestors.take();
            }
        }
    }

    fn inc_count(&self) {
        self.store.inc_count(self.id);
    }

    fn dec_count(&self) {
        self.store.dec_count(self.id);
    }

    pub(crate) fn with_matching_context<R, F: FnOnce(MatchingContext<'_, NodeId>) -> R>(&self, f: F) -> R {
        let tree = self.store.tree.borrow();

        f(MatchingContext {
            has_local_name: &|el, name| name == &tree.data(el).el().local_name,
            has_identifier: &|el, id| Some(id) == tree.data(el).el().identifier.as_ref(),
            has_class: &|el, cls| match &tree.data(el).el().class_name {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                None => false,
            },
            parent: &|el| tree.parent_node(el),
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
        self.dec_count()
    }
}

impl Debug for NodeRef {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.debug_tuple("NodeRef").field(&self.id).finish()
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

// TODO: move to util?
// (we might use it for CSS too)
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

    pub fn create_element(&self, local_name: &str) -> ElementRef {
        ElementRef(self.store.create_node(NodeData::Element(ElementData {
            local_name: local_name.into(),
            identifier: None,
            class_name: None,
            style: CssStyleDeclaration::new(),
            attributes: Vec::new(),
        })))
    }

    pub fn create_text_node(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(self.store.create_node(NodeData::Text(data.to_owned())))
    }

    pub fn create_comment(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(self.store.create_node(NodeData::Comment(data.to_owned())))
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

        if el.identifier.is_some() {
            names.push("id".to_owned());
        }

        if el.class_name.is_some() {
            names.push("class".to_owned());
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
            "id" => el.identifier.as_deref().cloned(),
            "class" => el.class_name.as_deref().cloned(),
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
            "id" => el.identifier = Some(value.into()),
            "class" => el.class_name = Some(value.into()),
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
            "id" => drop(el.identifier.take()),
            "class" => drop(el.identifier.take()),
            "style" => el.style = CssStyleDeclaration::EMPTY,
            _ => el.attributes.retain(|(a, _)| attr != **a),
        };
    }

    pub fn matches(&self, selector: &str) -> bool {
        self.with_matching_context(|ctx| ctx.match_selector(&Selector::from(selector), self.id).is_some())
    }

    //pub fn style() -> Rc<CssStyleDeclaration> { todo!() }
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
}

impl Store {
    fn create_node(self: &Rc<Self>, data: NodeData) -> NodeRef {
        let id = self.tree.borrow_mut().create_node(Node {
            ref_count: Cell::new(1),
            ancestors: Cell::new(Bloom::new()),
            data,
        });
        NodeRef {
            store: Rc::clone(self),
            id,
        }
    }

    fn node_ref(self: &Rc<Self>, id: NodeId) -> NodeRef {
        self.inc_count(id);

        NodeRef {
            store: self.clone(),
            id,
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
    }
}

struct Node {
    ref_count: Cell<u32>,
    ancestors: Cell<Bloom<NodeId>>,
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
    identifier: Option<Atom<String>>,
    class_name: Option<Atom<String>>,
    // TODO: Rc<>?
    style: CssStyleDeclaration,
    attributes: Vec<(Atom<String>, Atom<String>)>,
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
        //assert_eq!(div.style().css_text(), "display: block");

        //div.style().set_property("width", "100px");
        assert_eq!(div.attribute("style").as_deref(), Some("display: block; width: 100px"));

        div.remove_attribute("style");
        //assert_eq!(div.style().css_text(), "");
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
