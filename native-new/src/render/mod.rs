use crate::layout::ComputedLayout;
use crate::scene::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Image, Text, SurfaceData
};

pub trait Renderer {
    fn render(&mut self, surface: &SurfaceData);
}

mod webrender;
pub use self::webrender::WebrenderRenderService;

//mod html;
//pub use self::html::HtmlRenderer;

pub trait RenderService {
    fn render(&mut self, surface: &SurfaceData, computed_layouts: Vec<ComputedLayout>);
}
