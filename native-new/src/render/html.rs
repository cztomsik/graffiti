use crate::scene::SurfaceData;
use super::Renderer;

/// At first it doesn't make much sense because we are trying to replace HTML/DOM but it might be
/// very useful for debugging
///
/// Another very interesting use-case could be some automated visual regression testing.
///
/// It should be possible to switch layout on/off so that it's obvious if it works fine or not
pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&mut self, surface: &SurfaceData) {
        unimplemented!()
    }
}
