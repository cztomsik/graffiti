use super::{FlexDirection, LayoutContext, LayoutStyle, Size};

impl<K: Copy> LayoutContext<'_, K> {
    pub fn compute_flex(&mut self, node: K, style: &LayoutStyle, parent_size: Size<f32>) {
        let dir = style.flex_direction;

        // TODO: if not defined
        let available_space = self.resolve_size(style.size, parent_size);

        // skoda, ze nemuzu rict node.total_flex_basis() a mit to cele nekde bokem
        let total_flex_basis: f32 = self.children[node]
            .iter()
            .map(|&ch| {
                let mut res = self.resolve(self.styles[ch].flex_basis, parent_size.main(dir));
                if res.is_nan() {
                    // compute max-content size?
                    todo!()
                }

                res
            })
            .sum();
        let remaining_space = available_space.main(dir) - total_flex_basis;
        let total_grow: f32 = self.children[node].iter().map(|&ch| self.styles[ch].flex_grow).sum();

        //println!("{:?}", (available_space, total_flex_basis, remaining_space, total_grow));
        for &child in &self.children[node] {
            let child_style = &self.styles[child];
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
        let calculate = layout_tree! {
            (node(display = Flex, size.width = Px(300.), size.height = Px(10.))
                (node(flex_grow = 1., flex_basis = Px(0.)))
                (node(flex_grow = 2., flex_basis = Px(0.)))
            )
        };

        let results = calculate(Size::new(0., 0.));

        assert_eq!(results[0].size, Size::new(300., 10.));
        assert_eq!(results[1].size, Size::new(100., 10.));
        assert_eq!(results[2].size, Size::new(200., 10.));
    }
}
