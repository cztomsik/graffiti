// TODO: this is just a sketch
//       but it should be something like render tree, it should
//       listen to dom changes & maintain layout tree but do it in a
//       way which respects CSS
//       (display change means replace in layout tree, node is not inserted if display: none, etc.)
//       it should also keep attributed texts
//       and ignore any blocks inside display: inline
//       it should also wrap lone text node chunks into anonymous text block nodes
//       and it should mark those dirty if text fragment changes

use crete::{DocumentRef};
use crate::util::BitSet;

struct Updater {
  document: &DocumentRef,
  prev: Vec<PrevState>
}

struct PrevState {
  dom_node: Option<NodeRef>,
  display: CssDisplay,
  layout_node: Option<LayoutNode>,
  render_style: Option<RenderStyle>
}

impl Updater {
  // I think we should be fine with one bitset, we can detect everything
  // but we will need 2 passes because inserts are only possible when all parents
  // are created
  fn update(&mut self, dirty_nodes: BitSet) {
    self.prev.resize(dirty_nodes.capacity();

    for id in dirty_nodes {
      if let Some(dom_node) = self.document.find_node(id) {
        if let Some(parent_dom_node) = dom_node.parent_node() {

        } else {
          // removed

        }
      } else {
        // destroyed
      }
    }
  }
}
