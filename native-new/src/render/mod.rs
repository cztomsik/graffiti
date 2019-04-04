use crate::api::Scene;

pub trait SceneRenderer {
    fn render(&mut self, scene: &dyn Scene);
}

mod webrender;
pub use self::webrender::WebrenderRenderer;

//mod html;
//pub use self::html::HtmlRenderer;
