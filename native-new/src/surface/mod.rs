pub use crate::generated::BackgroundColor;
use crate::Id;
use std::collections::BTreeMap;

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
pub struct SurfaceService {
    children: Vec<Vec<Id>>,
    background_colors: BTreeMap<Id, Color>,
}

impl SurfaceService {
    pub fn append_child(&mut self, parent: Id, child: Id) {
        self.children[parent].push(child);
    }

    pub fn insert_before(&mut self, parent: Id, child: Id, before: Id) {
        let index = self.index_of(parent, before);
        self.children[parent].insert(index, child);
    }

    pub fn remove_child(&mut self, parent: Id, child: Id) {
        let index = self.index_of(parent, child);
        self.children[parent].remove(index);
    }

    fn index_of(&self, parent: Id, child: Id) -> usize {
        self.children[parent]
            .iter()
            .position(|id| *id == child)
            .expect("not found")
    }

    // layout props are currently handled by YogaLayoutService
    // ideally we would store them here and just pass them to LayoutService but that's not
    // how yoga works

    pub fn set_box_shadow(surface: Id, box_shadow: BoxShadow) {}
    pub fn set_background_color(surface: Id, color: Color) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc() -> SurfaceService {
        SurfaceService { children: vec![] }
    }

    #[test]
    fn test_children() {
        let mut svc = test_svc();
        svc.children = vec![vec![1, 2]];

        svc.append_child(0, 3);
        assert_eq!(svc.children[0], &vec![1, 2, 3][..]);

        svc.remove_child(0, 1);
        assert_eq!(svc.children[0], &vec![2, 3][..]);

        svc.insert_before(0, 1, 3);
        assert_eq!(svc.children[0], &vec![2, 1, 3][..]);
    }
}

type BoxShadow = ();
type Color = ();
