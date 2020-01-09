use std::collections::BTreeSet;
use crate::box_layout::{BoxLayoutTree, BoxLayoutImpl, Display, Dimension, Align, FlexDirection, FlexWrap};
use crate::commons::{SurfaceId, Color};
use crate::render::{Renderer, BoxShadow};
use crate::text_layout::{TextLayout, Text};

/// applies style changes to respective systems
/// (translates style change messages to appropriate method calls)
///
/// depending on what's been changed various things might need to be
/// recomputed, this is indicated by flags on `StyleUpdateResult`
///
/// it could be part of `Viewport` but it's easier to read/change
/// when it's separate
///
/// this is not a  style engine in any way, it doesn't do rules resolution,
/// it doesn't even keep defined rules, it just keeps some intermediate state
/// because some style props are not effective until other are set too
/// (`border-style` + `border-width` + `border-color`) and we need to store
/// these intermediate values somewhere
///
/// and lastly, some things can be "emulated" here, for example
/// we can (to some degree) support `display: block` even when the `box_layout`
/// knows only flexbox
pub struct StyleUpdater {
    // TODO: internal state (SurfaceId+StyleProp multimap)

    flex_direction_set: BTreeSet<SurfaceId>,
}

#[derive(Debug, Clone)]
pub struct StyleChange {
    pub surface: SurfaceId,
    pub prop: StyleProp
}

pub struct StyleUpdateResult {
    pub needs_layout: bool
}

#[derive(Debug, Clone)]
pub enum StyleProp {
    Display { value: Display },

    Width { value: Dimension },
    Height { value: Dimension },
    MinWidth { value: Dimension },
    MinHeight { value: Dimension },
    MaxWidth { value: Dimension },
    MaxHeight { value: Dimension },

    Top { value: Dimension },
    Right { value: Dimension },
    Bottom { value: Dimension },
    Left { value: Dimension },

    MarginTop { value: Dimension },
    MarginRight { value: Dimension },
    MarginBottom { value: Dimension },
    MarginLeft { value: Dimension },

    PaddingTop { value: Dimension },
    PaddingRight { value: Dimension },
    PaddingBottom { value: Dimension },
    PaddingLeft { value: Dimension },

    FlexGrow { value: f32 },
    FlexShrink { value: f32 },
    FlexBasis { value: Dimension },
    FlexDirection { value: FlexDirection },
    FlexWrap { value: FlexWrap },

    AlignSelf { value: Align },
    AlignContent { value: Align },
    AlignItems { value: Align },
    JustifyContent { value: Align },

    // visual
    Color { value: Color },
    BackgroundColor { value: Option<Color> },

    // TODO: border
    /*
    BorderTopWidth { value: f32 },
    BorderRightWidth { value: f32 },
    BorderBottomWidth { value: f32 },
    BorderLeftWidth { value: f32 },

    BorderTopStyle { value: BorderStyle },
    BorderRightStyle { value: BorderStyle },
    BorderBottomStyle { value: BorderStyle },
    BorderLeftStyle { value: BorderStyle },
    */

    // TODO: intermediate; clip in renderer
    BorderTopLeftRadius { value: Option<f32> },
    BorderTopRightRadius { value: Option<f32> },
    BorderBottomLeftRadius { value: Option<f32> },
    BorderBottomRightRadius { value: Option<f32> },

    // BackgroundImageUrl { value: String },

    // TODO: multiple
    BoxShadow { value: Option<BoxShadow> },

    // TODO: rethink
    Text { value: Option<Text> }
}

// TODO: rethink, this is wrong, we should not be responsible for loading
// (but decoding is probably okay) so we should get a stream or something
// or maybe just pointer to the raw (encoded) data
#[derive(Debug, Clone)]
pub struct Image {
    pub url: String,
}

impl StyleUpdater {
    pub fn new() -> Self {
        Self {
            flex_direction_set: BTreeSet::new(),
        }
    }

