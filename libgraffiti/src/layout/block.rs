use super::{LayoutContext, LayoutNodeId, LayoutResult, LayoutStyle, Size};

impl LayoutContext<'_> {
    pub fn compute_block(&mut self, node: LayoutNodeId, style: &LayoutStyle, parent_size: Size<f32>) {
        let mut y = self.results[node].padding.top;
        let mut content_height = 0.;

        let avail_inner = Size {
            width: f32::max(
                0.,
                parent_size.width - self.results[node].padding.left - self.results[node].padding.right,
            ),
            height: f32::max(
                0.,
                parent_size.height - self.results[node].padding.top - self.results[node].padding.bottom,
            ),
        };

        for child in self.tree.children(node) {
            self.compute_node(child, avail_inner);

            self.results[child].y = y;
            self.results[child].x = self.results[node].padding.left;

            y += self.results[child].size.height;
            content_height += self.results[child].size.height;
        }

        if self.results[node].size.width.is_nan() {
            self.results[node].size.width = parent_size.width;
        }

        if self.results[node].size.height.is_nan() {
            self.results[node].size.height = content_height;
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
            (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
        };

        tree.calculate(root, 0., 0.);
        assert_eq!(tree.debug(root), "Block(10.0, 10.0) []");
    }

    #[test]
    fn fixed_height() {
        let (mut tree, root) = layout_tree! {
            (node(display = Block, size.height = Px(10.)))
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
                (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
                (node(display = Block, size.height = Px(10.)))
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
            (node(display = Block, padding.top = Px(10.), padding.left = Px(10.))
                (node(display = Block, size.height = Px(10.)))
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
