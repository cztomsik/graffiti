// x decoupled from layout, styles, ... (commons & utils are allowed)
//   x create resources, return handles
// x types & granularity suited for rendering
// x easy to test (pass vec of bounds)
// x stateful

use crate::commons::{Bounds, Lookup, Mat3, Pos};

// handles
// public but opaque types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageId(usize);

// re-export value types
pub mod value_types;
use self::value_types::*;

// and backend
pub mod backend;
use self::backend::{BackendOp, FillStyle, RenderBackend};

// `BK` is some key to get layout bounds
pub struct Renderer<RB: RenderBackend, BK> {
    backend: RB,
    ui_state: UiState<RB, BK>,
}

impl<RB: RenderBackend, BK: Copy> Renderer<RB, BK> {
    pub fn new(backend: RB) -> Self {
        Self { backend, ui_state: UiState::new() }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.backend.resize(width, height);
    }

    // surface
    pub fn create_surface(&mut self, bounds_key: BK) -> SurfaceId {
        // maybe defaults shouldn't be here but it's probably inevitable,
        // yoga has some defaults, this has some too, etc.
        self.ui_state.bounds_keys.push(bounds_key);
        self.ui_state.transforms.push(None);
        self.ui_state.opacities.push(1.);
        self.ui_state.border_radii.push(None);
        self.ui_state.overflows.push(Overflow::Visible);
        self.ui_state.outline_shadows.push(Vec::new());
        self.ui_state.outlines.push(None);
        self.ui_state.background_colors.push(Color::TRANSPARENT);
        self.ui_state.background_images.push(Vec::new());
        self.ui_state.inset_shadows.push(Vec::new());
        self.ui_state.colors.push(Color::BLACK);
        self.ui_state.contents.push(None);
        self.ui_state.borders.push(None);

        SurfaceId(self.ui_state.background_colors.len() - 1)
    }

    // setters (in order in which they are needed during rendering)

    pub fn set_transform(&mut self, surface: SurfaceId, value: Option<Mat3>) {
        self.ui_state.transforms[surface.0] = value;
    }

    pub fn set_overflow(&mut self, surface: SurfaceId, value: Overflow) {
        self.ui_state.overflows[surface.0] = value;
    }

    pub fn set_opacity(&mut self, surface: SurfaceId, value: f32) {
        self.ui_state.opacities[surface.0] = value;
    }

    pub fn set_border_radius(&mut self, surface: SurfaceId, value: Option<BorderRadius>) {
        self.ui_state.border_radii[surface.0] = value;
    }

    pub fn set_outline_shadows(&mut self, surface: SurfaceId, value: Vec<OutlineShadow>) {
        self.ui_state.outline_shadows[surface.0] = value;
    }

    pub fn set_outline(&mut self, surface: SurfaceId, value: Option<Outline>) {
        self.ui_state.outlines[surface.0] = value;
    }

    pub fn set_background_color(&mut self, surface: SurfaceId, value: Color) {
        self.ui_state.background_colors[surface.0] = value;
    }

    pub fn set_background_images(&mut self, surface: SurfaceId, value: Vec<BackgroundImage>) {
        self.ui_state.background_images[surface.0] = value;
    }

    pub fn set_inset_shadows(&mut self, surface: SurfaceId, value: Vec<InsetShadow>) {
        self.ui_state.inset_shadows[surface.0] = value;
    }

    // content setters (mutually exlusive)
    //   - children
    //   - text
    //
    // could be struct but it'd need to be generic or accept &mut dyn Iterator
    // and it'd more verbose too: `set_content(Some(Content::Children(...)))`
    //
    // TODO: video/canvas
    //       - texture/image
    //       - maybe accept uv too
    //       - bounds are not known until render
    //       - inset shadow should not cover the content (tried in browser)
    //         - so it can't be done using BackgroundImage
    //
    // TODO: svg
    //       much harder, and out-of-scope for now but it should be
    //       scaled just like video/canvas using current surface bounds
    //       and it has to be rendered after inset shadows too
    //       note the text could be rendered using paths just like svg
    //       but text is composed of predefined glyphs unlike svg where
    //       each path can be unique and has to be rasterized

