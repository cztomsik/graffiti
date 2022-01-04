use crate::html::{parse_html, HtmlNode};
use crate::util::{Atom, Edge, IdTree, Node};
use std::borrow::Cow;
use std::fmt;
use std::ops::{Index, IndexMut};

pub use crate::util::NodeId;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
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

    pub fn from_html(html: &str) -> Self {
        let mut doc = Self::new();
        let mut stack: Vec<(NodeId, HtmlNode)> = parse_html(html)
            .unwrap()
            .into_iter()
            .map(|n| (doc.root(), n))
            .rev()
            .collect();

        while let Some((parent, node)) = stack.pop() {
            let node = match node {
                HtmlNode::Text(data) => doc.create_text_node(data),
                HtmlNode::Comment(data) => doc.create_comment(data),
                HtmlNode::Element {
                    local_name,
                    attributes,
                    children,
                } => {
                    let el = doc.create_element(local_name);

                    for (att, value) in attributes {
                        doc[el].el_mut().set_attribute(att, value);
                    }

                    for ch in children.into_iter().rev() {
                        stack.push((el, ch));
                    }

                    el
                }
            };

            doc.append_child(parent, node);
        }

        doc
    }

    #[allow(clippy::unused_self)]
    pub fn root(&self) -> NodeId {
        NodeId::new(1).unwrap()
    }

    pub fn create_element(&mut self, local_name: &str) -> NodeId {
        self.create_node(DomData::Element(ElementData {
            local_name: local_name.into(),
            attributes: Vec::default(),
            style: CssStyleDeclaration::default(),
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

    pub fn matches(&self, node: NodeId, selector: &str) -> bool {
        match Selector::parse(selector) {
            Ok(sel) => todo!(),//sel.match_element(self).is_some(),
            _ => false,
        }
    }

    pub fn query_selector(&self, node: NodeId, selector: &str) -> Option<NodeId> {
        self.query_selector_all(node, selector).next()
    }

    pub fn query_selector_all(&self, node: NodeId, selector: &str) -> impl Iterator<Item = NodeId> {
        let selector = Selector::parse(selector).unwrap_or(Selector::unsupported());
        // let els = self.descendants().into_iter().filter_map(|node| node.downcast::<ElementRef>());
        // els.filter(|el| selector.match_element(el).is_some()).collect()
        let res: Vec<NodeId> = todo!();
        res.into_iter()
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

    // private (at leats for now)
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
        &self.tree[node]
    }
}

impl IndexMut<NodeId> for Document {
    fn index_mut(&mut self, node: NodeId) -> &mut DomNode {
        self.changes.push(Change::Changed(node));

        &mut self.tree[node]
    }
}

pub enum DomData {
    Element(ElementData),
    Text(String),
    Comment(String),
    Document,
}

impl DomData {
    pub fn node_type(&self) -> NodeType {
        match self {
            DomData::Element(_) => NodeType::Element,
            DomData::Text(_) => NodeType::Text,
            DomData::Comment(_) => NodeType::Comment,
            DomData::Document => NodeType::Document,
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
    local_name: Atom<String>,
    attributes: Vec<(Atom<String>, Atom<String>)>,
    style: CssStyleDeclaration,
}

impl ElementData {
    pub fn local_name(&self) -> &str {
        &**self.local_name
    }

    pub fn attribute_names(&self) -> impl Iterator<Item = &str> {
        let style = (self.style.length() > 0).then(|| "style");

        self.attributes.iter().map(|(k, _)| &***k).chain(style.into_iter())
    }

    pub fn attribute(&self, attr: &str) -> Option<Cow<str>> {
        match attr {
            "style" => Some(Cow::Owned(self.style.to_string())),
            _ => self
                .attributes
                .iter()
                .find(|(a, _)| attr == **a)
                .map(|(_, v)| Cow::Borrowed(&***v)),
        }
    }

    pub fn set_attribute(&mut self, attr: &str, value: &str) {
        match attr {
            "style" => self.style.set_css_text(value),
            _ => {
                if let Some(a) = self.attributes.iter_mut().find(|(a, _)| attr == **a) {
                    a.1 = value.into();
                } else {
                    self.attributes.push((attr.into(), value.into()));
                }
            }
        }
    }

    pub fn remove_attribute(&mut self, attr: &str) {
        match attr {
            "style" => self.style.set_css_text(""),
            _ => self.attributes.retain(|(a, _)| attr != **a),
        };
    }

    pub fn style(&self) -> &CssStyleDeclaration {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut CssStyleDeclaration {
        &mut self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut doc = Document::new();
        assert_eq!(doc[doc.root()].node_type(), NodeType::Document);
        assert_eq!(doc[doc.root()].first_child(), None);

        let div = doc.create_element("div");
        assert_eq!(doc[div].node_type(), NodeType::Element);
        assert_eq!(doc[div].el().local_name(), "div");
        assert_eq!(doc[div].first_child(), None);

        let hello = doc.create_text_node("hello");
        assert_eq!(doc[hello].node_type(), NodeType::Text);
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
        assert_eq!(doc[div].el().style().css_text(), "display:block;");

        doc[div].el_mut().style_mut().set_property("width", "100px");
        assert_eq!(
            doc[div].el().attribute("style").as_deref(),
            Some("display:block;width:100px;")
        );

        doc[div].el_mut().remove_attribute("style");
        assert_eq!(doc[div].el().style().css_text(), "");
    }
}
