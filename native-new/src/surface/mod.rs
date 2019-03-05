pub use crate::generated::{BorderRadius, Border, BorderSide, BorderStyle, BoxShadow, Color, Image, Text};
use crate::Id;
use crate::storage::{DenseStorage, SparseStorage};
use std::fmt::Debug;
use std::fmt::Formatter;

/// In a perfect world this could be just:
///
/// class Surface {
///   children = []
///
///   padding, margin, ...
///
///   boxShadow, backgroundColor, ...
/// }
///
/// And then we could:
///
/// let computedLayouts = layoutFn(rootSurface)
/// renderFn(rootSurface, computedLayouts)
///
/// And this is basically an attempt to implement a whole tree of surfaces as a "struct of arrays"
/// which should make rendering way-faster (by being cpu cache-friendly)
///
/// Note that SurfaceData accessors reduce coupling on this internal structure
pub struct SurfaceService {
    children: DenseStorage<Id, Vec<Id>>,
    border_radii: SparseStorage<Id, BorderRadius>,
    box_shadows: SparseStorage<Id, BoxShadow>,
    background_colors: SparseStorage<Id, Color>,
    texts: SparseStorage<Id, Text>,
    images: SparseStorage<Id, Image>,
    borders: SparseStorage<Id, Border>,
}

impl SurfaceService {
    pub fn new() -> Self {
        SurfaceService {
            children: DenseStorage::new(),
            border_radii: SparseStorage::new(),
            box_shadows: SparseStorage::new(),
            background_colors: SparseStorage::new(),
            texts: SparseStorage::new(),
            images: SparseStorage::new(),
            borders: SparseStorage::new(),
        }
    }

    pub fn alloc(&mut self) {
        self.children.push(vec![]);
    }

    pub fn append_child(&mut self, parent: Id, child: Id) {
        self.children.get_mut(parent).push(child);
    }

    pub fn insert_before(&mut self, parent: Id, child: Id, before: Id) {
        let index = self.index_of(parent, before);
        self.children.get_mut(parent).insert(index, child);
    }

    pub fn remove_child(&mut self, parent: Id, child: Id) {
        let index = self.index_of(parent, child);
        self.children.get_mut(parent).remove(index);
    }

    fn index_of(&self, parent: Id, child: Id) -> usize {
        self.children.get(parent)
            .iter()
            .position(|id| *id == child)
            .expect("not found")
    }

    // layout props are currently handled by YogaLayoutService
    // ideally we would store them here and just pass them to LayoutService but that's not
    // how yoga works

    pub fn set_border_radius(&mut self, surface: Id, border_radius: Option<BorderRadius>) {
        self.border_radii.set(surface, border_radius);
    }

    pub fn set_box_shadow(&mut self, surface: Id, box_shadow: Option<BoxShadow>) {
        self.box_shadows.set(surface, box_shadow);
    }

    pub fn set_background_color(&mut self, surface: Id, color: Option<Color>) {
        self.background_colors.set(surface, color);
    }

    pub fn set_image(&mut self, surface: Id, image: Option<Image>) {
        self.images.set(surface, image);
    }

    pub fn set_text(&mut self, surface: Id, text: Option<Text>) {
        self.texts.set(surface, text);
    }

    pub fn set_border(&mut self, surface: Id, border: Option<Border>) {
        self.borders.set(surface, border);
    }

    pub fn get_surface_data(&self, surface: Id) -> SurfaceData {
        SurfaceData {
            svc: self,
            id: surface
        }
    }
}

pub struct SurfaceData<'a> {
    svc: &'a SurfaceService,
    id: Id
}

impl <'a> SurfaceData<'a> {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn border_radius(&self) -> Option<&'a BorderRadius> {
        self.svc.border_radii.get(self.id)
    }

    pub fn box_shadow(&self) -> Option<&'a BoxShadow> {
        self.svc.box_shadows.get(self.id)
    }

    pub fn background_color(&self) -> Option<&'a Color> {
        self.svc.background_colors.get(self.id)
    }

    pub fn image(&self) -> Option<&'a Image> {
        self.svc.images.get(self.id)
    }

    pub fn text(&self) -> Option<&'a Text> {
        self.svc.texts.get(self.id)
    }

    pub fn border(&self) -> Option<&'a Border> {
        self.svc.borders.get(self.id)
    }

    pub fn children(&'a self) -> impl Iterator<Item=SurfaceData<'a>> {
        self.svc.children.get(self.id).iter().map(move |child_id| self.svc.get_surface_data(*child_id))
    }
}

impl <'a> Debug for SurfaceData<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let children: Vec<SurfaceData> = self.children().map(|ch| ch).collect();

        write!(f, "#{} ", self.id())?;

        if let Some(radius) = self.border_radius() {
            write!(f, "BorderRadius{:?} ", (radius.0, radius.1, radius.2, radius.3))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc() -> SurfaceService {
        SurfaceService::new()
    }

    #[test]
    fn test_children() {
        let parent = 0;

        let mut svc = test_svc();
        svc.children.push(vec![1, 2]);

        svc.append_child(parent, 3);
        let expected: Vec<Id> = vec![1, 2, 3];
        assert_eq!(svc.children.get(0), &expected);

        svc.remove_child(0, 1);
        let expected: Vec<Id> = vec![2, 3];
        assert_eq!(svc.children.get(0), &expected);

        svc.insert_before(0, 1, 3);
        let expected: Vec<Id> = vec![2, 1, 3];
        assert_eq!(svc.children.get(0), &expected);
    }

    #[test]
    fn test_surface_data() {
        let mut svc = test_svc();
        let data = svc.get_surface_data(0);

        assert!(data.box_shadow().is_none());

        data.children();
    }
}
