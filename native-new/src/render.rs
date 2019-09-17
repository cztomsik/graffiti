use crate::commons::{Au, Pos, Bounds};
use std::collections::BTreeMap;
use std::io::Write;
use crate::generated::{SurfaceId, UpdateSceneMsg, StyleProp, BoxShadow, Color, Image, Text, Border, BorderRadius};
use crate::text_layout::{TextLayout, GlyphInstance};
use crate::util::Storage;

mod backend;
use crate::render::backend::RenderBackend;

/// All of this is basically just a `render(scene)` function which goes
/// through the tree and draws appropriate visuals using positions & sizes from
/// the `box_layout` & `text_layout` respectively.
///
/// Drawing is expected to be done on the GPU which means we need to first
/// prepare the work (`Frame`) before it can be executed on the `Backend`.
///
/// Some of the work can be shared between the frames so that's why this is
/// stateful and why it's necessary to call some methods if the scene
/// has been changed. The rest of the frame is rebuilt every frame because it's
/// way simpler then.
///
/// TODO: opaque pass to reduce both overdraw & amount of other batches
///
/// TODO: we can split building (preparing the work for the GPU)
///     and the actual rendering (executing gl draw commands) so that we will be
///     1 frame in the past but we can pipeline the work then
///     (build a new frame & draw the previous one in parallel)
//
/// TODO: optimize alloc
pub struct Renderer {
    // TODO: dyn
    backend: RenderBackend,
    pub scene: Scene,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            backend: RenderBackend::new(),
            scene: Scene {
                border_radii: BTreeMap::new(),
                box_shadows: BTreeMap::new(),
                background_colors: BTreeMap::new(),
                images: BTreeMap::new(),
                texts: BTreeMap::new(),
                borders: BTreeMap::new(),
                children: vec![vec![]]
            }
        }
    }

    // TODO: async/pipeline
    pub fn render(&mut self, all_bounds: &[Bounds], text_layout: &TextLayout) {
        let frame = self.prepare_frame(all_bounds, text_layout);
        self.render_frame(frame);
    }

    // TODO: think about finer-grained methods or just introduce them and delegate to them from here for now
    pub fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        for m in msgs.iter().cloned() {
            match m {
                UpdateSceneMsg::Alloc => self.scene.children.push(Vec::new()),
                UpdateSceneMsg::InsertAt { parent, child, index } => self.scene.children[parent].insert(index, child),
                UpdateSceneMsg::RemoveChild { parent, child } => self.scene.children[parent].retain(|ch| *ch != child),
                UpdateSceneMsg::SetStyleProp { surface, prop } => match prop {
                    StyleProp::BorderRadius(r) => self.scene.border_radii.set(surface, r),
                    StyleProp::BoxShadow(s) => self.scene.box_shadows.set(surface, s),
                    StyleProp::BackgroundColor(c) => self.scene.background_colors.set(surface, c),
                    StyleProp::Image(i) => self.scene.images.set(surface, i),
                    StyleProp::Text(t) => self.scene.texts.set(surface, t),
                    StyleProp::Border(b) => self.scene.borders.set(surface, b),
                    _ => {}
                }
            }
        }
    }

    fn prepare_frame(&mut self, all_bounds: &[Bounds], text_layout: &TextLayout) -> Frame {
        debug!("prepare frame");

        let root = 0;
        let mut builder = FrameBuilder::new();

        let mut context = RenderContext {
            text_layout,

            scene: &self.scene,
            all_bounds,
            bounds: all_bounds[root],

            // TODO: find a way to avoid allocations (maybe pool frames?)
            builder: &mut builder,
        };

        context.draw_surface(root);
        builder.finish();

        silly!("alpha_data {:?}", &builder.frame.alpha_data);
        silly!("indices {:?}", &builder.frame.indices);

        builder.frame
    }

    fn render_frame(&mut self, frame: Frame) {
        self.backend.render_frame(frame);
    }
}

struct RenderContext<'a> {
    text_layout: &'a TextLayout,
    scene: &'a Scene,
    all_bounds: &'a[Bounds],

    // TODO: clip
    bounds: Bounds,

    builder: &'a mut FrameBuilder
}

impl <'a> RenderContext<'a> {
    // TODO: bitflags
    fn draw_surface(&mut self, id: SurfaceId) {
        let parent_bounds = self.bounds;

        // TODO: maybe layout should do this too and provide bounds in absolute coords
        self.bounds = self.all_bounds[id].relative_to(parent_bounds.a);

        if let Some(box_shadow) = self.scene.box_shadows.get(&id) {
            self.draw_box_shadow(box_shadow);
        }

        if let Some(color) = self.scene.background_colors.get(&id) {
            self.draw_background_color(color);
        }

        if let Some(image) = self.scene.images.get(&id) {
            self.draw_image(image);
        }

        if let Some(text) = self.scene.texts.get(&id) {
            self.draw_text(text, self.text_layout.get_glyphs(id));
        }

        // TODO: try to avoid recursion?
        for c in &self.scene.children[id] {
            self.draw_surface(*c);
        }

        if let Some(border) = self.scene.borders.get(&id) {
            self.draw_border(border);
        }

        // restore because of recursion
        self.bounds = parent_bounds;
    }

    fn draw_box_shadow(&mut self, shadow: &BoxShadow) {
        // TODO: spread
        // TODO: blur
        self.builder.push_rect(self.bounds, &shadow.color);
    }

    fn draw_background_color(&mut self, color: &Color) {
        self.builder.push_rect(self.bounds, color);
    }

    fn draw_image(&mut self, _image: &Image) {
        // TODO
        self.builder.push_rect(self.bounds, &Color(100, 200, 255, 255));
    }

