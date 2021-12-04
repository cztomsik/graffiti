use super::{TexData, UvRect};
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
                // Box::new([V; size_expr]) doesn't work and const generics allocate on the stack
                // so not only it has to be copied but it can also blow the stack
                // so it's either this or unsafe {}, and it seems compiler can optimize most of this away
                pixels: vec![[255, 255, 255, 255]; (width * height) as _].into_boxed_slice(),
            },
        }
    }

    pub fn tex_data(&self) -> &TexData {
        &self.tex_data
    }

    pub fn push(&mut self, width: i32, height: i32, f: impl FnOnce(&mut TexData, usize, usize)) -> Option<UvRect> {
        let rect = self.packer.pack(width, height, false)?;

        f(&mut self.tex_data, rect.x as _, rect.y as _);

        Some(UvRect::new([rect.x as _, rect.y as _], [rect.right() as _, rect.bottom() as _]))
    }
}
