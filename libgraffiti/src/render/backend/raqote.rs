use super::{Color, FillStyle, LayerBuilder, RenderBackend};
use crate::commons::{Bounds, Pos};
use raqote::*;

// temporary backend just to test the renderer works properly
// might be a thing in future but now it just writes PNG file

pub struct RaqoteBackend {
    out_file: String,
    dt: DrawTarget,
    layers: Vec<Vec<RenderOp>>,
    textures: Vec<Texture>,
}

impl RaqoteBackend {
    pub fn new(out_file: String) -> Self {
        Self {
            out_file,
            dt: DrawTarget::new(0, 0),
            layers: Vec::new(),
            textures: Vec::new(),
        }
    }
}

impl RenderBackend for RaqoteBackend {
    type LayerId = usize;
    type TextureId = usize;
    type LayerBuilder = Vec<RenderOp>;

    fn resize(&mut self, width: f32, height: f32) {
        self.dt = DrawTarget::new(width as i32, height as i32);
    }

    fn create_layer(&mut self) -> Self::LayerId {
        self.layers.push(Vec::new());

        self.layers.len() - 1
    }

    fn rebuild_layer_with(&mut self, layer: Self::LayerId, mut f: impl FnMut(&mut Self::LayerBuilder)) {
        self.layers[layer].clear();
        f(&mut self.layers[layer]);
    }

    fn render_layer(&mut self, layer: Self::LayerId) {
        //self.dt.clear(Color::BLACK.into());

        render_op(&RenderOp::Layer(layer, Pos::ZERO), &self.layers, &self.textures, &mut self.dt);

        // TODO: render
        //let _data = self.dt.get_data();

        self.dt.write_png(&self.out_file).unwrap();
    }

    fn create_texture(&mut self, width: i32, height: i32, data: Box<[u8]>) -> Self::TextureId {
        assert_eq!(data.len() as i32, width * height * 4, "invalid texture data len");
        self.textures.push(Texture { width, height, data });

        self.textures.len() - 1
    }

    fn update_texture(&mut self, texture: Self::TextureId, mut f: impl FnMut(&mut [u8])) {
        f(&mut self.textures[texture].data);
    }
}

impl LayerBuilder<RaqoteBackend> for Vec<RenderOp> {
    fn push_rect(&mut self, bounds: Bounds, style: FillStyle<RaqoteBackend>) {
        self.push(RenderOp::FillRect(bounds, style));
    }

    fn push_layer(&mut self, layer: <RaqoteBackend as RenderBackend>::LayerId, origin: Pos) {
        self.push(RenderOp::Layer(layer, origin));
    }
}

fn render_op(op: &RenderOp, layers: &[Vec<RenderOp>], textures: &[Texture], dt: &mut DrawTarget) {
    match op {
        RenderOp::FillRect(bounds, style) => {
            let path = {
                let mut pb = PathBuilder::new();
                pb.rect(bounds.a.x, bounds.a.y, bounds.width(), bounds.height());
                pb.close();
                pb.finish()
            };

            // fill style
            let source = match style {
                FillStyle::SolidColor(color) => Source::Solid((*color).into()),

                FillStyle::Texture(texture, uv) => {
                    let Texture { width, height, ref data } = textures[*texture];
                    let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) };

                    let (w, h) = (width as f32, height as f32);
                    let transform = Transform::create_translation((uv.a.x * w) - bounds.a.x, (uv.a.y * h) - bounds.a.y)
                        .post_scale((uv.width() * w) / bounds.width(), (uv.height() * h) / bounds.height());

                    Source::Image(Image { width, height, data }, ExtendMode::Pad, FilterMode::Nearest, transform)
                }

                FillStyle::Msdf { .. } => panic!("TODO: msdf"),
            };

            dt.fill(&path, &source, &DrawOptions::new());
        }

        RenderOp::Layer(id, origin) => {
            let prev_transform = *dt.get_transform();

            dt.set_transform(&prev_transform.post_translate(Vector::new(origin.x, origin.y)));

            for op in &layers[*id] {
                render_op(op, layers, textures, dt);
            }

            dt.set_transform(&prev_transform);
        }
    }
}

pub enum RenderOp {
    FillRect(Bounds, FillStyle<RaqoteBackend>),
    Layer(<RaqoteBackend as RenderBackend>::LayerId, Pos),
}

pub enum Shape {
    Triangle(Pos, Pos, Pos),
    Rect(Bounds),
}

pub struct Texture {
    width: i32,
    height: i32,
    data: Box<[u8]>,
}

impl Into<SolidSource> for Color {
    fn into(self) -> SolidSource {
        SolidSource {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
