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
    // TODO: display: none
    // TODO: scroll
    // TODO: clip
    pub fn pick_at(&self, pos: Pos, children: &[&[SurfaceId]], bounds: &[Bounds]) -> SurfaceId {
        let result = 0;
        let parent = 0;

        // TODO: if the next child matches too we could skip going down
        for c in children[parent] {
            if bounds[*c].contains(pos) {
                result = c;
            }
        }

        result
    }
}
