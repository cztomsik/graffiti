use super::{FlexDirection, LayoutContext, LayoutNodeId, LayoutResult, LayoutStyle, Size};

impl LayoutContext<'_> {
    pub fn compute_flex(
        &mut self,
        node: LayoutNodeId,
        style: &LayoutStyle,
        parent_size: Size<f32>,
    ) {
        let dir = style.flex_direction;

        // TODO: if not defined
        let available_space = self.resolve_size(style.size, parent_size);

        let total_flex_basis: f32 = self
            .tree
            .children(node)
            .map(|ch| {
                let mut res = self.resolve(self.tree.style(ch).flex_basis, parent_size.main(dir));
                if res.is_nan() {
                    // compute max-content size?
                    todo!()
                }

                res
            })
            .sum();
        let remaining_space = available_space.main(dir) - total_flex_basis;
        let total_grow: f32 = self.tree.children(node).map(|ch| self.tree.style(ch).flex_grow).sum();

        //println!("{:?}", (available_space, total_flex_basis, remaining_space, total_grow));
        for child in self.tree.children(node) {
            let child_style = self.tree.style(child);
            let child_res = &mut self.results[child];

            if child_style.flex_grow > 0. {
                child_res
                    .size
                    .set_main(dir, (child_style.flex_grow / total_grow) * remaining_space);
                child_res.size.set_cross(dir, available_space.cross(dir));
                println!("{:?}", (child_style.flex_grow, child_res.size));
            } else {
                println!("TODO: nonflexible items should be already resolved here");
            }
        }
    }
}

// flexbox extensions
impl<T: Copy> Size<T> {
    fn main(&self, dir: FlexDirection) -> T {
        match dir {
            FlexDirection::Row => self.width,
            FlexDirection::Column => self.height,
        }
    }

    fn set_main(&mut self, dir: FlexDirection, val: T) {
        match dir {
            FlexDirection::Row => self.width = val,
            FlexDirection::Column => self.height = val,
        }
    }

    fn cross(&self, dir: FlexDirection) -> T {
        match dir {
            FlexDirection::Row => self.height,
            FlexDirection::Column => self.width,
        }
    }

    fn set_cross(&mut self, dir: FlexDirection, val: T) {
        match dir {
            FlexDirection::Row => self.height = val,
            FlexDirection::Column => self.width = val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    use Dimension::Px;
    use Display::*;

    #[test]
    fn flex_row_grow() {
        let (mut tree, root) = layout_tree! {
            (node(display = Flex, size.width = Px(300.), size.height = Px(10.))
                (node(flex_grow = 1., flex_basis = Px(0.)))
                (node(flex_grow = 2., flex_basis = Px(0.)))
            )
        };

        tree.calculate(root, 0., 0.);

        assert_eq!(
            tree.debug(root),
            stringify!(
                Flex(300.0, 10.0) [
                    Inline(100.0, 10.0) [],
                    Inline(200.0, 10.0) []
                ]
            )
        );
    }
}
