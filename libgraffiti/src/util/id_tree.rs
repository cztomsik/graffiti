use super::{Id, SlotMap};
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// A tree with stable ids, so it can be easily updated later.
/// ```
/// let mut tree = IdTree::new();
/// let node_id = tree.create_node("foo");
/// assert_eq!(tree[node_id].data(), "foo");
/// *tree[node_id].data_mut() = "bar";
/// assert_eq!(tree[node_id].data(), "bar");
/// ```
///
/// Inserts/removals are alloc-free but traversal might get a bit slower over
/// the time and in that case it might be a good idea to rebuild the tree.
/// Node can only be attached in one parent, append/insert will panic otherwise.
/// ```
/// let mut tree = IdTree::new();
/// let parent = tree.create_node("parent");
/// let child = tree.create_node("child");
/// tree.append_child(parent, child);
/// assert_eq!(tree[parent].first_child(), Some(child));
/// ```
///
/// Freeing is explicit and can eventually lead to panic if the node is still
/// attached somewhere. Also, ids are not generational so you have the
/// ABA problem.
/// ```
/// let mut tree = IdTree::new();
/// let foo = tree.create_node("foo");
/// tree.drop_node(foo);
/// let bar = tree.create_node("bar");
///
/// // same id, different node
/// assert_eq!(foo, bar);
/// ```
pub struct IdTree<T> {
    nodes: SlotMap<Id<Node<T>>, Node<T>>,
}

pub struct Node<T> {
    parent_node: Option<Id<Node<T>>>,
    first_child: Option<Id<Node<T>>>,
    next_sibling: Option<Id<Node<T>>>,
    previous_sibling: Option<Id<Node<T>>>,
    last_child: Option<Id<Node<T>>>,
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            parent_node: None,
            first_child: None,
            next_sibling: None,
            previous_sibling: None,
            last_child: None,
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent_node(&self) -> Option<Id<Node<T>>> {
        self.parent_node
    }

    pub fn first_child(&self) -> Option<Id<Node<T>>> {
        self.first_child
    }

    pub fn last_child(&self) -> Option<Id<Node<T>>> {
        self.last_child
    }

    pub fn previous_sibling(&self) -> Option<Id<Node<T>>> {
        self.previous_sibling
    }

    pub fn next_sibling(&self) -> Option<Id<Node<T>>> {
        self.next_sibling
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data()
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data_mut()
    }
}

impl<T> IdTree<T> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::default(),
        }
    }

    pub fn create_node(&mut self, data: T) -> Id<Node<T>> {
        self.nodes.insert(Node::new(data))
    }

    pub fn drop_node(&mut self, node: Id<Node<T>>) {
        self.nodes.remove(node);
    }

    pub fn children(&self, parent: Id<Node<T>>) -> Children<T> {
        Children {
            tree: self,
            next: self[parent].first_child(),
        }
    }

    pub fn traverse(&self, node: Id<Node<T>>) -> Traverse<T> {
        Traverse {
            tree: self,
            next: Some(Edge::Start(node)),
        }
    }

    pub fn append_child(&mut self, parent: Id<Node<T>>, child: Id<Node<T>>) {
        assert_eq!(self[child].parent_node, None);

        if self[parent].first_child == None {
            self[parent].first_child = Some(child);
        }

        if let Some(last) = self[parent].last_child {
            self[last].next_sibling = Some(child);
        }

        self[child].previous_sibling = self[parent].last_child;
        self[parent].last_child = Some(child);
        self[child].parent_node = Some(parent);
    }

    pub fn insert_before(&mut self, parent: Id<Node<T>>, child: Id<Node<T>>, before: Id<Node<T>>) {
        assert_eq!(self[child].parent_node, None);
        assert_eq!(self[before].parent_node, Some(parent));

        if self[before].previous_sibling == None {
            self[parent].first_child = Some(child);
        }

        if let Some(prev) = self[before].previous_sibling {
            self[prev].next_sibling = Some(child);
        }

        self[child].previous_sibling = self[before].previous_sibling;
        self[child].next_sibling = Some(before);
        self[child].parent_node = Some(parent);

        self[before].previous_sibling = Some(child);
    }

    pub fn remove_child(&mut self, parent: Id<Node<T>>, child: Id<Node<T>>) {
        assert_eq!(self[child].parent_node, Some(parent));

        if self[child].previous_sibling == None {
            self[parent].first_child = self[child].next_sibling;
        }

        if self[child].next_sibling == None {
            self[parent].last_child = self[child].previous_sibling;
        }

        if let Some(prev) = self[child].previous_sibling {
            self[prev].next_sibling = self[child].next_sibling;
        }

        if let Some(next) = self[child].next_sibling {
            self[next].previous_sibling = self[child].previous_sibling;
        }

        self[child].parent_node = None;
        self[child].next_sibling = None;
        self[child].previous_sibling = None;
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id<Node<T>>, &T)> + '_ {
        self.nodes.iter().map(|(id, node)| (id, &node.data))
    }
}

