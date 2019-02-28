pub use crate::generated::{Border, BoxShadow, Color, Image, Text};
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
    box_shadows: BTreeMap<Id, BoxShadow>,
    background_colors: BTreeMap<Id, Color>,
    texts: BTreeMap<Id, Text>,
    images: BTreeMap<Id, Image>,
    borders: BTreeMap<Id, Border>,
}

impl SurfaceService {
    pub fn new() -> Self {
        SurfaceService {
            children: vec![],
            box_shadows: BTreeMap::new(),
            background_colors: BTreeMap::new(),
            texts: BTreeMap::new(),
            images: BTreeMap::new(),
            borders: BTreeMap::new(),
        }
    }

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

    pub fn set_box_shadow(&mut self, surface: Id, box_shadow: Option<BoxShadow>) {}

    pub fn set_background_color(&mut self, surface: Id, color: Option<Color>) {
        self.background_colors.set_opt(surface, color);
    }

    pub fn set_image(&mut self, surface: Id, image: Option<Image>) {}
    pub fn set_text(&mut self, surface: Id, text: Option<Text>) {}
    pub fn set_border(&mut self, surface: Id, border: Option<Border>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_svc() -> SurfaceService {
        SurfaceService::new()
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

trait SetOpt<K, V> {
    fn set_opt(&mut self, key: K, opt_value: Option<V>);
}

impl <K, V> SetOpt<K, V> for BTreeMap<K, V>
where K: Ord
{
    fn set_opt(&mut self, key: K, opt_value: Option<V>) {
        if let Some(value) = opt_value {
            self.insert(key, value);
        } else {
            self.remove(&key);
        }
    }
}
