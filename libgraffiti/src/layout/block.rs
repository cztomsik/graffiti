use super::{Ctx, Size};

impl Ctx {
    // pub(super) fn compute_block(&self, block: &mut LayoutBox, parent_size: Size<f32>) {
    //     if block.size.width.is_nan() {
    //         block.size.width = parent_size.width;
    //     }

    //     let mut y = block.padding.top;

    //     for child in &mut block.children {
    //         self.compute_box(child, parent_size);
    //         child.y = y;
    //         child.x = block.padding.left;

    //         y += child.size.height;
    //     }

    //     if block.size.height.is_nan() {
    //         block.size.height = block.children.iter().map(|ch| ch.size.height).sum();
    //     }

    //     //println!("{:?}", block.size);
    // }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use Display::Block;
    use Dimension::Px;

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
