use crate::resources::OpResource;
use crate::surface::Surface;
use webrender::api::euclid::{TypedPoint2D, TypedRect, TypedSize2D};
use webrender::api::{
    BorderDisplayItem, BorderRadius, ClipMode, ColorF, ComplexClipRegion, DisplayListBuilder,
    ExternalScrollId, GlyphInstance, LayoutPixel, LayoutPoint, LayoutPrimitiveInfo, LayoutRect,
    MixBlendMode, PipelineId, PushStackingContextDisplayItem, RasterSpace, RectangleDisplayItem,
    ScrollSensitivity, StackingContext, TextDisplayItem, TransformStyle,
};
use yoga::Layout;

// operation to be performed during render, like builder.push_rect(), or api.hit_test()
// a bit like cpu opcode, it can have some parameters but it also can be relative to current
// state (registers) and/or provided layout rects (memory)
// we are really doing some kind of very simple virtual machine
#[derive(Deserialize, Debug, Clone)]
pub enum RenderOperation {
    // this was hack at first but it could be useful for hitSlop (hitBox can be bigger than clipBox)
    HitTest(u32),
    SaveRect,
    PushScrollClip(u64),
    PushBorderRadiusClip(f32),
    PopClip,
    Rectangle(RectangleDisplayItem),
    Border(BorderDisplayItem),
    Text(TextDisplayItem, Vec<GlyphInstance>),
    PopStackingContext,
    PushStackingContext(PushStackingContextDisplayItem),
}

pub struct RenderContext<'a> {
    pub pipeline_id: PipelineId,
    pub ops: &'a Vec<RenderOperation>,
    pub builder: &'a mut DisplayListBuilder,
    pub saved_rect: TypedRect<f32, LayoutPixel>,
    pub offset: (f32, f32),

    // for debugging purposes only
    pub depth: usize,
}

impl<'a> RenderContext<'a> {
    pub fn render_surface(&mut self, surface: &Surface) {
        let Surface {
            brush,
            clip,
            yoga_node,
            children,
        } = surface;
        let mut layout_rect = yoga_node.get_layout().to_layout_rect();
        layout_rect.origin.x += self.offset.0;
        layout_rect.origin.y += self.offset.1;

        debug!(
            "{}+ {:?} {:?} {:?}",
            String::from_utf8(vec![b' '; self.depth]).unwrap(),
            &brush,
            &clip,
            &layout_rect
        );

        if let Some(clip) = clip {
            self.render_op_resource(clip, &layout_rect);
        }

        if let Some(brush) = brush {
            self.render_op_resource(brush, &layout_rect);
        }

        if !children.is_empty() {
            let parent_offset = self.offset;

            self.depth += 1;
            self.offset = (layout_rect.origin.x, layout_rect.origin.y);

            for ch in children {
                self.render_surface(&ch.borrow());
            }

            self.offset = parent_offset;
            self.depth -= 1;
        }

        if clip.is_some() {
            self.render_op(&RenderOperation::PopClip, &layout_rect);
        }
    }

    fn render_op_resource(&mut self, op_resource: &OpResource, layout_rect: &LayoutRect) {
        let OpResource { start, length } = op_resource;

        for i in *start..(*start + *length) {
            self.render_op(&self.ops[i as usize], layout_rect)
        }
    }

    fn render_op(&mut self, op: &RenderOperation, layout_rect: &LayoutRect) {
        debug!(
            "{} - {:?}",
            String::from_utf8(vec![b' '; self.depth]).unwrap(),
            op
        );

        let mut info = LayoutPrimitiveInfo::new(layout_rect.clone());

        match op {
            RenderOperation::HitTest(tag) => {
                info.tag = Some((*tag as u64, 0 as u16));
                self.builder.push_rect(&info, ColorF::TRANSPARENT);
            }
            RenderOperation::SaveRect => {
                self.saved_rect = layout_rect.clone();
                debug!("saved rect {:?}", self.saved_rect);
            }
            RenderOperation::PushBorderRadiusClip(radius) => {
                let radii = BorderRadius::uniform(*radius);

                let complex_clip =
                    ComplexClipRegion::new(layout_rect.clone(), radii, ClipMode::Clip);

                let clip_id =
                    self.builder
                        .define_clip(layout_rect.clone(), vec![complex_clip], None);

                self.builder.push_clip_id(clip_id);
            }
            RenderOperation::PushScrollClip(id) => {
                let clip_id = self.builder.define_scroll_frame(
                    Some(ExternalScrollId(*id, self.pipeline_id)),
                    layout_rect.clone(),
                    self.saved_rect,
                    vec![],
                    None,
                    ScrollSensitivity::ScriptAndInputEvents,
                );

                debug!(
                    "push scroll clip clip = {:?} content = {:?}",
                    self.saved_rect, layout_rect
                );

                self.builder.push_clip_id(clip_id);
            }
            RenderOperation::PopClip => self.builder.pop_clip_id(),
            RenderOperation::Text(
                TextDisplayItem {
                    font_key,
                    color,
                    glyph_options,
                },
                glyphs,
            ) => {
                self.builder.push_stacking_context(
                    &info,
                    None,
                    TransformStyle::Flat,
                    MixBlendMode::Normal,
                    &vec![],
                    RasterSpace::Screen,
                );

                let text_info = LayoutPrimitiveInfo::new(LayoutRect::new(
                    LayoutPoint::new(0., 0.),
                    layout_rect.size.clone(),
                ));
                self.builder
                    .push_text(&text_info, glyphs, *font_key, *color, *glyph_options);

                self.builder.pop_stacking_context();
            }
            RenderOperation::Rectangle(RectangleDisplayItem { color }) => {
                self.builder.push_rect(&info, *color)
            }
            RenderOperation::Border(BorderDisplayItem { widths, details }) => {
                self.builder.push_border(&info, *widths, *details)
            }
            RenderOperation::PopStackingContext => self.builder.pop_stacking_context(),

            // TODO: filters
            RenderOperation::PushStackingContext(PushStackingContextDisplayItem {
                stacking_context,
            }) => {
                let StackingContext {
                    transform_style,
                    mix_blend_mode,
                    clip_node_id,
                    raster_space,
                } = stacking_context;

                self.builder.push_stacking_context(
                    &info,
                    *clip_node_id,
                    *transform_style,
                    *mix_blend_mode,
                    &Vec::new(),
                    *raster_space,
                );
                self.offset = (0., 0.);
            }
        }
    }
}

pub trait LayoutHelpers {
    fn to_layout_rect(&self) -> LayoutRect;
}

impl LayoutHelpers for Layout {
    fn to_layout_rect(&self) -> LayoutRect {
        LayoutRect::new(
            TypedPoint2D::new(self.left(), self.top()),
            TypedSize2D::new(self.width(), self.height()),
        )
    }
}
