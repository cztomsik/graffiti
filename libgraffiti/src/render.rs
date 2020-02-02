use std::collections::BTreeMap;
use crate::commons::{ElementId, TextId, ElementChild, Pos, Bounds, Color};
use crate::util::{Storage};
use crate::box_layout::{BoxLayoutTree, BoxLayoutImpl};
use crate::text_layout::GlyphInstance;

/// Goes through the tree and draws appropriate visuals at given bounds.
///
/// Drawing is expected to be done on the GPU which means we need to first
/// prepare the work before it can be executed on the `RenderBackend`.
///
/// The implementation is intentionally incorrect for the sake of simplicity and
/// CPU performance. It is designed to be fast enough to run on raspi and
/// it takes some shortcuts in order to do so. For example, it's possible to fool
/// clipping if you know how to.
///
/// This will be either fixed or maybe in the future, we might introduce some other
/// (optional) impl which would delegate to something more demanding but also
/// more complete (skia, maybe pathfinder but that would require GL3)
pub trait Renderer {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId);

    fn set_transform(&mut self, element: ElementId, transform: Option<Transform>);
    fn set_border_radius(&mut self, element: ElementId, border_radius: Option<BorderRadius>);
    fn set_box_shadow(&mut self, element: ElementId, box_shadow: Option<BoxShadow>);
    fn set_background_color(&mut self, element: ElementId, color: Color);
    fn set_color(&mut self, element: ElementId, color: Color);
    fn set_image(&mut self, element: ElementId, image: Option<Image>);
    fn set_border(&mut self, element: ElementId, border: Option<Border>);

    fn set_text_glyphs(&mut self, text: TextId, size: f32, glyphs: impl Iterator<Item=GlyphInstance>);

    // TODO: Index<ElementId/TextId, Output=Bounds> to break coupling
    // TODO: flags (bounds changed, tree changed)
    fn render(&mut self, element: ElementId, children: &[Vec<ElementChild>], box_layout: &BoxLayoutImpl);

    fn resize(&mut self, size: (f32, f32));
}

// some structs in the renderer-friendly way
// Option<BorderRadius> vs testing for non-zero values
// this however isn't necessarily how it's internally stored 

#[derive(Debug, Clone, Copy)]
pub struct BorderRadius {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Pos,
    pub blur: f32,
    pub spread: f32,
}

// TODO
pub struct Image {}
pub struct Border {}

#[derive(Debug, Clone, Copy)]
pub enum Transform {
    //Translate { x: f32, y: f32 },
    Scale { x: f32, y: f32 },
    //Rotate { deg: f32 },
    //Skew { x: f32, y: f32 },
}

// impl starts here

pub struct RendererImpl {
    backend: GlRenderBackend,
    ui_state: UiState,
}

// TODO: rename
// (read-only) input for rendering which was prepared during set_* calls
struct UiState {
    //flags: Vec<FastPathFlags>,
    transforms: BTreeMap<ElementId, Mat3>,
    background_colors: Vec<Color>,
    colors: Vec<Color>,
    //border_radii: BTreeMap<ElementId, BorderRadius>,
    //box_shadows: BTreeMap<ElementId, BoxShadow>,
    //images: BTreeMap<ElementId, Image>,
    //borders: BTreeMap<Element, Border>,
}

#[allow(dead_code)]
type FastPathFlags = u8;

impl RendererImpl {
    pub fn new(size: (f32, f32)) -> Self {
        let mut res = RendererImpl {
            backend: GlRenderBackend::new(),
            ui_state: UiState {
                transforms: BTreeMap::new(),
                background_colors: Vec::new(),
                colors: Vec::new(),
            },
        };

        res.resize(size);

        res
    }
}

