// TODO: move rest of this to dom.rs

#[derive(Debug)]
pub enum DocumentEvent<'a> {
    Create(NodeId, NodeType),
    Insert(NodeId, NodeId, usize),
    Remove(NodeId, NodeId),
    Cdata(NodeId, &'a str),

    // TODO: call during Document::Drop, probably in document order (children first)
    Drop(NodeId, NodeType),
}

// private shorthand
type Event<'a> = DocumentEvent<'a>;

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

    pub fn matches(&self, el: NodeId, selector: &str) -> bool {
        self.with_matching_context(|ctx| ctx.match_selector(&Selector::from(selector), el).is_some())
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
