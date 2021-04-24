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

    pub fn push(&mut self, width: i32, height: i32, f: impl FnOnce(&mut TexData, usize, usize)) -> Option<AABB> {
        let rect = self.packer.pack(width + PAD, height + PAD, false)?;

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
