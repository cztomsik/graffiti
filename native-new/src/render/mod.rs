use crate::layout::ComputedLayout;
use crate::surface::{BorderRadius, Border, BorderSide, BorderStyle, BoxShadow, Color, Image, Text};

mod webrender;
pub use self::webrender::WebrenderRenderService;
use crate::surface::SurfaceData;

pub trait RenderService {
    fn render(&mut self, surface: &SurfaceData, computed_layouts: Vec<ComputedLayout>);
}
