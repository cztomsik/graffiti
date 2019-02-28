use std::f32;
#[cfg(test)]
use ordered_float::OrderedFloat;
use yoga::{FlexStyle, Node as YogaNode, StyleUnit, Direction};

use super::{ComputedLayout, Dimension, Flex, LayoutService, Rect, Size};
use crate::Id;

pub struct YogaLayoutService {
    yoga_nodes: Vec<YogaNode>,
    computed_layouts: Vec<ComputedLayout>,
}

impl YogaLayoutService {
    pub fn new() -> Self {
        YogaLayoutService {
            yoga_nodes: vec![],
            computed_layouts: vec![]
        }
    }
}

impl LayoutService for YogaLayoutService {
    fn append_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        let index = parent.get_child_count();
        parent.insert_child(child, index);
    }

    fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.remove_child(child);
    }

    fn insert_at(&mut self, parent: Id, child: Id, index: u32) {
        let (parent, child) = get_two_muts(&mut self.yoga_nodes, parent, child);

        parent.insert_child(child, index);
    }

    fn set_size(&mut self, id: Id, size: Size) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::Width(size.0.into()),
            FlexStyle::Height(size.1.into()),
        ])
    }

    fn set_flex(&mut self, id: Id, flex: Flex) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::FlexGrow(flex.grow.into()),
            FlexStyle::FlexShrink(flex.shrink.into()),
            FlexStyle::FlexBasis(flex.basis.into()),
        ]);
    }

    fn set_padding(&mut self, id: Id, padding: Rect) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::PaddingTop(padding.0.into()),
            FlexStyle::PaddingRight(padding.1.into()),
            FlexStyle::PaddingBottom(padding.2.into()),
            FlexStyle::PaddingLeft(padding.3.into()),
        ]);
    }

    fn set_margin(&mut self, id: Id, margin: Rect) {
        self.yoga_nodes[id].apply_styles(&vec![
            FlexStyle::MarginTop(margin.0.into()),
            FlexStyle::MarginRight(margin.1.into()),
            FlexStyle::MarginBottom(margin.2.into()),
            FlexStyle::MarginLeft(margin.3.into()),
        ]);
    }

    fn compute_layout(&mut self, id: Id) {
        self.yoga_nodes[id].calculate_layout(f32::MAX, f32::MAX, Direction::LTR);

        self.computed_layouts = self
            .yoga_nodes
            .iter()
            .map(|n| {
                (
                    n.get_layout_left(),
                    n.get_layout_top(),
                    n.get_layout_width(),
                    n.get_layout_height(),
                )
            })
            .collect()
    }

    fn get_computed_layout(&self, id: Id) -> ComputedLayout {
        self.computed_layouts[id]
    }
}

impl Into<StyleUnit> for Dimension {
    fn into(self) -> StyleUnit {
        match self {
            Dimension::Auto => StyleUnit::Auto,
            Dimension::Percent(f) => StyleUnit::Percent(f.into()),
            Dimension::Point(f) => StyleUnit::Point(f.into()),
        }
    }
}

// mutably borrow two items from one vec
fn get_two_muts<'a, T>(vec: &mut Vec<T>, first: usize, second: usize) -> (&'a mut T, &'a mut T) {
    unsafe {
        let ptr = vec.as_mut_ptr();
        let len = vec.len();

        assert!(first < len);
        assert!(second < len);
        assert_ne!(first, second);

        (&mut *ptr.add(first), &mut *ptr.add(second))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc(count: usize) -> YogaLayoutService {
        let yoga_nodes = (0..count).map(|_n| YogaNode::new()).collect();

        YogaLayoutService {
            yoga_nodes,
            computed_layouts: vec![],
        }
    }

    #[test]
    fn test_append_child() {
        let mut svc = test_svc(2);
        let parent = 0;
        let child = 1;

        assert_eq!(svc.yoga_nodes[parent].get_child_count(), 0);
        svc.append_child(parent, child);
        assert_eq!(svc.yoga_nodes[parent].get_child_count(), 1);
    }

    #[test]
    fn test_layout_set_size() {
        let mut svc = test_svc(1);
        let id = 0;

        svc.set_size(id, Size(Dimension::Point(100.), Dimension::Percent(100.)));

        assert_eq!(
            svc.yoga_nodes[id].get_style_width(),
            StyleUnit::Point(OrderedFloat::from(100.))
        );
        assert_eq!(
            svc.yoga_nodes[id].get_style_height(),
            StyleUnit::Percent(OrderedFloat::from(100.))
        );
    }
}
