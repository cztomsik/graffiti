use super::{LayoutContext, LayoutStyle, Size};

impl<K: Copy> LayoutContext<'_, K> {
    pub fn compute_block(&mut self, node: K, style: &LayoutStyle, parent_size: Size<f32>) {
        let mut y = self.results[node].padding.top;
        let mut content_height = 0.;

        let avail_inner = Size::new(
            f32::max(
                0.,
                parent_size.width - self.results[node].padding.left - self.results[node].padding.right,
            ),
            f32::max(
                0.,
                parent_size.height - self.results[node].padding.top - self.results[node].padding.bottom,
            ),
        );

        for &child in &self.children[node] {
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
