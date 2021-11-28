use crate::util::SlotMap;
use super::{Ctx, NodeId, Size, LayoutResult};

impl Ctx<'_> {
    pub(super) fn compute_block(
        &self,
        results: &mut SlotMap<NodeId, LayoutResult>,
        node: NodeId,
        parent_size: Size<f32>,
    ) {
        let mut y = results[node].padding.top;

        let avail_inner = Size {
            width: f32::max(0., parent_size.width - results[node].padding.left - results[node].padding.right),
            height: f32::max(0., parent_size.height - results[node].padding.top - results[node].padding.bottom),
        };

        for &child in &self.nodes[node].children {
            self.compute_node(results, child, avail_inner);

            results[child].y = y;
            results[child].x = results[node].padding.left;

            y += results[child].size.height;
        }

        if results[node].size.width.is_nan() {
            results[node].size.width = parent_size.width;
        }

        if results[node].size.height.is_nan() {
            results[node].size.height = self.nodes[node].children.iter().map(|&ch| results[ch].size.height).sum();
        }
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

    #[test]
    fn padding() {
        let (mut tree, root) = layout_tree! {
            (node(display = Block, padding_top = Px(10.), padding_left = Px(10.))
                (node(display = Block, height = Px(10.)))
            )
        };

        tree.calculate(root, 100., 0.);
        assert_eq!(
            tree.debug(root),
            stringify!(
                Block(100.0, 20.0) [
                    Block(90.0, 10.0) [],
                ]
            )
        );
    }

    #[test]
    #[ignore]
    fn margin() {
        todo!()
    }
}
