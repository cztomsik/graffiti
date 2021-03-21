pub trait Tree {
  type NodeRef: Copy;
  type NodeValue;
  type NodeChildren: IntoIterator<Item = Self::NodeRef>;

  fn root(&self) -> Self::NodeRef;
  fn value(&self, node: Self::NodeRef) -> Self::NodeValue;
  fn children(&self, node: Self::NodeRef) -> Self::NodeChildren;
}

pub(crate) struct TreeAdapter<
  NodeRef,
  NodeValue,
  NodeChildren: IntoIterator<Item = NodeRef>,
  ValueFn: Fn(NodeRef) -> NodeValue,
  ChildrenFn: Fn(NodeRef) -> NodeChildren,
> {
  pub(crate) root: NodeRef,
  pub(crate) value_fn: ValueFn,
  pub(crate) children_fn: ChildrenFn,
}

// works both for "id trees" and &NodeStruct
// should also work for virtual/computed trees as well
impl<
      NodeRef: Copy,
      NodeValue,
      NodeChildren: IntoIterator<Item = NodeRef>,
      ValueFn: Fn(NodeRef) -> NodeValue,
      ChildrenFn: Fn(NodeRef) -> NodeChildren,
  > Tree for TreeAdapter<NodeRef, NodeValue, NodeChildren, ValueFn, ChildrenFn>
{
  type NodeRef = NodeRef;
  type NodeValue = NodeValue;
  type NodeChildren = NodeChildren;

  fn root(&self) -> Self::NodeRef {
      self.root
  }

  fn value(&self, node: Self::NodeRef) -> Self::NodeValue {
      (self.value_fn)(node)
  }

  fn children(&self, node: Self::NodeRef) -> Self::NodeChildren {
      (self.children_fn)(node)
  }
}
