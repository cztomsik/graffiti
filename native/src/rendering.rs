use crate::resources::OpResource;
use crate::surface::Surface;
use webrender::api::euclid::{TypedPoint2D, TypedRect, TypedSize2D};
use webrender::api::{
    BorderDisplayItem, BorderRadius, ClipMode, ColorF, ComplexClipRegion, DisplayListBuilder,
    ExternalScrollId, GlyphInstance, LayoutPixel, LayoutPoint, LayoutPrimitiveInfo, LayoutRect,
    LayoutSize, PipelineId, PushStackingContextDisplayItem, RectangleDisplayItem,
    ScrollSensitivity, StackingContext, TextDisplayItem,
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
    pub offset_x: f32,
    pub offset_y: f32,
}

pub fn render_surface(ctx: &mut RenderContext, surface: &Surface) {
    let Surface {
        brush,
        clip,
        yoga_node,
        children,
    } = surface;
    let layout = yoga_node.get_layout();

    if let Some(brush) = brush {
        render_op_resource(ctx, brush, &layout);
    }

    if let Some(clip) = clip {
        render_op_resource(ctx, clip, &layout);
    }

    ctx.offset_x += layout.left();
    ctx.offset_y += layout.top();

    for ch in children {
        render_surface(ctx, &ch.borrow());
    }

    if clip.is_some() {
        render_op(ctx, &RenderOperation::PopClip, &layout);
    }
}

fn render_op_resource(ctx: &mut RenderContext, op_resource: &OpResource, layout: &Layout) {
    let OpResource { start, length } = op_resource;

    for i in *start..(*start + *length) {
        render_op(ctx, &ctx.ops[i as usize], layout)
    }
}

fn render_op(ctx: &mut RenderContext, op: &RenderOperation, layout: &Layout) {
    let b = &mut ctx.builder;
    let mut info = layout.to_layout_info(ctx.offset_x, ctx.offset_y);

    debug!("render {:?} {:?}", op, &info);

    match op {
        RenderOperation::HitTest(tag) => {
            info.tag = Some((*tag as u64, 0 as u16));
            b.push_rect(&info, ColorF::TRANSPARENT);
        }
        RenderOperation::SaveRect => {
            ctx.saved_rect = layout.to_layout_rect();
            debug!("saved rect {:?}", ctx.saved_rect);
        }
        RenderOperation::PushBorderRadiusClip(radius) => {
            let radii = BorderRadius::uniform(*radius);

            let complex_clip =
                ComplexClipRegion::new(layout.to_layout_rect(), radii, ClipMode::Clip);

            let clip_id = b.define_clip(layout.to_layout_rect(), vec![complex_clip], None);

            b.push_clip_id(clip_id);
        }
        RenderOperation::PushScrollClip(id) => {
            let clip_id = b.define_scroll_frame(
                Some(ExternalScrollId(*id, ctx.pipeline_id)),
                layout.to_layout_rect(),
                ctx.saved_rect,
                vec![],
                None,
                ScrollSensitivity::ScriptAndInputEvents,
            );

            debug!(
                "push scroll clip clip = {:?} content = {:?}",
                ctx.saved_rect,
                layout.to_layout_rect()
            );

            b.push_clip_id(clip_id);
        }
        RenderOperation::PopClip => b.pop_clip_id(),
        RenderOperation::Text(
            TextDisplayItem {
                font_key,
                color,
                glyph_options,
            },
            glyphs,
        ) => b.push_text(&info, glyphs, *font_key, *color, *glyph_options),
        RenderOperation::Rectangle(RectangleDisplayItem { color }) => b.push_rect(&info, *color),
        RenderOperation::Border(BorderDisplayItem { widths, details }) => {
            b.push_border(&info, *widths, *details)
        }
        RenderOperation::PopStackingContext => b.pop_stacking_context(),

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

            b.push_stacking_context(
                &info,
                *clip_node_id,
                *transform_style,
                *mix_blend_mode,
                &Vec::new(),
                *raster_space,
            )
        }
    }
}

pub trait LayoutHelpers {
    fn to_layout_rect(&self) -> LayoutRect;
    fn to_layout_info(&self, x: f32, y: f32) -> LayoutPrimitiveInfo;
}

impl LayoutHelpers for Layout {
    fn to_layout_rect(&self) -> LayoutRect {
        LayoutRect::new(
            TypedPoint2D::new(self.left(), self.top()),
            TypedSize2D::new(self.width(), self.height()),
        )
    }

    fn to_layout_info(&self, x: f32, y: f32) -> LayoutPrimitiveInfo {
        let (left, top, width, height) =
            (x + self.left(), y + self.top(), self.width(), self.height());
        let layout_rect =
            LayoutRect::new(LayoutPoint::new(left, top), LayoutSize::new(width, height));

        LayoutPrimitiveInfo::new(layout_rect)
    }
}