    // TODO: create_text() -> TextId & Batch::Text(text_id)
    fn draw_text(&mut self, text: &Text, glyphs: &[GlyphInstance]) {
        // TODO: should be uniform
        let origin = self.bounds.a;

        debug!("text {:?} {:?}", &origin, &text.text);

        let Pos { x: start_x, y: start_y } = origin;

        for GlyphInstance { x, y, glyph_id: _ } in glyphs {
            let a = Pos::new(start_x + x, start_y + y);
            let b = Pos::new(start_x + x + 8., start_y + y + 10.);

            self.builder.push_rect(Bounds { a, b }, &text.color);
        }
    }

    fn draw_border(&mut self, Border { top, right, left, bottom }: &Border) {
        // TODO: BorderRadius
        // TODO: width > 0. & style != None

        // TODO: rethink this
        let mut right_bounds = self.bounds;
        right_bounds.a.x = self.bounds.b.x - right.width;

        let mut bottom_bounds = self.bounds;
        bottom_bounds.a.y = self.bounds.b.y - bottom.width;

        let mut left_bounds = self.bounds;
        left_bounds.b.x = self.bounds.a.x + left.width;

        let mut top_bounds = self.bounds;
        top_bounds.b.y = self.bounds.a.y + top.width;

        self.builder.push_rect(top_bounds, &top.color);
        self.builder.push_rect(right_bounds, &right.color);
        self.builder.push_rect(bottom_bounds, &bottom.color);
        self.builder.push_rect(left_bounds, &left.color);
    }
}

/// Represents the work which has to be done to render one "frame".
/// It is split to multiple "batches", sometimes because it's faster
/// and sometimes because of texture/buffer limits, etc.
///
/// It also contains data for this frame.
pub(crate) struct Frame {
    // vertices for generated alpha passes, packed in one buffer
    // this is possible, because they're always written in the drawing order
    alpha_data: Vec<u8>,

    // vertices for opaque primitives have to be separate because they are built
    // out-of-order and memcpy would probably kill all of the benefits of
    // the shared buffer
    //
    // opaque_rect_data: Vec<u8>,

    // for all of the batches
    indices: Vec<VertexIndex>,

    batches: Vec<Batch>
}

enum Batch {
    AlphaRects { num: usize }
}

/// Low-level frame building, can push primitives at given bounds and do
/// some simple optimizations (out-of-order opaque pass)
///
/// Should be unaware of the scene, renderer or anything else
/// (because simple things are more likely to be finished)
///
/// TODO: inspect Vec perf, pushing will be hot
struct FrameBuilder {
    frame: Frame,

    // TODO
    z: Au,

    // quads written, so that we can generate indices
    count: usize,
}

impl FrameBuilder {
    fn new() -> Self {
        Self {
            frame: Frame {
                alpha_data: Vec::new(),
                indices: Vec::new(),
                batches: Vec::new(),
            },
            z: 0.,
            count: 0,
        }
    }

    fn push_rect(&mut self, bounds: Bounds, color: &Color) {
        self.push_quad(color.3 == 255, &Quad::new(bounds, *color));
    }

    fn push_quad<T>(&mut self, _opaque: bool, quad: &Quad<T>) {
        // TODO: alpha colors should be drawn in alpha batches
        // all indices would be relative to the current batch
        // each batch has to start at new offset (important for vertex attrib pointer)
        //
        // if opaque {
        //
        // }

        let slice = unsafe { std::slice::from_raw_parts(std::mem::transmute(quad), std::mem::size_of::<Quad<T>>()) };

        self.frame.alpha_data.write(slice).expect("buf write");

        self.count += 1;
    }

    fn finish(&mut self) {
        // TODO: interleaving
        self.frame.batches.push(Batch::AlphaRects { num: self.count });

        // TODO: opaque

        // TODO: this should be also at the end of each batch
        self.append_indices();
    }

    fn append_indices(&mut self) {
        let data = &mut self.frame.indices;

        for i in 0..self.count {
            let base = (i * 4) as VertexIndex;

            // 6 indices for 2 triangles
            data.push(base + 1);
            data.push(base);
            data.push(base + 3);
            // second one
            data.push(base);
            data.push(base + 2);
            data.push(base + 3);
        }
    }
}

/// All of the state needed during the frame preparation in one struct
/// so it can be borrowed together
/// TODO: rename to UiState (or something what makes more sense)
pub struct Scene {
    border_radii: BTreeMap<SurfaceId, BorderRadius>,
    box_shadows: BTreeMap<SurfaceId, BoxShadow>,
    background_colors: BTreeMap<SurfaceId, Color>,
    images: BTreeMap<SurfaceId, Image>,
    texts: BTreeMap<SurfaceId, Text>,
    borders: BTreeMap<SurfaceId, Border>,

    // TODO: lift children up
    pub children: Vec<Vec<SurfaceId>>
}


/// Everything what's rendered, is quad-based, it's easier to imagine then
#[derive(Debug)]
struct Quad<T>([Vertex<T>; 4]);

impl <T: Copy> Quad<T> {
    fn new(Bounds { a, b }: Bounds, data: T) -> Self {
        Self([
            Vertex(a, data),
            Vertex(Pos::new(b.x, a.y), data),
            Vertex(Pos::new(a.x, b.y), data),
            Vertex(b, data),
        ])
    }
}

/// Vertex including some primitive-specific attributes
#[derive(Debug)]
struct Vertex<T>(Pos, T);

// for indexed drawing
// raspi can do only 65k vertices in one batch
// could be configurable but it's probably better to play it safe
type VertexIndex = u16;

// TODO: inspect if Color is really copied and consider #[repr(u32)] instead
// TODO: inspect Bounds copying too
impl Copy for Color {}

// TODO: once opaque pass is added, Z is needed for both opaque & alpha passes!
type Rect = Quad<Color>;
