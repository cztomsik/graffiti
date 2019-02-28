use crate::surface::{BoxShadow, Color, Border, Text, Image};
use crate::layout::ComputedLayout;
use crate::Id;
use std::collections::BTreeMap;

mod webrender;
pub use crate::render::webrender::WebrenderRenderService;

pub trait RenderService {
    fn render(&mut self, id: Id, data: &Data);
}

pub struct Data<'render> {
    computed_layouts: &'render Vec<ComputedLayout>,
    border_radii: &'render BTreeMap<Id, f32>,
    box_shadows: &'render BTreeMap<Id, BoxShadow>,
    background_colors: &'render BTreeMap<Id, Color>,
    images: &'render BTreeMap<Id, Image>,
    selections: &'render BTreeMap<Id, ()>,
    texts: &'render BTreeMap<Id, Image>,
    children: &'render Vec<Vec<Id>>,
    borders: &'render BTreeMap<Id, Border>
}