    pub fn set_children(&mut self, surface: SurfaceId, children: &[SurfaceId]) {
        // reuse prev vec
        if let Some(RenderContent::Children(prev)) = &mut self.ui_state.contents[surface.0] {
            prev.splice(0.., children.iter().copied());

            // shrink if it was too big (pathological case)
            if prev.capacity() > 100 {
                prev.shrink_to_fit();
            }

            return;
        }

        if children.is_empty() {
            return self.set_new_content(surface, None);
        }

        self.set_new_content(surface, Some(RenderContent::Children(children.into())));
    }

    pub fn set_text(&mut self, surface: SurfaceId, texture: RB::TextureId, glyphs: impl Iterator<Item = (Bounds, Bounds)>) {
        let glyphs = glyphs.map(|(bounds, uv)| {
            BackendOp::PushRect(
                bounds,
                FillStyle::Msdf {
                    texture,
                    uv,
                    factor: 0.5,
                    color: Color::RED,
                },
            )
        });

        // reuse prev layer
        if let Some(RenderContent::Text(prev)) = self.ui_state.contents[surface.0] {
            return self.backend.update_layer(prev, glyphs);
        }

        let layer = self.backend.create_layer();
        self.backend.update_layer(layer, glyphs);

        self.set_new_content(surface, Some(RenderContent::Text(layer)));
    }

    // cleanup prev value & set a new one
    fn set_new_content(&mut self, surface: SurfaceId, content: Option<RenderContent<RB>>) {
        if let Some(prev) = &self.ui_state.contents[surface.0] {
            match prev {
                RenderContent::Text(_prev) => println!("TODO: free prev text layer"),

                _ => {}
            }
        }

        self.ui_state.contents[surface.0] = content;
    }

    // TODO: set_text_shadow

    pub fn set_color(&mut self, surface: SurfaceId, value: Color) {
        self.ui_state.colors[surface.0] = value;
    }

    pub fn set_border(&mut self, surface: SurfaceId, value: Option<Border>) {
        self.ui_state.borders[surface.0] = value;
    }

    // high-level image, rect-packed into some existing or newly created texture
    // note that sharing texture can improve/decrease performance
    // depending on how much the images are changing
    pub fn create_image(&mut self, width: i32, height: i32) -> ImageId {
        // TODO: put it to some existing/new texture (rect-packing)
        self.ui_state.textures.push(self.backend.create_texture(width, height));

        ImageId(self.ui_state.textures.len() - 1)
    }

    pub fn set_image_data(&mut self /* data: rgb &[u8] */) {}

    // low-level image resource
    // suitable for videos and other dynamic image data
    pub fn create_texture(&mut self, width: i32, height: i32) -> RB::TextureId {
        self.backend.create_texture(width, height)
    }

    pub fn update_texture(&mut self, texture: RB::TextureId, data: &[u8]) {
        self.backend.update_texture(texture, data);
    }

    pub fn render_surface(&mut self, surface: SurfaceId, bounds: &impl Lookup<BK, Bounds>) {
        let current_bounds = bounds.lookup(self.ui_state.bounds_keys[surface.0]);
        let ui_state = &self.ui_state;

        // TODO: reuse
        let mut builder = Vec::new();

        let mut ctx = RenderContext {
            builder: &mut builder,
            ui_state,
            bounds,
            current_bounds,
        };

        ctx.render_surface(surface);

        self.backend.render(builder.into_iter());
    }
}

// internal impl starts here

// data-oriented storage
struct UiState<RB: RenderBackend, BK> {
    bounds_keys: Vec<BK>,
    opacities: Vec<f32>,
    overflows: Vec<Overflow>,
    transforms: Vec<Option<Mat3>>,
    border_radii: Vec<Option<BorderRadius>>,
    outline_shadows: Vec<Vec<OutlineShadow>>,
    outlines: Vec<Option<Outline>>,
    background_colors: Vec<Color>,
    background_images: Vec<Vec<BackgroundImage>>,
    inset_shadows: Vec<Vec<InsetShadow>>,
    colors: Vec<Color>,
    contents: Vec<Option<RenderContent<RB>>>,
    borders: Vec<Option<Border>>,

    textures: Vec<RB::TextureId>,
}

