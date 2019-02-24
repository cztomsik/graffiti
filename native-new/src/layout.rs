use serde::Deserialize;

use crate::generated::{Dimension, Flex, Rect, Size};

use yoga::{FlexStyle, Node as YogaNode, StyleUnit};

type Id = usize;

#[derive(Deserialize)]
#[serde(tag = "tag", content = "value")]
pub enum LayoutMsg {
    SetSize((Id, Size)),
    SetFlex((Id, Flex)),
    SetPadding((Id, Rect)),
    SetMargin((Id, Rect)),
}

pub fn update_layout(layout: &mut Vec<YogaNode>, msgs: Vec<LayoutMsg>) {
    for m in msgs {
        match m {
            LayoutMsg::SetSize((id, size)) => set_size(&mut layout[id], size),
            LayoutMsg::SetFlex((id, flex)) => set_flex(&mut layout[id], flex),
            LayoutMsg::SetPadding((id, padding)) => set_padding(&mut layout[id], padding),
            LayoutMsg::SetMargin((id, margin)) => set_margin(&mut layout[id], margin),
        }
    }
}

fn set_size(yoga_node: &mut YogaNode, size: Size) {
    yoga_node.apply_styles(&vec![
        FlexStyle::Width(size.0.into()),
        FlexStyle::Height(size.1.into()),
    ]);
}

fn set_flex(yoga_node: &mut YogaNode, flex: Flex) {
    yoga_node.apply_styles(&vec![
        FlexStyle::FlexGrow(flex.grow.into()),
        FlexStyle::FlexShrink(flex.shrink.into()),
        FlexStyle::FlexBasis(flex.basis.into()),
    ]);
}

fn set_padding(yoga_node: &mut YogaNode, padding: Rect) {
    yoga_node.apply_styles(&vec![
        FlexStyle::PaddingTop(padding.0.into()),
        FlexStyle::PaddingRight(padding.1.into()),
        FlexStyle::PaddingBottom(padding.2.into()),
        FlexStyle::PaddingLeft(padding.3.into()),
    ]);
}

fn set_margin(yoga_node: &mut YogaNode, margin: Rect) {
    yoga_node.apply_styles(&vec![
        FlexStyle::MarginTop(margin.0.into()),
        FlexStyle::MarginRight(margin.1.into()),
        FlexStyle::MarginBottom(margin.2.into()),
        FlexStyle::MarginLeft(margin.3.into()),
    ]);
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
