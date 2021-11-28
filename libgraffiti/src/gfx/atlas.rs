use super::{TexData, Vec2, AABB};
use rect_packer::Packer;

const PAD: i32 = 1;

pub struct Atlas {
    packer: Packer,
    tex_data: TexData,
}

impl Atlas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            packer: Packer::new(rect_packer::Config {
                width,
                height,
                border_padding: PAD,
                rectangle_padding: PAD,
            }),
            tex_data: TexData {
                width,
                height,
                pixels: vec![[0, 0, 0, 0]; (width * height) as _],
            },
        }
    }

    pub fn tex_data(&self) -> &TexData {
        &self.tex_data
    }

    // TODO: if we set TEXTURE_SIZE uniform, then vertex.uv can be just [u16; 2]
    //       because we can compute 0-1 in shader, which will save 4 bytes per vertex
    //       also, currently the texture is fixed to 1024x1024 so that would be super-easy
    pub fn push(&mut self, width: i32, height: i32, f: impl FnOnce(&mut TexData, usize, usize)) -> Option<AABB> {
        let rect = self.packer.pack(width, height, false)?;

        f(&mut self.tex_data, rect.x as _, rect.y as _);

        Some(AABB::new(
            Vec2::new(
                rect.x as f32 / self.tex_data.width as f32,
                rect.y as f32 / self.tex_data.height as f32,
            ),
            Vec2::new(
                rect.right() as f32 / self.tex_data.width as f32,
                rect.bottom() as f32 / self.tex_data.height as f32,
            ),
        ))
    }
}
