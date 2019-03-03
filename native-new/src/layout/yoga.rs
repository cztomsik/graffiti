#[cfg(test)]
use ordered_float::OrderedFloat;
use std::f32;
use yoga::types::{Align, FlexDirection as YogaFlexDirection, Wrap};
use yoga::{Direction, FlexStyle, Node as YogaNode, StyleUnit};

use super::{
    ComputedLayout, Dimension, Flex, FlexAlign, FlexDirection, FlexWrap, Flow, JustifyContent,
    LayoutService, Rect, Size,
};
use crate::storage::DenseStorage;
use crate::surface::SurfaceData;
use crate::Id;
use yoga::types::Justify;

pub struct YogaLayoutService {
    yoga_nodes: DenseStorage<Id, YogaNode>,
}

impl YogaLayoutService {
    pub fn new() -> Self {
        YogaLayoutService {
            yoga_nodes: DenseStorage::new(),
        }
    }

    pub fn alloc(&mut self) {
        self.yoga_nodes.push(YogaNode::new())
    }

    pub fn append_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = self.yoga_nodes.get_two_muts(parent, child);

        let index = parent.get_child_count();
        parent.insert_child(child, index);
    }

    pub fn remove_child(&mut self, parent: Id, child: Id) {
        let (parent, child) = self.yoga_nodes.get_two_muts(parent, child);

        parent.remove_child(child);
    }

    // easier with index rather than with Id
    pub fn insert_at(&mut self, parent: Id, child: Id, index: u32) {
        let (parent, child) = self.yoga_nodes.get_two_muts(parent, child);

        parent.insert_child(child, index);
    }

    pub fn set_size(&mut self, id: Id, size: Size) {
        self.yoga_nodes.get_mut(id).apply_styles(&vec![
            FlexStyle::Width(size.0.into()),
            FlexStyle::Height(size.1.into()),
        ])
    }

    pub fn set_flex(&mut self, id: Id, flex: Flex) {
        self.yoga_nodes.get_mut(id).apply_styles(&vec![
            FlexStyle::FlexGrow(flex.flex_grow.into()),
            FlexStyle::FlexShrink(flex.flex_shrink.into()),
            FlexStyle::FlexBasis(flex.flex_basis.into()),
        ]);
    }

    pub fn set_flow(&mut self, id: Id, flow: Flow) {
        self.yoga_nodes.get_mut(id).apply_styles(&vec![
            FlexStyle::FlexDirection(flow.flex_direction.into()),
            FlexStyle::FlexWrap(flow.flex_wrap.into()),
            FlexStyle::JustifyContent(flow.justify_content.into()),
            FlexStyle::AlignContent(flow.align_content.into()),
            FlexStyle::AlignItems(flow.align_items.into())
        ]);
    }

    pub fn set_padding(&mut self, id: Id, padding: Rect) {
        self.yoga_nodes.get_mut(id).apply_styles(&vec![
            FlexStyle::PaddingTop(padding.0.into()),
            FlexStyle::PaddingRight(padding.1.into()),
            FlexStyle::PaddingBottom(padding.2.into()),
            FlexStyle::PaddingLeft(padding.3.into()),
        ]);
    }

    pub fn set_margin(&mut self, id: Id, margin: Rect) {
        self.yoga_nodes.get_mut(id).apply_styles(&vec![
            FlexStyle::MarginTop(margin.0.into()),
            FlexStyle::MarginRight(margin.1.into()),
            FlexStyle::MarginBottom(margin.2.into()),
            FlexStyle::MarginLeft(margin.3.into()),
        ]);
    }
}

impl LayoutService for YogaLayoutService {
    fn get_computed_layouts(&mut self, surface: &SurfaceData) -> Vec<ComputedLayout> {
        self.yoga_nodes
            .get_mut(surface.id())
            .calculate_layout(f32::MAX, f32::MAX, Direction::LTR);

        self.yoga_nodes
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

impl Into<YogaFlexDirection> for FlexDirection {
    fn into(self) -> YogaFlexDirection {
        match self {
            FlexDirection::Column => YogaFlexDirection::Column,
            FlexDirection::ColumnReverse => YogaFlexDirection::ColumnReverse,
            FlexDirection::Row => YogaFlexDirection::Row,
            FlexDirection::RowReverse => YogaFlexDirection::RowReverse,
        }
    }
}

impl Into<Align> for FlexAlign {
    fn into(self) -> Align {
        match self {
            FlexAlign::Auto => Align::Auto,
            FlexAlign::Baseline => Align::Baseline,
            FlexAlign::Center => Align::Center,
            FlexAlign::FlexStart => Align::FlexStart,
            FlexAlign::FlexEnd => Align::FlexEnd,
            FlexAlign::SpaceAround => Align::SpaceAround,
            FlexAlign::SpaceBetween => Align::SpaceBetween,
            FlexAlign::Stretch => Align::Stretch,
        }
    }
}

impl Into<Justify> for JustifyContent {
    fn into(self) -> Justify {
        match self {
            JustifyContent::Center => Justify::Center,
            JustifyContent::FlexStart => Justify::FlexStart,
            JustifyContent::FlexEnd => Justify::FlexEnd,
            JustifyContent::SpaceAround => Justify::SpaceAround,
            JustifyContent::SpaceBetween => Justify::SpaceBetween,
            JustifyContent::SpaceEvenly => Justify::SpaceEvenly,
        }
    }
}

impl Into<Wrap> for FlexWrap {
    fn into(self) -> Wrap {
        match self {
            FlexWrap::Wrap => Wrap::Wrap,
            FlexWrap::WrapReverse => Wrap::WrapReverse,
            FlexWrap::NoWrap => Wrap::NoWrap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc(count: usize) -> YogaLayoutService {
        let mut svc = YogaLayoutService::new();

        for _n in 0..count {
            svc.alloc();
        }

        svc
    }

    #[test]
    fn test_append_child() {
        let mut svc = test_svc(2);
        let parent = 0;
        let child = 1;

        assert_eq!(svc.yoga_nodes.get(parent).get_child_count(), 0);
        svc.append_child(parent, child);
        assert_eq!(svc.yoga_nodes.get(parent).get_child_count(), 1);
    }

    #[test]
    fn test_layout_set_size() {
        let mut svc = test_svc(1);
        let id = 0;

        svc.set_size(id, Size(Dimension::Point(100.), Dimension::Percent(100.)));

        assert_eq!(
            svc.yoga_nodes.get(id).get_style_width(),
            StyleUnit::Point(OrderedFloat::from(100.))
        );
        assert_eq!(
            svc.yoga_nodes.get(id).get_style_height(),
            StyleUnit::Percent(OrderedFloat::from(100.))
        );
    }
}
