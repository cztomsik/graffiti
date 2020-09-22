use crate::style::Style;
use crate::util::{SlotMap, Versioned};

pub struct Document {
    nodes: SlotMap<NodeId, Versioned<Node>>,
}

impl Document {
    pub const ROOT: NodeId = 0;

    pub fn new() -> Self {
        let mut doc = Self { nodes: SlotMap::new() };

        assert_eq!(doc.create_element(0), Self::ROOT);
        doc.remove_tag(Self::ROOT, 0);

        doc
    }

    pub fn create_element(&mut self, local_name_tag: Tag) -> NodeId {
        self.create_node(NodeData::Element(ElementData {
            tags: vec![local_name_tag],
            style: Style::new(),
            child_nodes: Vec::new(),
        }))
    }

    pub fn tags(&self, element: NodeId) -> &[Tag] {
        &self.nodes[element].el().tags
    }

    pub fn add_tag(&mut self, element: NodeId, tag: Tag) {
        self.nodes[element] = self.nodes[element].with(|el| el.el_mut().tags.push(tag));
    }

    pub fn remove_tag(&mut self, element: NodeId, tag: Tag) {
        self.nodes[element] = self.nodes[element].with(|el| el.el_mut().tags.retain(|t| *t != tag));
    }

    pub fn child_nodes(&self, element: NodeId) -> &[NodeId] {
        &self.nodes[element].el().child_nodes
    }

    pub fn insert_child(&mut self, element: NodeId, child: NodeId, index: usize) {
        self.nodes[element] = self.nodes[element].with(|el| el.el_mut().child_nodes.insert(index, child));
        self.nodes[child] = self.nodes[child].with(|ch| ch.parent = Some(element));
    }

    pub fn remove_child(&mut self, element: NodeId, child: NodeId) {
        self.nodes[element] = self.nodes[element].with(|el| el.el_mut().child_nodes.retain(|ch| *ch != child));
        self.nodes[child] = self.nodes[child].with(|ch| ch.parent = None);
    }

    pub fn set_style(&mut self, element: NodeId, prop: &str, value: &str) {
        self.nodes[element] = self.nodes[element].with(|el| el.el_mut().style.set_prop_value(prop, value).unwrap_or(()));
    }

    pub fn style(&self, node: NodeId) -> &Style {
        &self.nodes[node].el().style
    }

    pub fn create_text_node(&mut self, text: &str) -> NodeId {
        self.create_node(NodeData::Text(text.to_owned()))
    }

    pub fn text(&self, text_node: NodeId) -> &str {
        self.nodes[text_node].text()
    }

    pub fn set_text(&mut self, text_node: NodeId, text: &str) {
        self.nodes[text_node] = self.nodes[text_node].with(|tn| tn.data = NodeData::Text(text.to_owned()))
    }

    // shared for both node types
    pub fn version(&self, node: NodeId) -> u32 {
        self.nodes[node].version()
    }

    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].parent
    }
    // TODO: image, canvas, video/texture/dyn external/paintable?
    pub fn visit_node(&self, node: NodeId, visitor: &mut impl NodeVisitor) {
        let n = &self.nodes[node];

        match &n.data {
            NodeData::Element(ElementData { child_nodes, .. }) => visitor.visit_element(node, n.version(), child_nodes),
            NodeData::Text(text) => visitor.visit_text(node, n.version(), text),
        }
    }

    pub fn free_node(&mut self, node: NodeId) {
        silly!("free node {:?}", node);

        self.nodes.remove(node);
    }

    // helpers

    fn create_node(&mut self, data: NodeData) -> NodeId {
        self.nodes.insert(Versioned::new(Node { parent: None, data }))
    }
}

pub type Tag = u32;

pub type NodeId = u32;

// trait, so we can freely add new variants
pub trait NodeVisitor {
    fn visit_element(&mut self, element: NodeId, version: u32, child_nodes: &[NodeId]) {}
    fn visit_text(&mut self, text_node: NodeId, version: u32, text: &str) {}
}

// private from here

#[derive(Clone)]
struct Node {
    parent: Option<NodeId>,
    data: NodeData,
}

#[derive(Clone)]
enum NodeData {
    Element(ElementData),
    Text(String),
}

#[derive(Clone)]
struct ElementData {
    tags: Vec<Tag>,
    style: Style,
    child_nodes: Vec<NodeId>,
}

// TODO: macro?
impl Node {
    fn el(&self) -> &ElementData {
        if let NodeData::Element(data) = &self.data {
            data
        } else {
            panic!("not an element")
        }
    }

    fn el_mut(&mut self) -> &mut ElementData {
        if let NodeData::Element(data) = &mut self.data {
            data
        } else {
            panic!("not an element")
        }
    }

    fn text(&self) -> &str {
        if let NodeData::Text(data) = &self.data {
            data
        } else {
            panic!("not a text node")
        }
    }
}
