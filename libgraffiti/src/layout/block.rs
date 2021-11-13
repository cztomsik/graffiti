use crate::util::SlotMap;
use super::{Ctx, NodeId, Size, LayoutResult};

impl Ctx<'_> {
    pub(super) fn compute_block(
        &self,
        results: &mut SlotMap<NodeId, LayoutResult>,
        node: NodeId,
        parent_size: Size<f32>,
    ) {
        //let mut y = block.padding.top;

        for child in self.tree.children(node) {
            self.compute_node(results, child, parent_size);
            // child.y = y;
            // child.x = block.padding.left;

            //y += child.size.height;
        }

        if results[node].size.width.is_nan() {
            results[node].size.width = parent_size.width;
        }

        if results[node].size.height.is_nan() {
            results[node].size.height = self.tree.children(node).map(|ch| results[ch].size.height).sum();
        }

        //println!("{:?}", block.size);
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use Dimension::Px;
    use Display::Block;

    #[test]
    fn fixed_width_height() {
        let (mut tree, root) = layout_tree! {
            (node(display = Block, width = Px(10.), height = Px(10.)))
        };

        tree.calculate(root, 0., 0.);
        assert_eq!(tree.debug(root), "Block(10.0, 10.0) []");
    }

    #[test]
    fn fixed_height() {
        let (mut tree, root) = layout_tree! {
            (node(display = Block, height = Px(10.)))
        };

        tree.calculate(root, 0., 10.);
        assert_eq!(tree.debug(root), "Block(0.0, 10.0) []");

        tree.calculate(root, 10., 0.);
        assert_eq!(tree.debug(root), "Block(10.0, 10.0) []");
    }

    #[test]
    fn content_height() {
        let (mut tree, root) = layout_tree! {
            (node(display = Block)
                (node(display = Block, width = Px(10.), height = Px(10.)))
                (node(display = Block, height = Px(10.)))
            )
        };

        tree.calculate(root, 100., 0.);
        assert_eq!(
            tree.debug(root),
            stringify!(
                Block(100.0, 20.0) [
                    Block(10.0, 10.0) [],
                    Block(100.0, 10.0) []
                ]
            )
        );
    }
}
