use super::{Vec2, RGBA8};

pub struct Atlas {
    pub(crate) tex_data: Vec<RGBA8>,
    pos: Vec2,
    row_height: f32
}

impl Atlas {
    pub fn new() -> Self {
        Self {
            tex_data: [[0, 0, 0, 0]; 1024 * 1024].into(),
            pos: Vec2::ZERO,
            row_height: 0.
        }
    }

    pub fn alloc(size: Vec2) -> () {
        // TODO: padding
        //const PADDING = 1;

        todo!()
    }
}
