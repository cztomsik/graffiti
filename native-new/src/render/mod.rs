use crate::generated::SurfaceId;
use crate::SceneListener;

pub trait Renderer: SceneListener {
    fn hit_test(&self, pos: (f32, f32)) -> SurfaceId;

    fn scroll(&mut self, pos: (f32, f32), delta: (f32, f32));
}

mod webrender;
pub use self::webrender::WebrenderRenderer;
