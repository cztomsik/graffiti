// turns render tree into series of skia calls
//
// maybe we can get rid of skia one day or make it an optional feature/backend
// but it's not a priority right now and there are no good alternatives
// at the moment anyway (clip radii, filters)
//
// notes:
// - render tree is just a slice of `RenderEdge`(s) which means it should be prefetch-friendly
//   and the `ContainerStyle` doesn't need to be <64b
// - incrementality will be handled elsewhere, this should be as simple and stateless as possible
//   except of maybe using some LRU caches for managing mid-lived resources, etc.

use skia_safe::textlayout::Paragraph;
use skia_safe::{
    gpu::{gl::FramebufferInfo, BackendRenderTarget, DirectContext},
    Canvas, ColorType, Paint, RRect, Surface,
};
use skia_safe::{ClipOp, MaskFilter};
use std::ops::Deref;

// for now we just re-export some skia primitives
pub use skia_safe::{Color, Matrix, Point, Rect};

pub enum RenderEdge<P: Deref<Target = Paragraph>> {
    OpenContainer(Rect, ContainerStyle),
    CloseContainer,
    Text(Rect, P),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ContainerStyle {
    pub transform: Option<Matrix>,
    pub opacity: Option<f32>,
    pub border_radii: Option<[f32; 4]>,
    pub shadow: Option<Shadow>,
    pub outline: Option<Outline>,
    pub clip: bool,
    pub bg_color: Option<Color>,
    // pub images/gradients: Vec<?>
    pub border: Option<Border>,
}

#[derive(Debug, Clone, Copy)]
pub struct Shadow(pub (f32, f32), pub f32, pub f32, pub Color);

#[derive(Debug, Clone, Copy)]
pub struct Outline(pub f32, pub Option<StrokeStyle>, pub Color);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeStyle {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Debug, Clone, Copy)]
pub struct Border {
    top: BorderSide,
    right: BorderSide,
    bottom: BorderSide,
    left: BorderSide,
}

#[derive(Debug, Clone, Copy)]
pub struct BorderSide(pub f32, pub Option<StrokeStyle>, pub Color);

pub struct Renderer {
    gr_ctx: DirectContext,
    surface: Surface,
}

struct RenderContext<'a> {
    canvas: &'a mut Canvas,
}

enum Shape {
    Rect(Rect),
    RRect(RRect),
}

impl Renderer {
    pub fn new(size: (i32, i32)) -> Self {
        let mut gr_ctx = DirectContext::new_gl(None, None).unwrap();
        let surface = Self::create_surface(&mut gr_ctx, size);

        Self { gr_ctx, surface }
    }

    pub fn render<P: Deref<Target = Paragraph>>(&mut self, render_tree: &[RenderEdge<P>]) {
        let canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);

        let mut ctx = RenderContext { canvas };

        for edge in render_tree {
            ctx.draw_edge(edge);
        }

        self.surface.flush();
    }

    pub fn resize(&mut self, size: (i32, i32)) {
        self.surface = Self::create_surface(&mut self.gr_ctx, size);
    }

    fn create_surface(gr_ctx: &mut DirectContext, size: (i32, i32)) -> Surface {
        let target = BackendRenderTarget::new_gl(
            size,
            None,
            8,
            FramebufferInfo {
                fboid: 0,
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
            },
        );

        Surface::from_backend_render_target(
            gr_ctx,
            &target,
            skia_safe::gpu::SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap()
    }
}

impl RenderContext<'_> {
    fn draw_edge<P: Deref<Target = Paragraph>>(&mut self, edge: &RenderEdge<P>) {
        match edge {
            RenderEdge::OpenContainer(rect, style) => self.open_container(*rect, style),
            RenderEdge::CloseContainer => self.close_container(),
            RenderEdge::Text(rect, paragraph) => self.draw_text(*rect, paragraph),
        }
    }

    fn open_container(&mut self, rect: Rect, style: &ContainerStyle) {
        self.canvas.save();

        let shape = match &style.border_radii {
            None => Shape::Rect(rect),
            Some(radii) => {
                let mut rrect = RRect::default();
                let &[a, b, c, d] = radii;
                rrect.set_rect_radii(rect, &[(a, a).into(), (b, b).into(), (c, c).into(), (d, d).into()]);

                Shape::RRect(rrect)
            }
        };

        if let Some(matrix) = &style.transform {
            self.canvas.concat(matrix);
        }

        if let Some(opacity) = style.opacity {
            // TODO
        }

        if let Some(shadow) = &style.shadow {
            self.draw_shadow(rect, shadow);
        }

        if let Some(outline) = &style.outline {
            self.draw_outline(rect, outline);
        }

        if style.clip {
            // TODO: subtract borders?
            self.clip_shape(&shape, ClipOp::Intersect, style.transform.is_some());
        }

        if let Some(color) = style.bg_color {
            self.draw_bg_color(&shape, color);
        }

        // TODO: image(s)

        // TODO: scroll
        self.canvas.translate((rect.x(), rect.y()));
    }

    fn close_container(&mut self) {
        //let (border,) = ctx.stack.pop().unwrap();

        // if let Some(border) = ctx.stack.pop {
        //     self.draw_border(border_rect, border_radii, border)
        // }

        self.canvas.restore();
    }

    fn draw_text(&mut self, rect: Rect, paragraph: &Paragraph) {
        paragraph.paint(self.canvas, Point::new(rect.x(), rect.y()));
    }

    fn draw_outline(&mut self, rect: Rect, outline: &Outline) {
        let &Outline(width, style, color) = outline;

        let mut paint = Paint::default();
        paint.set_stroke(true);
        paint.set_stroke_width(width);
        paint.set_color(color);

        // TODO: stroke styles

        let d = width / 2.;

        self.canvas.draw_rect(rect.with_outset((d, d)), &paint);
    }

    fn draw_bg_color(&mut self, shape: &Shape, color: Color) {
        let mut paint = Paint::default();
        paint.set_color(color);

        self.draw_shape(shape, &paint);
    }

    // TODO: radii
    fn draw_shadow(&mut self, rect: Rect, shadow: &Shadow) {
        let &Shadow(offset, blur, spread, color) = shadow;

        let mut paint = Paint::default();
        paint.set_color(color);
        paint.set_mask_filter(MaskFilter::blur(skia_safe::BlurStyle::Outer, sigma(blur), false));

        // TODO: fix negative spread (visible with transparent background)
        //       maybe draw those with inverse clip?
        self.canvas
            .draw_rect(rect.with_offset(offset).with_outset((spread, spread)), &paint);
    }

    fn draw_shape(&mut self, shape: &Shape, paint: &Paint) {
        match shape {
            Shape::Rect(rect) => self.canvas.draw_rect(rect, paint),
            Shape::RRect(rrect) => self.canvas.draw_rrect(rrect, paint),
        };
    }

    fn clip_shape(&mut self, shape: &Shape, op: ClipOp, antialias: bool) {
        match shape {
            Shape::Rect(rect) => self.canvas.clip_rect(rect, op, antialias),
            Shape::RRect(rrect) => self.canvas.clip_rrect(rrect, op, antialias),
        };
    }
}

fn sigma(radius: f32) -> f32 {
    // (1 / sqrt(3))
    0.5773502692 * radius + 0.5
}