impl<T> Default for IdTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Id<Node<T>>> for IdTree<T> {
    type Output = Node<T>;

    fn index(&self, node: Id<Node<T>>) -> &Node<T> {
        &self.nodes[node]
    }
}

impl<T> IndexMut<Id<Node<T>>> for IdTree<T> {
    fn index_mut(&mut self, node: Id<Node<T>>) -> &mut Node<T> {
        &mut self.nodes[node]
    }
}

#[derive(Debug)]
pub enum Edge<T> {
    Start(T),
    End(T),
}

pub struct Traverse<'a, T> {
    tree: &'a IdTree<T>,
    next: Option<Edge<Id<Node<T>>>>,
}

impl<T> Iterator for Traverse<'_, T> {
    type Item = Edge<Id<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(next) => {
                self.next = match next {
                    Edge::Start(node) => match self.tree[node].first_child {
                        Some(first_child) => Some(Edge::Start(first_child)),
                        None => Some(Edge::End(node)),
                    },
                    Edge::End(node) => match self.tree[node].next_sibling {
                        Some(next_sibling) => Some(Edge::Start(next_sibling)),
                        None => self.tree[node].parent_node.map(Edge::End),
                    },
                };
                Some(next)
            }
            None => None,
        }
    }
}

pub struct Children<'a, T> {
    tree: &'a IdTree<T>,
    next: Option<Id<Node<T>>>,
}

impl<T> Iterator for Children<'_, T> {
    type Item = Id<Node<T>>;

    fn next(&mut self) -> Option<Id<Node<T>>> {
        match self.next.take() {
            Some(next) => {
                self.next = self.tree.nodes[next].next_sibling;
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
        assert_eq!(tree[root].parent_node(), None);
        assert_eq!(tree[root].first_child(), None);
        assert_eq!(tree[root].last_child(), None);
        assert_eq!(tree[root].next_sibling(), None);
        assert_eq!(tree[root].previous_sibling(), None);

        let ch1 = tree.create_node("ch1");
        let ch2 = tree.create_node("ch2");
        let ch3 = tree.create_node("ch3");
        let ch4 = tree.create_node("ch4");

        tree.append_child(root, ch1);
        assert_eq!(tree[root].first_child(), Some(ch1));
        assert_eq!(tree[root].last_child(), Some(ch1));
        assert_eq!(tree[ch1].parent_node(), Some(root));
        assert_eq!(tree[ch1].next_sibling(), None);
        assert_eq!(tree[ch1].previous_sibling(), None);

        tree.append_child(root, ch2);
        assert_eq!(tree[root].first_child(), Some(ch1));
        assert_eq!(tree[root].last_child(), Some(ch2));
        assert_eq!(tree[ch1].next_sibling(), Some(ch2));
        assert_eq!(tree[ch2].previous_sibling(), Some(ch1));
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