    pub fn update_styles(&mut self, box_layout: &mut BoxLayoutImpl, text_layout: &mut TextLayout, renderer: &mut Renderer, changes: &[StyleChange]) -> StyleUpdateResult {
        let mut result = StyleUpdateResult { needs_layout: false };

        for StyleChange { surface, prop } in changes {
            match prop {
                // start with layout-independent things
                StyleProp::Color { value } => renderer.set_text_color(*surface, *value),
                StyleProp::BackgroundColor { value } => renderer.set_background_color(*surface, *value),
                //StyleProp::BoxShadow { value } => renderer.set_box_shadow(*surface, *value),

                // TODO: intermediate (top-left, top-right, ...) & set Option<BorderRadius>
                // StyleProp::BorderRadius { surface, value } => renderer.set_border_radius(*surface, *value),

                // TODO: Image

                // TODO: Border
                // might need relayout!

                // layout will be needed
                layout_change => {
                    result.needs_layout = true;

                    match layout_change {
                        StyleProp::Display { value } => {
                            if !self.flex_direction_set.contains(surface) {
                                match value {
                                    Display::None => error!("TODO: display: none"),
                                    Display::Block => box_layout.set_flex_direction(*surface, FlexDirection::Column),
                                    Display::Flex => box_layout.set_flex_direction(*surface, FlexDirection::Row),
                                }
                            }
                        }

                        StyleProp::Width { value } => box_layout.set_width(*surface, *value),
                        StyleProp::Height { value } => box_layout.set_height(*surface, *value),
                        StyleProp::MinWidth { value } => box_layout.set_min_width(*surface, *value),
                        StyleProp::MinHeight { value } => box_layout.set_min_height(*surface, *value),
                        StyleProp::MaxWidth { value } => box_layout.set_max_width(*surface, *value),
                        StyleProp::MaxHeight { value } => box_layout.set_max_height(*surface, *value),

                        StyleProp::Top { value } => box_layout.set_top(*surface, *value),
                        StyleProp::Right { value } => box_layout.set_right(*surface, *value),
                        StyleProp::Bottom { value } => box_layout.set_bottom(*surface, *value),
                        StyleProp::Left { value } => box_layout.set_left(*surface, *value),

                        StyleProp::MarginTop { value } => box_layout.set_margin_top(*surface, *value),
                        StyleProp::MarginRight { value } => box_layout.set_margin_right(*surface, *value),
                        StyleProp::MarginBottom { value } => box_layout.set_margin_bottom(*surface, *value),
                        StyleProp::MarginLeft { value } => box_layout.set_margin_left(*surface, *value),

                        StyleProp::PaddingTop { value } => box_layout.set_padding_top(*surface, *value),
                        StyleProp::PaddingRight { value } => box_layout.set_padding_right(*surface, *value),
                        StyleProp::PaddingBottom { value } => box_layout.set_padding_bottom(*surface, *value),
                        StyleProp::PaddingLeft { value } => box_layout.set_padding_left(*surface, *value),

                        StyleProp::FlexGrow { value } => box_layout.set_flex_grow(*surface, *value),
                        StyleProp::FlexShrink { value } => box_layout.set_flex_shrink(*surface, *value),
                        StyleProp::FlexBasis { value } => box_layout.set_flex_basis(*surface, *value),
                        StyleProp::FlexDirection { value } => {
                            self.flex_direction_set.insert(*surface);
                            box_layout.set_flex_direction(*surface, *value);
                        }
                        StyleProp::FlexWrap { value } => box_layout.set_flex_wrap(*surface, *value),

                        StyleProp::AlignSelf { value } => box_layout.set_align_self(*surface, *value),
                        StyleProp::AlignContent { value } => box_layout.set_align_content(*surface, *value),
                        StyleProp::AlignItems { value } => box_layout.set_align_items(*surface, *value),
                        StyleProp::JustifyContent { value } => box_layout.set_justify_content(*surface, *value),

                        // TODO: this is temporary
                        StyleProp::Text { value } => {
                            text_layout.set_text(*surface, value.clone());
                            // TODO: renderer needs just size & color (which is not part of the text)
                            renderer.set_text(*surface, value.clone());

                            box_layout.set_measure_key(*surface, match value.is_some() {
                                true => Some(*surface),
                                false => None
                            })
                        }

                        /*
                        StyleProp::BackgroundImageUrl { value } => {
                            renderer.set_image(*surface, value);
                        }
                        */

                        _ => { error!("TODO: set {:?}", &layout_change); }
                    }
                }
            }
        }

        result
    }
}