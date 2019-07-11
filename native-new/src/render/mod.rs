use crate::SceneListener;
use crate::generated::SurfaceId;

pub trait Renderer: SceneListener {
    fn render(&mut self, layout: &dyn Layout, text_layout: &dyn TextLayout);

    fn hit_test(&self, pos: (f32, f32)) -> SurfaceId;

    fn scroll(&mut self, pos: (f32, f32), delta: (f32, f32));
}

mod webrender;
pub use self::webrender::WebrenderRenderer;
use crate::layout::Layout;
use crate::text::TextLayout;