impl Renderer for RendererImpl {
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId) {
        self.ui_state.background_colors.resize(elements_count, Color::TRANSPARENT);
        self.ui_state.colors.resize(elements_count, Color::BLACK);

        self.backend.realloc(texts_count);
    }

    fn set_transform(&mut self, element: ElementId, transform: Option<Transform>) {
        self.ui_state.transforms.set(element, transform.map(|t| match t {
            Transform::Scale { x, y } => Mat3([
                x, 0., 0.,
                0., y, 0.,
                0., 0., 1.,
            ])
        }));
    }

    fn set_border_radius(&mut self, _element: ElementId, _border_radius: Option<BorderRadius>) {
        todo!()
    }

    fn set_box_shadow(&mut self, _element: ElementId, _shadow: Option<BoxShadow>) {
        todo!()
    }

    fn set_background_color(&mut self, element: ElementId, color: Color) {
        self.ui_state.background_colors[element] = color;
    }

    fn set_color(&mut self, element: ElementId, color: Color) {
        self.ui_state.colors[element] = color;
    }

    fn set_image(&mut self, _element: ElementId, _image: Option<Image>) {
        todo!()
    }

    fn set_border(&mut self, _element: ElementId, _border: Option<Border>) {
        todo!()
    }

    fn set_text_glyphs(&mut self, text: TextId, size: f32, glyphs: impl Iterator<Item=GlyphInstance>) {
        // TODO: at this level, glyph instance should be pos + index and here should be the mapping
        // to texture + coords
        self.backend.set_text_glyphs(text, size, glyphs.map(|g| (g.bounds, g.coords)));
    }

    fn render(&mut self, element: ElementId, children: &[Vec<ElementChild>], box_layout: &BoxLayoutImpl) {
        // TODO: skip if only tranforms were changed
        self.backend.clear();

        // TODO:
        // - skip bounds
        // - skip rects
        // - skip ops
        let mut ctx = RenderContext {
            children,
            box_layout,

            ui_state: &self.ui_state,
            parent_bounds: Bounds::ZERO,

            backend: &mut self.backend,
        };

        ctx.render_element(element);

        self.backend.render();
    }

    fn resize(&mut self, size: (f32, f32)) {
        self.backend.resize(size);
    }
}

struct RenderContext<'a> {
    children: &'a [Vec<ElementChild>],
    box_layout: &'a BoxLayoutImpl,

    ui_state: &'a UiState,
    parent_bounds: Bounds,

    backend: &'a mut GlRenderBackend,
}

impl <'a> RenderContext<'a> {
    fn render_element(&mut self, element: ElementId) {
        // TODO: skip if empty/hidden (fastpath flag)

        // TODO: skip if layout is up-to-date
        let bounds = self.box_layout.get_element_bounds(element);
        let mut pop_transform = false;

        if let Some(transform) = self.ui_state.transforms.get(&element) {
            let origin = bounds.center().relative_to(self.parent_bounds.a);

            self.backend.push_transform(*transform, origin);
            pop_transform = true;
        }

        let bounds = bounds.relative_to(self.parent_bounds.a);

        // TODO: clip

        // TODO: shadow

        let bg_col = self.ui_state.background_colors[element];

        if bg_col.a != 0 {
            self.backend.push_rect(bounds, bg_col);
        }

        // TODO: gradient
        // TODO: image
        // TODO: inner shadow

        // TODO: clip children too if overflow: hidden

        // render children
        let parent_bounds = self.parent_bounds;
        self.parent_bounds = bounds;

        for c in &self.children[element] {
            match c {
                // TODO: replace recursion with iteration
                ElementChild::Element { id } => self.render_element(*id),
                ElementChild::Text { id } => self.render_text(*id, self.box_layout.get_text_bounds(*id).a, self.ui_state.colors[element]),
            }
        }

        if pop_transform {
            self.backend.pop_transform();
        }

        // TODO: border

        self.parent_bounds = parent_bounds;
    }

    fn render_text(&mut self, id: TextId, pos: Pos, color: Color) {
        self.backend.push_text(id, pos.relative_to(self.parent_bounds.a), color);
    }
}

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitives, prepared by higher-level `Renderer`
///
/// Backend does the real drawing but it's also very simple and can't do much
/// optimizations and has absolutely no idea about scene or anything else.
/// You don't want to use it directly and so it's useless just by itself.
pub trait RenderBackend {
    fn realloc(&mut self, texts_count: TextId);

    fn set_text_glyphs(&mut self, text: TextId, size: f32, glyphs: impl Iterator<Item=(Bounds, Bounds)>);

    fn clear(&mut self);
    fn push_transform(&mut self, transform: Mat3, origin: Pos);
    fn pop_transform(&mut self);
    fn push_rect(&mut self, bounds: Bounds, color: Color);
    fn push_text(&mut self, id: TextId, pos: Pos, color: Color);

    // TODO: initial_clip (update region) to reduce overdraw (of up-to-date parts)
    // and given how GPUs work it should finish faster (more free units) 
    fn render(&mut self);

    fn resize(&mut self, size: (f32, f32));
}

mod gl;
use self::gl::{GlRenderBackend, Mat3};
