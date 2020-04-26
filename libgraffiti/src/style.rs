// - have concept of sheets, selectors, rules, ...
//   - maybe start with one global sheet?
// - cascading
// - update() -> Vec<(el NodeId, Vec<StyleProp>)>
// - *
// - tagName
// - className
// - direct descendant
// - descendant
// - :hover
// - no :before/after, :nth, nor siblings
//
// later:
// - accept Dimension in non-layout props
//   (dynamic properties)

use crate::box_layout::{Align, Dimension, Display, FlexDirection, FlexWrap, Overflow};
use crate::render::value_types::Color;
use crate::text::TextAlign;

// supported style props
#[derive(Debug, Clone)]
pub enum StyleProp {
    // TODO: border colors
    // TODO: FontStyle, FontVariant
    AlignContent(Align),
    AlignItems(Align),
    AlignSelf(Align),
    BackgroundColor(Color),
    BorderBottomLeftRadius(f32),
    BorderBottomRightRadius(f32),
    //BorderBottomStyle(BorderStyle),
    BorderBottomWidth(f32),
    //BorderLeftStyle(BorderStyle),
    BorderLeftWidth(f32),
    //BorderRightStyle(BorderStyle),
    BorderRightWidth(f32),
    BorderTopLeftRadius(f32),
    BorderTopRightRadius(f32),
    //BorderTopStyle(BorderStyle),
    BorderTopWidth(f32),
    Bottom(Dimension),
    Color(Color),
    Display(Display),
    FlexBasis(Dimension),
    FlexDirection(FlexDirection),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexWrap(FlexWrap),
    FontFamily(String),
    FontSize(f32),
    Height(Dimension),
    JustifyContent(Align),
    Left(Dimension),
    LineHeight(f32),
    MarginBottom(Dimension),
    MarginLeft(Dimension),
    MarginRight(Dimension),
    MarginTop(Dimension),
    MaxHeight(Dimension),
    MaxWidth(Dimension),
    MinHeight(Dimension),
    MinWidth(Dimension),
    Overflow(Overflow),
    PaddingBottom(Dimension),
    PaddingLeft(Dimension),
    PaddingRight(Dimension),
    PaddingTop(Dimension),
    Right(Dimension),
    TextAlign(TextAlign),
    Top(Dimension),
    Width(Dimension),
}

pub struct StyleEngine {}

impl StyleEngine {}
