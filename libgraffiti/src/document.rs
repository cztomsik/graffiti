use crate::css::{CssStyle, MatchingContext, Selector};
use crate::util::{Atom, Edge, Id, IdTree, Node};
use std::borrow::Cow;
use std::fmt;
use std::num::NonZeroU32;
use std::ops::{Index, IndexMut};

pub type NodeId = Id<DomNode>;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Element = 1,
    Text = 3,
    Comment = 8,
    Document = 9,
}

type DomNode = Node<DomData>;

#[derive(Clone, Copy)]
pub enum Change {
    Created(NodeId),
    Destroyed(NodeId),
    Changed(NodeId),
    Inserted(NodeId),
    Removed(NodeId),
}

/// Parsed tree with stable ids and built-in change-tracking
pub struct Document {
    tree: IdTree<DomData>,
    changes: Vec<Change>,
}

impl Document {
    pub fn new() -> Self {
        let mut doc = Document {
            tree: IdTree::default(),
            changes: Vec::default(),
        };

        assert_eq!(doc.root(), doc.create_node(DomData::Document));

        doc
    }

    #[allow(clippy::unused_self)]
    pub fn root(&self) -> NodeId {
        NodeId::new(NonZeroU32::new(1).unwrap())
    }

    pub fn node(&self, node: NodeId) -> &DomNode {
        &self.tree[node]
    }

    pub fn node_mut(&mut self, node: NodeId) -> &mut DomNode {
        self.changes.push(Change::Changed(node));
        &mut self.tree[node]
    }

    pub fn create_element(&mut self, local_name: &str) -> NodeId {
        self.create_node(DomData::Element(ElementData {
            local_name: local_name.into(),
            attributes: Vec::default(),
            style: CssStyle::default(),
        }))
    }

    pub fn children(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.tree.children(node)
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        self.changes.push(Change::Inserted(child));
        self.tree.append_child(parent, child);
    }

    pub fn insert_before(&mut self, parent: NodeId, child: NodeId, before: NodeId) {
        self.changes.push(Change::Inserted(child));
        self.tree.insert_before(parent, child, before);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.changes.push(Change::Removed(child));
        self.tree.remove_child(parent, child);
    }

    pub fn traverse(&self, node: NodeId) -> impl Iterator<Item = Edge<NodeId>> + '_ {
        self.tree.traverse(node)
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
        let selector = Selector::parse(selector).unwrap_or_else(|_| Selector::unsupported());

