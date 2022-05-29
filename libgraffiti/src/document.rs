use crate::css::{MatchingContext, Selector, Style};
use crate::util::Atom;
use std::borrow::Cow;
use std::fmt;
use std::ops::{Index, IndexMut};

pub type NodeId = usize;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Element = 1,
    Text = 3,
    Document = 9,
}

pub struct Document {
    nodes: Vec<DomData>,
    parents: Vec<Option<NodeId>>,
    children: Vec<Vec<NodeId>>,
}

impl Document {
    pub const ROOT: NodeId = 0;

    pub fn new() -> Self {
        let mut doc = Document {
            nodes: Vec::default(),
            parents: Vec::default(),
            children: Vec::default(),
        };

        assert_eq!(Self::ROOT, doc.create_node(DomData::Document));

        doc
    }

    pub fn node(&self, node: NodeId) -> &DomData {
        &self.nodes[node]
    }

    pub fn node_mut(&mut self, node: NodeId) -> &mut DomData {
        &mut self.nodes[node]
    }

    pub fn create_element(&mut self, local_name: impl Into<Atom>) -> NodeId {
        self.create_node(DomData::Element(ElementData {
            local_name: local_name.into(),
            attributes: Vec::default(),
            style: Style::default(),
        }))
    }

    pub fn children(&self, node: NodeId) -> &[NodeId] {
        &self.children[node]
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        assert_eq!(self.parents[child], None);
        self.children[parent].push(child);
        self.parents[child] = Some(parent);
    }

    pub fn insert_before(&mut self, parent: NodeId, child: NodeId, before: NodeId) {
        assert_eq!(self.parents[child], None);
        assert_eq!(self.parents[before], Some(parent));

        let index = self.children[parent].iter().position(|ch| *ch == before).unwrap();
        self.children[parent].insert(index, child);
        self.parents[child] = Some(parent);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        assert_eq!(self.parents[child], Some(parent));
        self.children[parent].retain(|ch| *ch != child);
        self.parents[child] = None;
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
        todo!();
        Vec::new().iter().copied()
        // let selector = Selector::parse(selector).unwrap_or_else(|_| Selector::unsupported());

        // self.descendant_elements(node)
        //     .filter(|e| self.match_selector(&selector, node).is_some())
    }

    pub fn create_text_node(&mut self, data: &str) -> NodeId {
        self.create_node(DomData::Text(data.to_owned()))
    }

    pub fn text_content(&self, node: NodeId) -> Cow<'_, str> {
        match &self[node] {
            DomData::Text(data) => Cow::Borrowed(data),
            _ => self
                .children(node)
                .iter()
                .fold(Cow::Borrowed(""), |res, &ch| res + self.text_content(ch)),
        }
    }

    pub fn drop_node(&mut self, node: NodeId) {
        self.nodes.remove(node);
    }

    // helpers
    fn create_node(&mut self, data: DomData) -> NodeId {
        let id = self.nodes.len();

        self.nodes.push(data);
        self.parents.push(None);
        self.children.push(Vec::new());

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
    type Output = DomData;

    fn index(&self, node: NodeId) -> &DomData {
        self.node(node)
    }
}

impl IndexMut<NodeId> for Document {
    fn index_mut(&mut self, node: NodeId) -> &mut DomData {
        self.node_mut(node)
    }
}

impl MatchingContext for Document {
    type ElementRef = NodeId;

    fn parent_element(&self, element: NodeId) -> Option<NodeId> {
        match self.parents[element] {
            Some(p) if self[p].kind() == NodeKind::Element => Some(p),
            _ => None,
        }
    }

    fn local_name(&self, el: Self::ElementRef) -> &str {
        &self[el].el().local_name
    }

    fn attribute(&self, el: Self::ElementRef, attr: &str) -> Option<&str> {
        self[el]
            .el()
            .attributes
            .iter()
            .find(|(a, _)| attr == &**a)
            .map(|(_, v)| &**v)
    }
}

pub enum DomData {
    Element(ElementData),
    Text(String),
    Document,
}

impl DomData {
    pub fn kind(&self) -> NodeKind {
        match self {
            DomData::Element(_) => NodeKind::Element,
            DomData::Text(_) => NodeKind::Text,
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
            DomData::Text(s) => s,
            _ => panic!("not a text"),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        match self {
            DomData::Text(s) => *s = text.to_owned(),
            _ => panic!("not a text"),
        }
    }
}

pub struct ElementData {
    local_name: Atom,
    attributes: Vec<(Atom, Atom)>,
    style: Style,
}

impl ElementData {
    pub fn local_name(&self) -> Atom {
        self.local_name
    }

    pub fn attribute_names(&self) -> impl Iterator<Item = Atom> + '_ {
        self.attributes.iter().map(|(k, _)| *k)
    }

    pub fn attribute(&self, attr: impl Into<Atom>) -> Option<&str> {
        let attr = attr.into();
        self.attributes.iter().find(|(a, _)| attr == *a).map(|(_, v)| &**v)
    }

    pub fn set_attribute(&mut self, attr: impl Into<Atom>, value: &str) {
        let attr = attr.into();

        if let Some(a) = self.attributes.iter_mut().find(|(a, _)| attr == *a) {
            a.1 = value.into();
        } else {
            self.attributes.push((attr.into(), value.into()));
        }
    }

    pub fn remove_attribute(&mut self, attr: impl Into<Atom>) {
        let attr = attr.into();
        self.attributes.retain(|(a, _)| attr != *a);
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn set_style(&mut self, style: impl Into<Style>) {
        self.style = style.into()
    }
}

/*
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
        assert!(doc.element_matches(div, "div#panel"));

        doc.append_child(doc.root(), div);
        assert_eq!(doc.query_selector(doc.root(), "div#panel"), Some(div));
    }

    #[test]
    fn inline_style() {
        let mut doc = Document::new();
        let div = doc.create_element("div");

        doc[div].el_mut().set_style("display: block");
        assert_eq!(doc[div].el().style().to_string(), "display:block;");

        // doc[div].el_mut().style_mut().set_property("width", "100px");
        // assert_eq!(
        //     doc[div].el().attribute("style").as_deref(),
        //     Some("display:block;width:100px;")
        // );

        doc[div].el_mut().set_style("");
        assert_eq!(doc[div].el().style().to_string(), "");
    }
}

*/
