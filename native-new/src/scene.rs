pub use crate::api::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Flex, Flow, Image,
    Size, Text, SurfaceId, Dimension, Dimensions, Scene
};
use crate::layout::{LayoutTree, YogaTree};
use crate::api::Rect;
use std::collections::BTreeMap;
use crate::storage::Storage;
use crate::text::LaidText;

/// A tree of surfaces (UI elements) along with all of their layout/visual properties
///
/// In a perfect world this could be just (js pseudo code):
///
/// class Surface {
///   children = []
///   padding, margin, ...
///   boxShadow, backgroundColor, ...
/// }
///
/// And then we could:
///
/// rootSurface = new Surface()
/// rootSurface.padding = ...
/// rootSurface.children[0].backgroundColor = ...
///
/// let computedLayouts = layoutFn(rootSurface)
/// renderFn(rootSurface, computedLayouts)
///
/// BUT
///     1. object graphs are complicated to do in safe rust (borrowing)
///        (scene.get_surface_mut(id).append_child(scene.get_surface(id)))
///
///     2. most of the data will be sparse which is not good for performance
///
///     3. we often want to react on changes
///
/// This is an attempt to implement a whole tree of surfaces (rootNode) as a
/// "struct of arrays" which should make rendering way-faster (by being cpu cache-friendly)
pub struct ArrayScene {
    // TODO: add vec of presence bitflags, so that we can quickly detect if surface has a border/shadow/... or not
    // 1 cache line could speed up 64 surfaces
    children: Vec<Vec<SurfaceId>>,
    border_radii: BTreeMap<SurfaceId, BorderRadius>,
    box_shadows: BTreeMap<SurfaceId, BoxShadow>,
    background_colors: BTreeMap<SurfaceId, Color>,
    texts: BTreeMap<SurfaceId, Text>,
    images: BTreeMap<SurfaceId, Image>,
    borders: BTreeMap<SurfaceId, Border>,
    layout_tree: YogaTree
}

impl ArrayScene {
    pub fn new() -> Self {
        let mut scene = ArrayScene {
            children: vec![],
            border_radii: BTreeMap::new(),
            box_shadows: BTreeMap::new(),
            background_colors: BTreeMap::new(),
            texts: BTreeMap::new(),
            images: BTreeMap::new(),
            borders: BTreeMap::new(),
            layout_tree: YogaTree::new()
        };

        // root
        scene.create_surface();

        scene
    }

    pub fn set_layout_size(&mut self, width: f32, height: f32) {
        self.layout_tree.set_size(
            0,
            Size(
                Dimension::Point(width),
                Dimension::Point(height),
            ),
        );
    }

    pub fn calculate_layout(&mut self) {
        self.layout_tree.calculate();
    }

    fn index_of(&self, parent: SurfaceId, child: SurfaceId) -> usize {
        self.children[parent]
            .iter()
            .position(|id| *id == child)
            .expect("child not found")
    }
}

impl Scene for ArrayScene {
    fn create_surface(&mut self) -> SurfaceId {
        let id = self.children.len();

        self.layout_tree.alloc();
        self.children.push(vec![]);

        id
    }

    fn children(&self, parent: SurfaceId) -> &[SurfaceId] {
        &self.children[parent]
    }