impl<RB: RenderBackend, BK: Copy> UiState<RB, BK> {
    fn new() -> Self {
        Self {
            bounds_keys: Vec::new(),
            opacities: Vec::new(),
            overflows: Vec::new(),
            transforms: Vec::new(),
            border_radii: Vec::new(),
            outline_shadows: Vec::new(),
            outlines: Vec::new(),
            background_colors: Vec::new(),
            background_images: Vec::new(),
            inset_shadows: Vec::new(),
            colors: Vec::new(),
            contents: Vec::new(),
            borders: Vec::new(),

            textures: Vec::new(),
        }
    }
}

struct RenderContext<'a, RB: RenderBackend, BK: Copy, BS: Lookup<BK, Bounds>> {
    builder: &'a mut Vec<BackendOp<RB>>,
    ui_state: &'a UiState<RB, BK>,
    bounds: &'a BS,
    current_bounds: Bounds,
}

impl<RB: RenderBackend, BK: Copy, BS: Lookup<BK, Bounds>> RenderContext<'_, RB, BK, BS> {
    fn render_surface(&mut self, surface: SurfaceId) {
        if let Some(t) = &self.ui_state.transforms[surface.0] {
            self.builder.push(BackendOp::PushTransform(*t));
        }

        // TODO: overflow (scroll)
        // TODO: opacity
        // TODO: border_radius (clip downwards, (border/shadow only on this level))

        for s in &self.ui_state.outline_shadows[surface.0] {
            self.render_outline_shadow(s);
        }

        if let Some(o) = &self.ui_state.outlines[surface.0] {
            self.render_outline(o);
        }

        // TODO: clip if Overflow::Hidden
        // (should be after outline)

        self.render_background_color(self.ui_state.background_colors[surface.0]);

        for b in &self.ui_state.background_images[surface.0] {
            self.render_background_image(b);
        }

        for s in &self.ui_state.inset_shadows[surface.0] {
            self.render_inset_shadow(s);
        }

        if let Some(content) = &self.ui_state.contents[surface.0] {
            match content {
                RenderContent::Children(children) => self.render_children(children),
                RenderContent::Text(layer) => self.render_text(*layer),
            }
        }

        if let Some(b) = &self.ui_state.borders[surface.0] {
            self.render_border(b);
        }

        if self.ui_state.transforms[surface.0].is_some() {
            self.builder.push(BackendOp::PopTransform);
        }
    }

    fn render_outline_shadow(&mut self, shadow: &OutlineShadow) {
        if shadow.blur != 0. {
            println!("TODO: OutlineShadow blur");
        }

        self.builder
            .push(BackendOp::PushRect(self.current_bounds.inflate_uniform(shadow.spread), FillStyle::SolidColor(shadow.color)));
    }

    fn render_outline(&mut self, outline: &Outline) {
        let Outline { width, color, .. } = *outline;
        let Bounds { a, b } = self.current_bounds;

        // top
        self.builder.push(BackendOp::PushRect(
            Bounds {
                a,
                b: Pos { x: b.x + width, y: a.y - width },
            },
            FillStyle::SolidColor(color),
        ));

        // right
        self.builder.push(BackendOp::PushRect(
            Bounds {
                a: Pos { x: b.x + width, y: a.y },
                b: Pos { x: b.x, y: b.y + width },
            },
            FillStyle::SolidColor(color),
        ));

        // bottom
        self.builder.push(BackendOp::PushRect(
            Bounds {
                a: Pos { x: a.x - width, y: b.y + width },
                b,
            },
            FillStyle::SolidColor(color),
        ));

        // left
        self.builder.push(BackendOp::PushRect(
            Bounds {
                a: Pos { x: a.x - width, y: a.y - width },
                b: Pos { x: a.x, y: b.y },
            },
            FillStyle::SolidColor(color),
        ));
    }

    fn render_background_color(&mut self, color: Color) {
        if color.a != 0 {
            self.builder.push(BackendOp::PushRect(self.current_bounds, FillStyle::SolidColor(color)));
        }
    }

    fn render_background_image(&mut self, background_image: &BackgroundImage) {
        match background_image {
            BackgroundImage::Image { image } => self.builder.push(BackendOp::PushRect(
                self.current_bounds,
                FillStyle::Texture(self.ui_state.textures[image.0], Bounds { a: Pos::ZERO, b: Pos::ONE }),
            )),
            BackgroundImage::LinearGradient {} => println!("TODO: render linear gradient"),
            BackgroundImage::RadialGradient {} => println!("TODO: render radial gradient"),
        }
    }

    fn render_inset_shadow(&mut self, shadow: &InsetShadow) {
        println!("TODO: render_inset_shadow");
    }

    fn render_children(&mut self, children: &[SurfaceId]) {
        for ch in children {
            let prev_bounds = self.current_bounds;

            self.current_bounds = self.bounds.lookup(self.ui_state.bounds_keys[ch.0]).translate(prev_bounds.a);
            self.render_surface(*ch);

            self.current_bounds = prev_bounds;
        }
    }

    fn render_text(&mut self, text_layer: RB::LayerId) {
        self.builder.push(BackendOp::PushLayer(text_layer));
    }

    //fn render_text_shadow(&mut self) {}

    fn render_border(&mut self, border: &Border) {
        // note the border is always inside (it acts like padding in layout)

        // TODO: border_radius

        // TODO: corners (overdraw will be visible with alpha colors)
        // TODO: different edge colors (push_triangle)

        let Bounds { a, b } = self.current_bounds;

        if let Some(BorderSide { width, style, color }) = border.top {
            if style == BorderStyle::Solid {
                self.builder.push(BackendOp::PushRect(Bounds { a, b: Pos { x: b.x, y: a.y + width } }, FillStyle::SolidColor(color)))
            }
        }

        if let Some(BorderSide { width, style, color }) = border.right {
            if style == BorderStyle::Solid {
                self.builder.push(BackendOp::PushRect(Bounds { a: Pos { x: b.x - width, y: a.y }, b }, FillStyle::SolidColor(color)))
            }
        }

        if let Some(BorderSide { width, style, color }) = border.bottom {
            if style == BorderStyle::Solid {
                self.builder.push(BackendOp::PushRect(Bounds { a: Pos { x: a.x, y: b.y - width }, b }, FillStyle::SolidColor(color)))
            }
        }

        if let Some(BorderSide { width, style, color }) = border.left {
            if style == BorderStyle::Solid {
                self.builder.push(BackendOp::PushRect(Bounds { a, b: Pos { x: a.x + width, y: b.y } }, FillStyle::SolidColor(color)))
            }
        }
    }
}

