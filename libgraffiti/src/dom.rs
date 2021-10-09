// document model
// x holds the data/truth (tree of nodes)
// x allows changes
// x qs(a)
// x weak data

use crate::css::{CssStyleDeclaration, MatchingContext, Selector};
use crate::util::{Atom, Bloom, SlotMap};
use std::any::TypeId;
use std::cell::{Cell, RefCell, Ref};
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

pub type NodeId = u32;

pub struct NodeRef {
    store: Rc<Store>,
    id: NodeId,
}

impl NodeRef {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn node_type(&self) -> NodeType {
        self.store.nodes.borrow()[self.id].node_type()
    }

    pub fn parent_node(&self) -> Option<NodeRef> {
        self.store.nodes.borrow()[self.id]
            .parent_node
            .get()
            .map(|id| self.store.node_ref(id))
    }

    pub fn first_child(&self) -> Option<NodeRef> {
        self.store.nodes.borrow()[self.id]
            .first_child
            .get()
            .map(|id| self.store.node_ref(id))
    }

    pub fn last_child(&self) -> Option<NodeRef> {
        self.store.nodes.borrow()[self.id]
            .last_child
            .get()
            .map(|id| self.store.node_ref(id))
    }

    pub fn previous_sibling(&self) -> Option<NodeRef> {
        self.store.nodes.borrow()[self.id]
            .previous_sibling
            .get()
            .map(|id| self.store.node_ref(id))
    }

    pub fn next_sibling(&self) -> Option<NodeRef> {
        self.store.nodes.borrow()[self.id]
            .next_sibling
            .get()
            .map(|id| self.store.node_ref(id))
    }

    pub fn append_child(&self, child: &NodeRef) {
        let nodes = self.store.nodes.borrow();

        if nodes[self.id].first_child.get() == None {
            nodes[self.id].first_child.set(Some(child.id))
        }

        if let Some(last) = nodes[self.id].last_child.get() {
            nodes[last].next_sibling.set(Some(child.id));
        }

        nodes[child.id].previous_sibling.set(nodes[self.id].last_child.get());
        nodes[self.id].last_child.set(Some(child.id));
        nodes[child.id].parent_node.set(Some(self.id));

        // TODO: add to damaged
        child.update_ancestors();
        child.inc_count();
    }

    // TODO: test
    pub fn insert_before(&self, child: &NodeRef, before: &NodeRef) {
        let nodes = self.store.nodes.borrow();

        if nodes[self.id].first_child.get() == Some(before.id) {
            nodes[self.id].first_child.set(Some(child.id))
        }

        nodes[before.id].previous_sibling.set(Some(child.id));

        nodes[child.id].next_sibling.set(Some(before.id));
        nodes[child.id].parent_node.set(Some(self.id));

        // TODO: add to damaged
        child.update_ancestors();
        child.inc_count();
    }

    pub fn remove_child(&self, child: &NodeRef) {
        let nodes = self.store.nodes.borrow();

        if nodes[self.id].first_child.get() == Some(child.id) {
            nodes[self.id].first_child.set(nodes[child.id].next_sibling.get())
        }

        if nodes[self.id].last_child.get() == Some(child.id) {
            nodes[self.id].last_child.set(nodes[child.id].previous_sibling.get())
        }

        if let Some(prev) = nodes[child.id].previous_sibling.get() {
            nodes[prev].next_sibling.set(nodes[child.id].next_sibling.get());
        }

        if let Some(next) = nodes[child.id].next_sibling.get() {
            nodes[next].previous_sibling.set(nodes[child.id].previous_sibling.get());
        }

        nodes[child.id].parent_node.set(None);
        nodes[child.id].next_sibling.set(None);
        nodes[child.id].previous_sibling.set(None);

        child.clear_ancestors();
        child.dec_count();
    }

