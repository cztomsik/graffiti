use crate::layout::ComputedLayout;
use crate::scene::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Image, Text, SurfaceData
};

mod webrender;
pub use self::webrender::WebrenderRenderService;

pub trait RenderService {
    fn render(&mut self, surface: &SurfaceData, computed_layouts: Vec<ComputedLayout>);
}
