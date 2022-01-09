use super::{LayoutNodeId, LayoutTree};

// LISPish macro
//
// let (tree, root) = layout_tree! (node(style_prop = val, ...)
//   (node(...) ...)
//   (text "hello")
//   ...
// )
macro_rules! layout_tree {
  ($root:tt) => ({
      let mut tree = crate::layout::LayoutTree::new();
      let root = layout_tree!(tree, $root);
      (tree, root)
  });
  // TODO: text
  ($tree:expr, ( text $text:literal )) => ($tree.create_node());
  ($tree:expr, ( $tag:ident ($($($prop:ident).+ = $val:expr),*) $($inner:tt)* )) => ({
      let node = $tree.create_node();
      let mut style = crate::layout::LayoutStyle::default();
      $(style.$($prop).+ = $val;)*
      $tree.set_style(node, style);

      for child in [ $(layout_tree!($tree, $inner)),* ] {
          $tree.append_child(node, child);
      }

      node
  });
}

impl LayoutTree {
    pub(super) fn debug(&self, node: LayoutNodeId) -> String {
        struct DebugRef<'a>(&'a LayoutTree, LayoutNodeId);
        impl std::fmt::Debug for DebugRef<'_> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                let style = &self.0.style(self.1);
                let size = self.0.layout_result(self.1).size;

                write!(fmt, "{:?}({:?}, {:?}) ", style.display, size.width, size.height)?;
                fmt.debug_list()
                    .entries(self.0.children(self.1).map(|id| DebugRef(self.0, id)))
                    .finish()
            }
        }

        format!("{:?}", DebugRef(self, node))
    }
}
