use super::{LayoutContext, LayoutResult, LayoutStyle, Size};
use crate::util::{Id, IdTree, Node, SlotMap};

pub type LayoutNodeId = Id<Node<LayoutData>>;

#[derive(Default)]
pub struct LayoutTree {
    tree: IdTree<LayoutData>,
    results: SlotMap<LayoutNodeId, LayoutResult>,
}

impl LayoutTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_node(&mut self) -> LayoutNodeId {
        let id = self.tree.create_node(LayoutData::default());
        self.results.put(id, LayoutResult::default());

        id
    }

    pub fn drop_node(&mut self, node: LayoutNodeId) {
        self.tree.drop_node(node);
        self.results.remove(node);
    }

    pub fn style(&self, node: LayoutNodeId) -> &LayoutStyle {
        &self.tree[node].style
    }

    pub fn set_style(&mut self, node: LayoutNodeId, style: LayoutStyle) {
        self.tree[node].style = style;
    }

    pub fn children(&self, node: LayoutNodeId) -> impl Iterator<Item = LayoutNodeId> + '_ {
        self.tree.children(node)
    }

    pub fn append_child(&mut self, parent: LayoutNodeId, child: LayoutNodeId) {
        self.tree.append_child(parent, child);
    }

    pub fn insert_before(&mut self, parent: LayoutNodeId, child: LayoutNodeId, before: LayoutNodeId) {
        self.tree.insert_before(parent, child, before);
    }

    pub fn remove_child(&mut self, parent: LayoutNodeId, child: LayoutNodeId) {
        self.tree.remove_child(parent, child);
    }

    pub fn measure(&mut self, node: LayoutNodeId) -> Option<&Box<dyn Fn()>> {
        self.tree[node].measure.as_ref()
    }

    pub fn set_measure(&mut self, node: LayoutNodeId, measure: Option<Box<dyn Fn()>>) {
        self.tree[node].measure = measure;
        self.mark_dirty(node);
    }

    pub fn mark_dirty(&mut self, node: LayoutNodeId) {
        self.tree[node].needs_measure = true;
    }

    pub fn calculate(&mut self, node: LayoutNodeId, avail_width: f32, avail_height: f32) {
        println!("-- calculate");

        let size = Size {
            width: avail_width,
            height: avail_height,
        };

        let mut results = std::mem::take(&mut self.results);

        LayoutContext::new(self, &mut results, size).compute_node(node, size);

        self.results = results;
    }

    pub fn layout_result(&self, node: LayoutNodeId) -> &LayoutResult {
        &self.results[node]
    }
}

#[derive(Default)]
pub struct LayoutData {
    style: LayoutStyle,
    measure: Option<Box<dyn Fn()>>,
    needs_measure: bool,
}
