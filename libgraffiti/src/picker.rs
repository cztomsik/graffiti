use crate::generated::SurfaceId;
use crate::commons::{Pos, Bounds};

/// Useful for events, to find the uppermost surface at given position
///
/// Note that the surface should receive event even if it has no own content
/// (container with padding but no background)
///
/// And because we don't support z-index we can just recursively traverse all
/// the children and find the last match (following siblings are always on top)
///
/// TODO: maybe building and quering some AABB tree might still be faster but
/// it has to be done against the surface bounds and not against what has been
/// actually rendered (so that the container will still receive the event)
pub struct SurfacePicker {
    // stateless for now
}

impl SurfacePicker {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: display: none
    // TODO: scroll
    // TODO: clip
    pub fn pick_at(&self, pos: Pos, children: &Vec<Vec<SurfaceId>>, bounds: &[Bounds]) -> SurfaceId {
        let mut parent = 0;
        let mut continue_down;

        // TODO: because bounds are not absolute
        let mut offset = Pos::default();

        // go down (starting from root) through each matching surface and return the last & deepest one
        loop {
            continue_down = false;
            offset = bounds[parent].a.relative_to(offset);

            for c in &children[parent] {
                if bounds[*c].relative_to(offset).contains(pos) {
                    parent = *c;
                    continue_down = true;
                }
            }

            if !continue_down {
                return parent
            }
        }
    }
}
