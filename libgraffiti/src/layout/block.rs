use super::{LayoutContext, LayoutResult, LayoutStyle, LayoutTree, Rect, Size};

impl<T: LayoutTree> LayoutContext<'_, T> {
    pub(super) fn compute_block(
        &mut self,
        result: &mut LayoutResult,
        padding: &Rect<f32>,
        style: &LayoutStyle,
        children: &[T::NodeRef],
        parent_size: Size<f32>,
    ) {
        let mut y = padding.top;
        let mut content_height = 0.;

        let avail_inner = Size::new(
            f32::max(0., parent_size.width - padding.left - padding.right),
            f32::max(0., parent_size.height - padding.top - padding.bottom),
        );

        for &child in children {
            self.compute_node(child, avail_inner);

            self.results[child].pos = (padding.left, y);

            content_height += self.results[child].size.height;
            y += self.results[child].size.height;
        }

        if result.size.width.is_nan() {
            result.size.width = parent_size.width;
        }

        if result.size.height.is_nan() {
            result.size.height = content_height + padding.top + padding.bottom;
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
        let calculate = layout_tree! {
            (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
        };

        let results = calculate(Size::new(0., 0.));
        assert_eq!(results[0].size, Size::new(10., 10.));
    }

    #[test]
    fn fixed_height() {
        let calculate = layout_tree! {
            (node(display = Block, size.height = Px(10.)))
        };

        let results = calculate(Size::new(0., 10.));
        assert_eq!(results[0].size, Size::new(0., 10.));

        let results = calculate(Size::new(10., 0.));
        assert_eq!(results[0].size, Size::new(10., 10.));
    }

    #[test]
    fn content_height() {
        let calculate = layout_tree! {
            (node(display = Block)
                (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
                (node(display = Block, size.height = Px(10.)))
            )
        };

        let results = calculate(Size::new(100., 0.));
        assert_eq!(results[0].size, Size::new(100., 20.));
        assert_eq!(results[1].size, Size::new(10., 10.));
        assert_eq!(results[2].size, Size::new(100., 10.));
    }

    #[test]
    fn padding() {
        let calculate = layout_tree! {
            (node(display = Block, padding.top = Px(10.), padding.left = Px(10.))
                (node(display = Block, size.height = Px(10.)))
            )
        };

        let results = calculate(Size::new(100., 0.));
        assert_eq!(results[0].size, Size::new(100., 20.));
        assert_eq!(results[1].size, Size::new(90., 10.));
    }

    #[test]
    #[ignore]
    fn margin() {
        todo!()
    }
}
