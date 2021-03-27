// generic, id-based, intrusive linked tree

use crate::util::SlotMap;

pub type NodeId = u32;

pub struct IdTree<T> {
    nodes: SlotMap<NodeId, Node<T>>,
}

impl<T> IdTree<T> {
    pub fn new() -> Self {
        Self { nodes: SlotMap::new() }
    }

    pub fn create_node(&mut self, data: T) -> NodeId {
        self.nodes.insert(Node {
            data,
            parent: None,
            first_child: None,
            next_sibling: None,
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

    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].parent
    }

    pub fn first_child(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].first_child
    }

    pub fn prev_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.children(self.parent(node)?)
            .find(|n| self.nodes[*n].next_sibling == Some(node))
    }

    pub fn next_sibling(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].next_sibling
    }

    pub fn insert_child(&mut self, parent: NodeId, child: NodeId, index: usize) {
        debug_assert_eq!(self.nodes[child].parent, None);

        if index == 0 {
            self.nodes[child].next_sibling = self.first_child(parent);
            self.nodes[parent].first_child = Some(child);
        } else {
            let prev = (1..index)
                .fold(self.first_child(parent), |n, _| self.next_sibling(n?))
                .expect("out of bounds");

            self.nodes[child].next_sibling = self.next_sibling(prev);
            self.nodes[prev].next_sibling = Some(child);
        }

        self.nodes[child].parent = Some(parent);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        debug_assert_eq!(self.nodes[child].parent, Some(parent));

        if let Some(prev) = self.prev_sibling(child) {
            self.nodes[prev].next_sibling = self.next_sibling(child);
        } else {
            self.nodes[parent].first_child = self.next_sibling(child);
        }

        self.nodes[child].next_sibling = None;
        self.nodes[child].parent = None;
    }

    pub fn children(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        Children {
            tree: self,
            next: self.nodes[node].first_child,
        }
    }
}

struct Node<T> {
    parent: Option<NodeId>,
    first_child: Option<NodeId>,
    next_sibling: Option<NodeId>,
    data: T,
}

pub struct Children<'a, T> {
    tree: &'a IdTree<T>,
    next: Option<NodeId>,
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(next) => {
                self.next = self.tree.nodes[next].next_sibling;
                Some(next)
            }
            _ => None,
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
        assert_eq!(tree.parent(root), None);
        assert_eq!(tree.first_child(root), None);
        assert_eq!(tree.next_sibling(root), None);
        assert_eq!(tree.prev_sibling(root), None);

        let ch1 = tree.create_node("ch1");
        let ch2 = tree.create_node("ch2");
        let ch3 = tree.create_node("ch3");

        tree.insert_child(root, ch1, 0);
        assert_eq!(tree.first_child(root), Some(ch1));
        assert_eq!(tree.parent(ch1), Some(root));
        assert_eq!(tree.next_sibling(ch1), None);
        assert_eq!(tree.prev_sibling(ch1), None);

        tree.insert_child(root, ch2, 1);
        assert_eq!(tree.first_child(root), Some(ch1));
        assert_eq!(tree.next_sibling(ch1), Some(ch2));
        assert_eq!(tree.prev_sibling(ch2), Some(ch1));

        assert_eq!(tree.children(root).collect::<Vec<_>>(), vec![ch1, ch2]);

        tree.insert_child(root, ch3, 0);

        assert_eq!(tree.children(root).collect::<Vec<_>>(), vec![ch3, ch1, ch2]);

        tree.remove_child(root, ch1);
        tree.remove_child(root, ch2);

        assert_eq!(tree.children(root).collect::<Vec<_>>(), vec![ch3]);

        tree.insert_child(root, ch2, 0);
        tree.insert_child(root, ch1, 0);

        assert_eq!(tree.children(root).collect::<Vec<_>>(), vec![ch1, ch2, ch3]);
    }
}
