use super::{BackendOp, Color, FillStyle, RenderBackend};
use crate::commons::Mat3;
use raqote::*;

// temporary backend just to test the renderer works properly
// might be a thing in future but now it just writes PNG file

pub struct RaqoteBackend {
    out_file: String,
    dt: DrawTarget,
    layers: Vec<Vec<BackendOp<Self>>>,
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

impl std::fmt::Debug for RaqoteBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("RaqoteBackend").field(&self.out_file).finish()
    }
}

impl RenderBackend for RaqoteBackend {
    type LayerId = usize;
    type TextureId = usize;

    fn resize(&mut self, width: f32, height: f32) {
        self.dt = DrawTarget::new(width as i32, height as i32);
    }

    fn render(&mut self, ops: impl Iterator<Item = BackendOp<Self>>) {
        //self.dt.clear(Color::BLACK.into());

        let mut transform_stack = Vec::new();

        ops.for_each(|op| render_op(&op, &self.layers, &self.textures, &mut self.dt, &mut transform_stack));

        // TODO: render
        //let _data = self.dt.get_data();

        self.dt.write_png(&self.out_file).unwrap();
    }

    fn create_layer(&mut self) -> Self::LayerId {
        self.layers.push(Vec::new());

        self.layers.len() - 1
    }

    fn update_layer(&mut self, layer: Self::LayerId, ops: impl Iterator<Item = BackendOp<Self>>) {
        self.layers[layer] = ops.collect();
    }

    fn create_texture(&mut self, width: i32, height: i32) -> Self::TextureId {
        let data = vec![0; (width * height * 4) as usize].into_boxed_slice();

        self.textures.push(Texture { width, height, data });

        self.textures.len() - 1
    }

    fn update_texture(&mut self, texture: Self::TextureId, data: &[u8]) {
        &mut self.textures[texture].data[..].copy_from_slice(data);
    }
}

fn render_op(op: &BackendOp<RaqoteBackend>, layers: &[Vec<BackendOp<RaqoteBackend>>], textures: &[Texture], dt: &mut DrawTarget, transform_stack: &mut Vec<Transform>) {
    silly!("BackendOp::{:?}", op);

    match op {
        BackendOp::PushTransform(m) => {
            transform_stack.push(*dt.get_transform());
            dt.set_transform(&(*m).into());
        }

        BackendOp::PopTransform => dt.set_transform(&transform_stack.pop().unwrap()),

        BackendOp::PushRect(bounds, style) => {
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
                    let transform =
                        Transform::create_translation((uv.a.x * w) - bounds.a.x, (uv.a.y * h) - bounds.a.y).post_scale((uv.width() * w) / bounds.width(), (uv.height() * h) / bounds.height());

                    Source::Image(Image { width, height, data }, ExtendMode::Pad, FilterMode::Nearest, transform)
                }

                FillStyle::Msdf { color, .. } => {
                    Source::Solid((*color).into())

                    //panic!("TODO: msdf")
                }
            };

            dt.fill(&path, &source, &DrawOptions::new());
        }

        BackendOp::PushLayer(id) => {
            for op in &layers[*id] {
                render_op(op, layers, textures, dt, transform_stack);
            }
        }
    }
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

impl Into<Transform> for Mat3 {
    fn into(self) -> Transform {
        let m = self.0;

        Transform::row_major(m[0], m[1], m[3], m[4], m[6], m[7])
    }
}