    pub fn query_selector(&self, selector: &str) -> Option<ElementRef> {
        self.query_selector_all(selector).get(0).cloned()
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<ElementRef> {
        let selector = Selector::from(selector);
        let els = self.descendants().filter_map(|edge| match edge {
            NodeEdge::Start(node) if self.store.nodes.borrow()[node].node_type() == NodeType::Element => Some(node),
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
        self.downcast_ref().map(Clone::clone)
    }

    // helpers

    fn descendants(&self) -> Traverse {
        Traverse {
            nodes: self.store.nodes.borrow(),
            next: self.store.nodes.borrow()[self.id]
                .first_child
                .get()
                .map(NodeEdge::Start),
        }
    }

    fn descendants_and_self(&self) -> Traverse {
        // TODO: Traverse::from(self), Traverse::from(self.first_child())
        //       but then we would need Option<> for nodes ref
        Traverse {
            nodes: self.store.nodes.borrow(),
            next: Some(NodeEdge::Start(self.id)),
        }
    }

    fn update_ancestors(&self) {
        let nodes = self.store.nodes.borrow();

        for edge in self.descendants_and_self() {
            if let NodeEdge::Start(node) = edge {
                let parent = nodes[node].parent_node.get().unwrap();
                nodes[node].ancestors.set(nodes[parent].ancestors.get().with(&parent));
            }
        }
    }

    fn clear_ancestors(&self) {
        let nodes = self.store.nodes.borrow();

        for edge in self.descendants_and_self() {
            if let NodeEdge::Start(node) = edge {
                nodes[node].ancestors.take();
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
        let nodes = self.store.nodes.borrow();

        f(MatchingContext {
            has_local_name: &|el, name| name == &nodes[el].el().local_name,
            has_identifier: &|el, id| Some(id) == nodes[el].el().identifier.as_ref(),
            has_class: &|el, cls| match &nodes[el].el().class_name {
                Some(s) => s.split_ascii_whitespace().any(|part| part == **cls),
                None => false,
            },
            parent: &|el| nodes[el].parent_node.get(),
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
        DocumentRef(create_node(&Default::default(), NodeData::Document))
    }

    pub fn create_element(&self, local_name: &str) -> ElementRef {
        ElementRef(create_node(
            &self.store,
            NodeData::Element(ElementData {
                local_name: local_name.into(),
                identifier: None,
                class_name: None,
                style: CssStyleDeclaration::new(),
                attributes: Vec::new(),
            }),
        ))
    }

    pub fn create_text_node(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(create_node(&self.store, NodeData::Text(data.to_owned())))
    }

    pub fn create_comment(&self, data: &str) -> CharacterDataRef {
        CharacterDataRef(create_node(&self.store, NodeData::Comment(data.to_owned())))
    }
}

impl ElementRef {
    pub fn local_name(&self) -> Atom<String> {
        self.store.nodes.borrow()[self.id].el().local_name.clone()
    }

    pub fn attribute_names(&self) -> Vec<String> {
        let nodes = self.store.nodes.borrow();
        let el = nodes[self.id].el();
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
        let nodes = self.store.nodes.borrow();
        let el = nodes[self.id].el();

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
        let mut nodes = self.store.nodes.borrow_mut();
        let mut el = nodes[self.id].el_mut();

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
        let mut nodes = self.store.nodes.borrow_mut();
        let mut el = nodes[self.id].el_mut();

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
        self.store.nodes.borrow()[self.id].cdata().clone()
    }

    pub fn set_data(&self, data: &str) {
        *self.store.nodes.borrow_mut()[self.id].cdata_mut() = data.to_owned()
    }
}

#[derive(Default)]
pub struct Store {
    nodes: RefCell<SlotMap<NodeId, Node>>,

    // TODO: move to slotmap?
    free_ids: RefCell<Vec<NodeId>>,
}

impl Store {
    fn node_ref(self: &Rc<Self>, id: NodeId) -> NodeRef {
        self.inc_count(id);

        NodeRef {
            store: self.clone(),
            id,
        }
    }

    fn inc_count(&self, id: NodeId) {
        let node = &self.nodes.borrow()[id];
        node.ref_count.set(node.ref_count.get() + 1);
    }

    fn dec_count(&self, id: NodeId) {
        let prev = self.nodes.borrow()[id].ref_count.get();
        self.nodes.borrow()[id].ref_count.set(prev - 1);

        if prev == 1 {
            self.drop_node(id);
        }
    }

    fn drop_node(&self, id: NodeId) {
        // potentially drop whole subtree (first)
        let mut next = self.nodes.borrow()[id].first_child.take();
        while let Some(child) = next {
            next = self.nodes.borrow()[child].next_sibling.take();
            self.dec_count(child);
        }

        self.nodes.borrow_mut().remove(id);
        self.free_ids.borrow_mut().push(id);
    }
}

struct Node {
    parent_node: Cell<Option<NodeId>>,
    first_child: Cell<Option<NodeId>>,
    next_sibling: Cell<Option<NodeId>>,
    previous_sibling: Cell<Option<NodeId>>,
    last_child: Cell<Option<NodeId>>,
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

fn create_node(store: &Rc<Store>, data: NodeData) -> NodeRef {
    let mut free_ids = store.free_ids.borrow_mut();
    let mut nodes = store.nodes.borrow_mut();

    let node = Node {
        parent_node: Cell::new(None),
        first_child: Cell::new(None),
        next_sibling: Cell::new(None),
        previous_sibling: Cell::new(None),
        last_child: Cell::new(None),
        ref_count: Cell::new(1),
        ancestors: Cell::new(Bloom::new()),
        data,
    };

    let id = if let Some(id) = free_ids.pop() {
        nodes.put(id, node);
        id
    } else {
        nodes.insert(node)
    };

    NodeRef {
        store: Rc::clone(store),
        id,
    }
}

#[derive(Clone, Debug)]
enum NodeEdge {
    Start(NodeId),
    End(NodeId),
}

struct Traverse<'a> {
    nodes: Ref<'a, SlotMap<NodeId, Node>>,
    next: Option<NodeEdge>,
}

impl Iterator for Traverse<'_> {
    type Item = NodeEdge;

    fn next(&mut self) -> Option<NodeEdge> {
        match self.next.take() {
            Some(next) => {
                self.next = match next {
                    NodeEdge::Start(node) => match self.nodes[node].first_child.get() {
                        Some(first_child) => Some(NodeEdge::Start(first_child)),
                        None => Some(NodeEdge::End(node)),
                    },
                    NodeEdge::End(node) => match self.nodes[node].next_sibling.get() {
                        Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                        None => match self.nodes[node].parent_node.get() {
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
    fn tree() {
        let doc = DocumentRef::new();
        assert_eq!(doc.parent_node(), None);
        assert_eq!(doc.first_child(), None);
        assert_eq!(doc.next_sibling(), None);
        assert_eq!(doc.previous_sibling(), None);

        let ch1 = doc.create_text_node("ch1");
        let ch2 = doc.create_text_node("ch2");
        let ch3 = doc.create_text_node("ch3");

        doc.append_child(&ch1);
        assert_eq!(doc.first_child(), Some(ch1.clone().as_node()));
        assert_eq!(ch1.parent_node(), Some(doc.clone().as_node()));
        assert_eq!(ch1.next_sibling(), None);
        assert_eq!(ch1.previous_sibling(), None);

        doc.append_child(&ch2);
        assert_eq!(doc.first_child(), Some(ch1.as_node()));
        assert_eq!(ch1.next_sibling(), Some(ch2.as_node()));
        assert_eq!(ch2.previous_sibling(), Some(ch1.as_node()));

        //assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2]);

        doc.insert_before(&ch3, &ch1);

        //assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3, ch1, ch2]);

        doc.remove_child(&ch1);
        doc.remove_child(&ch2);

        //assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch3]);

        doc.insert_before(&ch2, &ch3);
        doc.insert_before(&ch1, &ch2);

        //assert_eq!(doc.child_nodes(root).collect::<Vec<_>>(), vec![ch1, ch2, ch3]);
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
