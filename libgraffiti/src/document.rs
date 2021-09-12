// TODO: move rest of this to dom.rs

impl Document {
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

    pub(crate) fn descendant_children(&self, element: NodeId) -> Vec<NodeId> {
        self.children(element)
            .flat_map(move |ch| std::iter::once(ch).chain(self.descendant_children(ch)))
            .collect()
    }

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