    fn append_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        self.layout_tree.append_child(parent, child);
        self.children[parent].push(child);
    }

    fn insert_before(&mut self, parent: SurfaceId, child: SurfaceId, before: SurfaceId) {
        let index = self.index_of(parent, before);
        self.children[parent].insert(index, child);
        self.layout_tree.insert_at(parent, child, index as u32);
    }

    fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        let index = self.index_of(parent, child);
        self.children[parent].remove(index);
        self.layout_tree.remove_child(parent, child);
    }

    fn set_size(&mut self, surface: SurfaceId, size: Size) {
        self.layout_tree.set_size(surface, size);
    }

    fn set_flex(&mut self, surface: SurfaceId, flex: Flex) {
        self.layout_tree.set_flex(surface, flex);
    }

    fn set_flow(&mut self, surface: SurfaceId, flow: Flow) {
        self.layout_tree.set_flow(surface, flow);
    }

    fn set_padding(&mut self, surface: SurfaceId, padding: Dimensions) {
        self.layout_tree.set_padding(surface, padding);
    }

    fn set_margin(&mut self, surface: SurfaceId, margin: Dimensions) {
        self.layout_tree.set_margin(surface, margin);
    }

    fn computed_layout(&self, surface: SurfaceId) -> Rect {
        self.layout_tree.computed_layout(surface)
    }

    fn text_layout(&self, surface: SurfaceId) -> LaidText {
        self.layout_tree.text_layout(surface)
    }

    fn border_radius(&self, surface: SurfaceId) -> Option<&BorderRadius> {
        self.border_radii.get(&surface)
    }

    fn set_border_radius(&mut self, surface: SurfaceId, border_radius: Option<BorderRadius>) {
        self.border_radii.set(surface, border_radius);
    }

    fn box_shadow(&self, surface: SurfaceId) -> Option<&BoxShadow> {
        self.box_shadows.get(&surface)
    }

    fn set_box_shadow(&mut self, surface: SurfaceId, box_shadow: Option<BoxShadow>) {
        self.box_shadows.set(surface, box_shadow);
    }

    fn background_color(&self, surface: SurfaceId) -> Option<&Color> {
        self.background_colors.get(&surface)
    }

    fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>) {
        self.background_colors.set(surface, color);
    }

    fn image(&self, surface: SurfaceId) -> Option<&Image> {
        self.images.get(&surface)
    }

    fn set_image(&mut self, surface: SurfaceId, image: Option<Image>) {
        self.images.set(surface, image);
    }

    fn text(&self, surface: SurfaceId) -> Option<&Text> {
        self.texts.get(&surface)
    }

    fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        self.texts.set(surface, text.clone());
        self.layout_tree.set_text(surface, text);
    }

    fn border(&self, surface: SurfaceId) -> Option<&Border> {
        self.borders.get(&surface)
    }

    fn set_border(&mut self, surface: SurfaceId, border: Option<Border>) {
        self.borders.set(surface, border);
        // TODO: layout_service.set_border
    }
}

// TODO
/*
impl<'a> Debug for SurfaceData<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let children: Vec<SurfaceData> = self.children().map(|ch| ch).collect();

        write!(f, "#{} ", self.id())?;

        if let Some(radius) = self.border_radius() {
            write!(
                f,
                "BorderRadius{:?} ",
                (radius.0, radius.1, radius.2, radius.3)
            )?;
        }

        if let Some(text) = self.text() {
            write!(f, "Text({}) ", text.text)?;
        }

        if let Some(image) = self.image() {
            write!(f, "Img({}) ", image.url)?;
        }

        if let Some(color) = self.background_color() {
            write!(f, "BgColor({:02x}{:02x}{:02x}) ", color.0, color.1, color.2)?;
        }

        if !children.is_empty() {
            children.fmt(f)?
        }

        Ok(())
    }
}

*/

/*
#[cfg(test)]
mod tests {
    use super::*;

    type Id = SurfaceId;

    fn test_scene() -> Scene {
        Scene::new()
    }

    #[test]
    fn test_children() {
        let parent = 0;

        let mut scene = test_scene();
        scene.children.push(vec![1, 2]);

        scene.append_child(parent, 3);
        let expected: Vec<Id> = vec![1, 2, 3];
        assert_eq!(scene.children.get(0), &expected);

        scene.remove_child(0, 1);
        let expected: Vec<Id> = vec![2, 3];
        assert_eq!(scene.children.get(0), &expected);

        scene.insert_before(0, 1, 3);
        let expected: Vec<Id> = vec![2, 1, 3];
        assert_eq!(scene.children.get(0), &expected);
    }

    #[test]
    fn test_surface_data() {
        let mut scene = test_scene();
        let data = scene.get_surface_data(0);

        assert!(data.box_shadow().is_none());

        data.children();
    }
}
*/