enum RenderContent<RB: RenderBackend> {
    Children(Vec<SurfaceId>),
    Text(RB::LayerId),
}

#[cfg(test)]
mod tests {
    use super::backend::BackendOp;
    use super::*;

    #[test]
    fn empty_surface() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.render_surface(c, &vec![Bounds::ZERO]);

        assert_eq!(r.backend.log, vec!["render"]);
    }

    #[test]
    fn outline() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLUE,
            }),
        );
        r.render_surface(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(
            r.backend.log,
            vec![
                "render",
                "PushRect(Bounds((0.0, 0.0), (101.0, -1.0)), SolidColor(#0000ff))",
                "PushRect(Bounds((101.0, 0.0), (100.0, 101.0)), SolidColor(#0000ff))",
                "PushRect(Bounds((-1.0, 101.0), (100.0, 100.0)), SolidColor(#0000ff))",
                "PushRect(Bounds((-1.0, -1.0), (0.0, 100.0)), SolidColor(#0000ff))"
            ]
        );
    }

    #[test]
    fn background_color() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_background_color(c, Color::GREEN);
        r.render_surface(
            c,
            &vec![Bounds {
                a: Pos::ZERO,
                b: Pos { x: 100., y: 100. },
            }],
        );

        assert_eq!(r.backend.log, vec!["render", "PushRect(Bounds((0.0, 0.0), (100.0, 100.0)), SolidColor(#00ff00))"]);
    }

    #[test]
    fn children() {
        let mut r = create_test_renderer();
        let parent = r.create_surface(0);
        let child = r.create_surface(1);

        r.set_background_color(child, Color { r: 255, g: 0, b: 0, a: 255 });
        r.set_children(parent, &[child]);

        r.render_surface(
            parent,
            &vec![
                Bounds {
                    a: Pos::ZERO,
                    b: Pos { x: 100., y: 100. },
                },
                Bounds {
                    a: Pos { x: 50., y: 50. },
                    b: Pos { x: 150., y: 150. },
                },
            ],
        );

        assert_eq!(r.backend.log, vec!["render", "PushRect(Bounds((50.0, 50.0), (150.0, 150.0)), SolidColor(#ff0000))"]);
    }

    #[test]
    fn it_works() {
        let mut r = create_test_renderer();
        let c = r.create_surface(0);

        r.set_overflow(c, Overflow::Visible);
        r.set_opacity(c, 0.5);
        r.set_border_radius(
            c,
            Some(BorderRadius {
                top_left: 5.,
                top_right: 5.,
                bottom_right: 5.,
                bottom_left: 5.,
            }),
        );
        r.set_outline_shadows(
            c,
            vec![OutlineShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_outline(
            c,
            Some(Outline {
                width: 1.,
                style: OutlineStyle::Solid,
                color: Color::BLACK,
            }),
        );
        r.set_background_color(c, Color::BLACK);
        r.set_inset_shadows(
            c,
            vec![InsetShadow {
                offset: Pos::ZERO,
                blur: 5.,
                spread: 5.,
                color: Color::BLACK,
            }],
        );
        r.set_color(c, Color::BLACK);
        r.set_border(
            c,
            Some(Border {
                top: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::RED,
                }),
                right: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::GREEN,
                }),
                bottom: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::BLUE,
                }),
                left: Some(BorderSide {
                    width: 1.,
                    style: BorderStyle::Solid,
                    color: Color::YELLOW,
                }),
            }),
        );

        r.render_surface(c, &vec![Bounds::ZERO]);

        assert_eq!(
            r.backend.log,
            vec![
                "render",
                "PushRect(Bounds((-5.0, -5.0), (5.0, 5.0)), SolidColor(#000000))",
                "PushRect(Bounds((0.0, 0.0), (1.0, -1.0)), SolidColor(#000000))",
                "PushRect(Bounds((1.0, 0.0), (0.0, 1.0)), SolidColor(#000000))",
                "PushRect(Bounds((-1.0, 1.0), (0.0, 0.0)), SolidColor(#000000))",
                "PushRect(Bounds((-1.0, -1.0), (0.0, 0.0)), SolidColor(#000000))",
                "PushRect(Bounds((0.0, 0.0), (0.0, 0.0)), SolidColor(#000000))",
                "PushRect(Bounds((0.0, 0.0), (0.0, 1.0)), SolidColor(#ff0000))",
                "PushRect(Bounds((-1.0, 0.0), (0.0, 0.0)), SolidColor(#00ff00))",
                "PushRect(Bounds((0.0, -1.0), (0.0, 0.0)), SolidColor(#0000ff))",
                "PushRect(Bounds((0.0, 0.0), (1.0, 0.0)), SolidColor(#ffff00))"
            ]
        );
    }

    fn create_test_renderer<BK: Copy>() -> Renderer<TestRenderBackend, BK> {
        Renderer::new(TestRenderBackend { log: Vec::new(), layers: Vec::new() })
    }

    #[derive(Debug)]
    struct TestRenderBackend {
        layers: Vec<Vec<BackendOp<Self>>>,
        log: Vec<String>,
    }

    impl RenderBackend for TestRenderBackend {
        type LayerId = usize;
        type TextureId = usize;

        fn resize(&mut self, width: f32, height: f32) {
            self.log.push(format!("resize {:?} {:?}", width, height));
        }

        fn render(&mut self, ops: impl Iterator<Item = BackendOp<Self>>) {
            self.log.push("render".to_string());

            for op in ops {
                self.log.push(format!("{:?}", op));
            }
        }

        fn create_layer(&mut self) -> Self::LayerId {
            let id = self.log.len();

            self.log.push(format!("create_layer -> {:?}:", &id));

            id
        }

        fn update_layer(&mut self, layer: Self::LayerId, ops: impl Iterator<Item = BackendOp<Self>>) {
            self.log.push(format!("update_layer {:?}", &layer));

            for op in ops {
                self.log.push(format!("{:?}", op));
            }
        }

        fn create_texture(&mut self, width: i32, height: i32) -> Self::TextureId {
            let id = self.log.len();

            self.log.push(format!("create_texture {:?} {:?} -> {:?}", width, height, id));

            id
        }

        fn update_texture(&mut self, texture: Self::TextureId, _data: &[u8]) {
            self.log.push(format!("update_texture {:?}", texture));
        }
    }
}