        self.traverse(node).filter_map(move |e| match e {
            Edge::Start(node)
                if self[node].kind() == NodeKind::Element && self.match_selector(&selector, node).is_some() =>
            {
                Some(node)
            }
            _ => None,
        })
    }

    pub fn create_text_node(&mut self, data: &str) -> NodeId {
        self.create_node(DomData::Text(data.to_owned()))
    }

    pub fn create_comment(&mut self, data: &str) -> NodeId {
        self.create_node(DomData::Comment(data.to_owned()))
    }

    pub fn text_content(&self, node: NodeId) -> Cow<'_, str> {
        match &self[node].data() {
            DomData::Text(data) => Cow::Borrowed(data),
            DomData::Comment(_) => Cow::Borrowed(""),
            _ => self
                .children(node)
                .fold(Cow::Borrowed(""), |res, ch| res + self.text_content(ch)),
        }
    }

    pub fn drop_node(&mut self, node: NodeId) {
        self.changes.push(Change::Destroyed(node));
        self.tree.drop_node(node);
    }

    // private (at least for now)
    pub(crate) fn take_changes(&mut self) -> Vec<Change> {
        std::mem::take(&mut self.changes)
    }

    // helpers
    fn create_node(&mut self, data: DomData) -> NodeId {
        let id = self.tree.create_node(data);
        self.changes.push(Change::Created(id));

        id
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

impl Index<NodeId> for Document {
    type Output = DomNode;

    fn index(&self, node: NodeId) -> &DomNode {
        self.node(node)
    }
}

impl IndexMut<NodeId> for Document {
    fn index_mut(&mut self, node: NodeId) -> &mut DomNode {
        self.node_mut(node)
    }
}

pub enum DomData {
    Element(ElementData),
    Text(String),
    Comment(String),
    Document,
}

impl DomData {
    pub fn kind(&self) -> NodeKind {
        match self {
            DomData::Element(_) => NodeKind::Element,
            DomData::Text(_) => NodeKind::Text,
            DomData::Comment(_) => NodeKind::Comment,
            DomData::Document => NodeKind::Document,
        }
    }

    pub fn el(&self) -> &ElementData {
        match self {
            DomData::Element(el) => el,
            _ => panic!("not an element"),
        }
    }

    pub fn el_mut(&mut self) -> &mut ElementData {
        match self {
            DomData::Element(el) => el,
            _ => panic!("not an element"),
        }
    }

    pub fn text(&self) -> &str {
        match self {
            DomData::Text(s) | DomData::Comment(s) => s,
            _ => panic!("not a text/comment"),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        match self {
            DomData::Text(s) | DomData::Comment(s) => *s = text.to_owned(),
            _ => panic!("not a text/comment"),
        }
    }
}

pub struct ElementData {
    local_name: Atom,
    attributes: Vec<(Atom, Atom)>,
    style: CssStyle,
}

impl ElementData {
    pub fn local_name(&self) -> &str {
        &*self.local_name
    }

    pub fn attribute_names(&self) -> impl Iterator<Item = &str> {
        let style = (self.style.length() > 0).then(|| "style");

        self.attributes.iter().map(|(k, _)| &**k).chain(style.into_iter())
    }

    pub fn attribute(&self, attr: &str) -> Option<Cow<str>> {
        match attr {
            "style" => Some(Cow::Owned(self.style.to_string())),
            _ => self
                .attributes
                .iter()
                .find(|(a, _)| attr == &**a)
                .map(|(_, v)| Cow::Borrowed(&**v)),
        }
    }

    pub fn set_attribute(&mut self, attr: &str, value: &str) {
        match attr {
            "style" => self.style = CssStyle::parse(value).unwrap_or_default(),
            _ => {
                if let Some(a) = self.attributes.iter_mut().find(|(a, _)| attr == &**a) {
                    a.1 = value.into();
                } else {
                    self.attributes.push((attr.into(), value.into()));
                }
            }
        }
    }

    pub fn remove_attribute(&mut self, attr: &str) {
        match attr {
            "style" => self.style = CssStyle::default(),
            _ => self.attributes.retain(|(a, _)| attr != &**a),
        };
    }

    pub fn style(&self) -> &CssStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut CssStyle {
        &mut self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut doc = Document::new();
        assert_eq!(doc[doc.root()].kind(), NodeKind::Document);
        assert_eq!(doc[doc.root()].first_child(), None);

        let div = doc.create_element("div");
        assert_eq!(doc[div].kind(), NodeKind::Element);
        assert_eq!(doc[div].el().local_name(), "div");
        assert_eq!(doc[div].first_child(), None);

        let hello = doc.create_text_node("hello");
        assert_eq!(doc[hello].kind(), NodeKind::Text);
        assert_eq!(doc[hello].text(), "hello");
        doc[hello].set_text("hello world");
        assert_eq!(doc[hello].text(), "hello world");

        let other = doc.create_text_node("test");

        doc.append_child(div, hello);
        assert_eq!(doc[div].first_child(), Some(hello));

        doc.append_child(div, other);
        assert_eq!(doc[div].first_child(), Some(hello));
        assert_eq!(doc[hello].next_sibling(), Some(other));

        doc.remove_child(div, other);
        assert_eq!(doc[div].first_child(), Some(hello));
        assert_eq!(doc[div].next_sibling(), None);
    }

    #[test]
    fn qsa() {
        let mut doc = Document::new();
        let div = doc.create_element("div");

        doc[div].el_mut().set_attribute("id", "panel");
        assert_eq!(doc[div].el().attribute("id").as_deref(), Some("panel"));

        // even before connecting, browsers do the same
        assert!(doc.matches(div, "div#panel"));

        doc.append_child(doc.root(), div);
        assert_eq!(doc.query_selector(doc.root(), "div#panel"), Some(div));
    }

    #[test]
    fn inline_style() {
        let mut doc = Document::new();
        let div = doc.create_element("div");

        doc[div].el_mut().set_attribute("style", "display: block");
        assert_eq!(doc[div].el().style().to_string(), "display:block;");

        doc[div].el_mut().style_mut().set_property("width", "100px");
        assert_eq!(
            doc[div].el().attribute("style").as_deref(),
            Some("display:block;width:100px;")
        );

        doc[div].el_mut().remove_attribute("style");
        assert_eq!(doc[div].el().style().to_string(), "");
    }
}
