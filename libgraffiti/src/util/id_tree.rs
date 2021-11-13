// id-based linked tree
// x alloc-free inserts/removals
// - TODO: tree.rebuild() -> (Self, Iterator<(OldId, NewId)>)?
// - TODO: consider using Cell<> so only create/drop requires &mut?
//         (but then the whole tree would be !Sync)

use super::SlotMap;
use std::num::NonZeroU32;

pub type NodeId = NonZeroU32;

pub struct IdTree<T> {
    nodes: SlotMap<NodeId, Node<T>>,
}

struct Node<T> {
    parent_node: Option<NodeId>,
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    last_child: Option<NodeId>,
    data: T,
}

impl<T> IdTree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }

    pub fn create_node(&mut self, data: T) -> NodeId {
        self.nodes.insert(Node {
            parent_node: None,
            first_child: None,
            next_sibling: None,
            previous_sibling: None,
            last_child: None,
            data,
        })
    }

    pub fn drop_node(&mut self, node: NodeId) {
        self.nodes.remove(node);
    }

    pub fn data(&self, node: NodeId) -> &T {
        &self.nodes[node].data
    }

    pub fn data_mut(&mut self, node: NodeId) -> &mut T {
        &mut self.nodes[node].data
    }

    pub fn parent_node(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].parent_node
    }

    pub fn first_child(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].first_child
    }

    pub fn last_child(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].last_child
    }

    pub fn previous_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].previous_sibling
    }

    pub fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].next_sibling
    }

    pub fn children(&self, parent: NodeId) -> Children<T> {
        Children {
            nodes: &self.nodes,
            next: self.first_child(parent),
        }
    }

    pub fn traverse(&self, parent: NodeId) -> Traverse<T> {
        Traverse {
            nodes: &self.nodes,
            next: Some(NodeEdge::Start(parent)),
        }
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        let nodes = &mut self.nodes;

        if nodes[parent].first_child == None {
            nodes[parent].first_child = Some(child);
        }

        if let Some(last) = nodes[parent].last_child {
            nodes[last].next_sibling = Some(child);
        }

        nodes[child].previous_sibling = nodes[parent].last_child;
        nodes[parent].last_child = Some(child);
        nodes[child].parent_node = Some(parent);
    }

    pub fn insert_before(&mut self, parent: NodeId, child: NodeId, before: NodeId) {
        debug_assert_eq!(self.nodes[child].parent_node, None);
        debug_assert_eq!(self.nodes[before].parent_node, Some(parent));

        let nodes = &mut self.nodes;

        if nodes[before].previous_sibling == None {
            nodes[parent].first_child = Some(child);
        }

        if let Some(prev) = nodes[before].previous_sibling {
            nodes[prev].next_sibling = Some(child);
        }

        nodes[child].previous_sibling = nodes[before].previous_sibling;
        nodes[child].next_sibling = Some(before);
        nodes[child].parent_node = Some(parent);
        
        nodes[before].previous_sibling = Some(child);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        debug_assert_eq!(self.nodes[child].parent_node, Some(parent));

        let nodes = &mut self.nodes;

        if nodes[child].previous_sibling == None {
            nodes[parent].first_child = nodes[child].next_sibling;
        }

        if nodes[child].next_sibling == None {
            nodes[parent].last_child = nodes[child].previous_sibling;
        }

        if let Some(prev) = nodes[child].previous_sibling {
            nodes[prev].next_sibling = nodes[child].next_sibling;
        }

        if let Some(next) = nodes[child].next_sibling {
            nodes[next].previous_sibling = nodes[child].previous_sibling;
        }

        nodes[child].parent_node = None;
        nodes[child].next_sibling = None;
        nodes[child].previous_sibling = None;
    }
}

impl<T> Default for IdTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum Edge<T> {
    Start(T),
    End(T),
}

pub type NodeEdge = Edge<NodeId>;

pub struct Traverse<'a, T> {
    nodes: &'a SlotMap<NodeId, Node<T>>,
    next: Option<NodeEdge>,
}

impl<T> Iterator for Traverse<'_, T> {
    type Item = NodeEdge;

    fn next(&mut self) -> Option<NodeEdge> {
        match self.next.take() {
            Some(next) => {
                self.next = match next {
                    NodeEdge::Start(node) => match self.nodes[node].first_child {
                        Some(first_child) => Some(NodeEdge::Start(first_child)),
                        None => Some(NodeEdge::End(node)),
                    },
                    NodeEdge::End(node) => match self.nodes[node].next_sibling {
                        Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                        None => self.nodes[node].parent_node.map(NodeEdge::End),
                    },
                };
                Some(next)
            }
            None => None,
        }
    }
}

pub struct Children<'a, T> {
    nodes: &'a SlotMap<NodeId, Node<T>>,
    next: Option<NodeId>,
}

impl<T> Iterator for Children<'_, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        match self.next.take() {
            Some(next) => {
                self.next = self.nodes[next].next_sibling;
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
        let mut tree = IdTree::new();
        let root = tree.create_node("root");
        assert_eq!(tree.parent_node(root), None);
        assert_eq!(tree.first_child(root), None);
        assert_eq!(tree.last_child(root), None);
        assert_eq!(tree.next_sibling(root), None);
        assert_eq!(tree.previous_sibling(root), None);

        let ch1 = tree.create_node("ch1");
        let ch2 = tree.create_node("ch2");
        let ch3 = tree.create_node("ch3");
        let ch4 = tree.create_node("ch4");

        tree.append_child(root, ch1);
        assert_eq!(tree.first_child(root), Some(ch1));
        assert_eq!(tree.last_child(root), Some(ch1));
        assert_eq!(tree.parent_node(ch1), Some(root));
        assert_eq!(tree.next_sibling(ch1), None);
        assert_eq!(tree.previous_sibling(ch1), None);

        tree.append_child(root, ch2);
        assert_eq!(tree.first_child(root), Some(ch1));
        assert_eq!(tree.last_child(root), Some(ch2));
        assert_eq!(tree.next_sibling(ch1), Some(ch2));
        assert_eq!(tree.previous_sibling(ch2), Some(ch1));
        assert_eq!(tree.children(root).collect::<Vec<_>>(), &[ch1, ch2]);

        tree.insert_before(root, ch3, ch1);
        assert_eq!(tree.children(root).collect::<Vec<_>>(), &[ch3, ch1, ch2]);

        tree.insert_before(root, ch4, ch2);
        assert_eq!(tree.children(root).collect::<Vec<_>>(), &[ch3, ch1, ch4, ch2]);

        tree.remove_child(root, ch1);
        tree.remove_child(root, ch2);
        tree.remove_child(root, ch4);
        assert_eq!(tree.children(root).collect::<Vec<_>>(), &[ch3]);

        tree.insert_before(root, ch2, ch3);
        tree.insert_before(root, ch1, ch2);
        assert_eq!(tree.children(root).collect::<Vec<_>>(), &[ch1, ch2, ch3]);
        assert_eq!(
            format!("{:?}", tree.traverse(root).collect::<Vec<_>>()),
            "[Start(1), Start(2), End(2), Start(3), End(3), Start(4), End(4), End(1)]"
        );
    }
}
