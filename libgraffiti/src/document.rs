// TODO: move rest of this to dom.rs

// observable model
// x holds the data/truth (tree of nodes)
// x allows changes
// x notifies listener
// x panics for invalid node types
//  (another layer on top of this should make sure it never happens)

use crate::css::{MatchingContext, Selector, CssStyleDeclaration};
use crate::util::{Atom, SlotMap};
use std::any::Any;
use std::borrow::Cow;

pub type NodeId = u32;

#[derive(Debug)]
pub enum DocumentEvent<'a> {
    Create(NodeId, NodeType),
    Insert(NodeId, NodeId, usize),
    Remove(NodeId, NodeId),
    Cdata(NodeId, &'a str),

    // TODO: call during Document::Drop, probably in document order (children first)
    Drop(NodeId, NodeType),
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
type Event<'a> = DocumentEvent<'a>;

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

    pub fn root(&self) -> NodeId {
        self.root
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
        let node_type = self.node_type(node);
        drop(self.nodes.remove(node));
        self.weak_data.remove(node);
        self.free_ids.push(node);

        self.emit(Event::Drop(node, node_type));
    }


    // element

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
            "style" => el_data.style = CssStyleDeclaration::EMPTY,
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

    pub fn element_style(&self, element: NodeId) -> &CssStyleDeclaration {
        &self.el(element).style
    }

    pub fn element_style_property_value(&self, element: NodeId, prop: &str) -> Option<String> {
        self.el(element).style.property_value(prop)
    }

    pub fn set_element_style_property(&mut self, element: NodeId, prop: &str, value: &str) {
        self.el_mut(element).style.set_property(prop, value);
    }

    // helpers

    fn create_node(&mut self, data: NodeData) -> NodeId {
        let node = Node {
            parent: None,
            first_child: None,
            next_sibling: None,
            data,
        };

        if let Some(id) = self.free_ids.pop() {
            self.nodes.put(id, node);
            self.weak_data.put(id, Vec::new());
            return id
        }

        let id = self.nodes.insert(node);

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
            parent: &|el| self.parent_element(el),
        })
    }

    fn emit(&self, event: Event) {
        for listener in &self.listeners {
            listener(self, &event);
        }
    }
}

struct ElementData {
    local_name: Atom<String>,
    identifier: Option<Atom<String>>,
    class_name: Option<Atom<String>>,
    style: CssStyleDeclaration,
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
