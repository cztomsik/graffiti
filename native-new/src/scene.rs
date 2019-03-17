pub use crate::api::{
    Border, BorderRadius, BorderSide, BorderStyle, BoxShadow, Color, Flex, Flow, Image, Rect,
    Size, Text, SurfaceId
};
use crate::storage::{DenseStorage, SparseStorage};
use std::fmt::Debug;
use std::fmt::Formatter;

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
///     1. object graphs are complicated to do in rust
///     2. most of the data will be sparse which is not good for performance
///
/// And this is basically an attempt to implement a whole tree of surfaces (rootNode) as a
/// "struct of arrays" which should make rendering way-faster (by being cpu cache-friendly)
///
/// Note that SurfaceData accessors reduce coupling on this internal structure
pub struct Scene {
    children: DenseStorage<SurfaceId, Vec<SurfaceId>>,
    border_radii: SparseStorage<SurfaceId, BorderRadius>,
    box_shadows: SparseStorage<SurfaceId, BoxShadow>,
    background_colors: SparseStorage<SurfaceId, Color>,
    texts: SparseStorage<SurfaceId, Text>,
    images: SparseStorage<SurfaceId, Image>,
    borders: SparseStorage<SurfaceId, Border>,
}

// this is internal module, used by window, NOT an implementation of api::SceneUpdateContext,
// it's just coincidental similarity
impl Scene {
    pub fn new() -> Self {
        Scene {
            children: DenseStorage::new(),
            border_radii: SparseStorage::new(),
            box_shadows: SparseStorage::new(),
            background_colors: SparseStorage::new(),
            texts: SparseStorage::new(),
            images: SparseStorage::new(),
            borders: SparseStorage::new(),
        }
    }

    pub fn create_surface(&mut self) -> SurfaceId {
        self.children.insert(vec![]) as u16
    }

    pub fn append_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        self.children.get_mut(parent).push(child);
    }

    pub fn insert_before(&mut self, parent: SurfaceId, child: SurfaceId, before: SurfaceId) {
        let index = self.index_of(parent, before);
        self.children.get_mut(parent).insert(index, child);
    }

    pub fn remove_child(&mut self, parent: SurfaceId, child: SurfaceId) {
        let index = self.index_of(parent, child);
        self.children.get_mut(parent).remove(index);
    }

    // layout props are currently handled by YogaLayoutService
    // ideally we would store them here and just pass them to LayoutService but that's not
    // how yoga works

    pub fn set_border_radius(&mut self, surface: SurfaceId, border_radius: Option<BorderRadius>) {
        self.border_radii.set(surface, border_radius);
    }

    pub fn set_box_shadow(&mut self, surface: SurfaceId, box_shadow: Option<BoxShadow>) {
        self.box_shadows.set(surface, box_shadow);
    }

    pub fn set_background_color(&mut self, surface: SurfaceId, color: Option<Color>) {
        self.background_colors.set(surface, color);
    }

    pub fn set_image(&mut self, surface: SurfaceId, image: Option<Image>) {
        self.images.set(surface, image);
    }

    pub fn set_text(&mut self, surface: SurfaceId, text: Option<Text>) {
        self.texts.set(surface, text);
    }

    pub fn set_border(&mut self, surface: SurfaceId, border: Option<Border>) {
        self.borders.set(surface, border);
    }

    fn index_of(&self, parent: SurfaceId, child: SurfaceId) -> usize {
        self.children
            .get(parent)
            .iter()
            .position(|id| *id == child)
            .expect("not found")
    }

    pub fn get_surface_data(&self, surface: SurfaceId) -> SurfaceData {
        SurfaceData {
            scene: self,
            id: surface,
        }
    }
}

pub struct SurfaceData<'a> {
    scene: &'a Scene,
    id: SurfaceId,
}

impl<'a> SurfaceData<'a> {
    pub fn id(&self) -> SurfaceId {
        self.id
    }

    pub fn border_radius(&self) -> Option<&'a BorderRadius> {
        self.scene.border_radii.get(self.id)
    }

    pub fn box_shadow(&self) -> Option<&'a BoxShadow> {
        self.scene.box_shadows.get(self.id)
    }

    pub fn background_color(&self) -> Option<&'a Color> {
        self.scene.background_colors.get(self.id)
    }

    pub fn image(&self) -> Option<&'a Image> {
        self.scene.images.get(self.id)
    }

    pub fn text(&self) -> Option<&'a Text> {
        self.scene.texts.get(self.id)
    }

    pub fn border(&self) -> Option<&'a Border> {
        self.scene.borders.get(self.id)
    }

    pub fn children(&'a self) -> impl Iterator<Item = SurfaceData<'a>> {
        self.scene
            .children
            .get(self.id)
            .iter()
            .map(move |child_id| self.scene.get_surface_data(*child_id))
    }
}

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
