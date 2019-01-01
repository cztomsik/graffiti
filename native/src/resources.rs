use std::sync::Mutex;

use webrender::api::{
    BorderDisplayItem, GlyphInstance, PushStackingContextDisplayItem, RectangleDisplayItem,
    TextDisplayItem,
};

pub struct ResourceManager {
    pub buckets: Vec<RenderOperation>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            buckets: Vec::with_capacity(100),
        }
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        CURRENT_RESOURCE_MANAGER.with(|mutex| {
            let mut rm = mutex.lock().unwrap();

            f(&mut rm)
        })
    }

    pub fn create_bucket(&mut self, op: RenderOperation) -> BucketId {
        // TODO: panic if not u32
        let index = self.buckets.len() as u32;

        self.buckets.push(op);

        index
    }

    pub fn update_bucket(&mut self, bucket_id: BucketId, op: RenderOperation) {
        match self.buckets.get_mut(bucket_id as usize) {
            None => panic!("bucket not found"),
            Some(bucket) => *bucket = op,
        }
    }
}

pub type BucketId = u32;

// a bit like opcode
// mostly follows SpecificDisplayItem::* but the Text actually holds glyphs
#[derive(Deserialize, Debug)]
pub enum RenderOperation {
    // this was hack at first but it could be useful for hitSlop (hitBox can be bigger than clipBox)
    HitTest(u32),
    SaveRect,
    PushScrollClip,
    PushBorderRadiusClip(f32),
    PopClip,
    Rectangle(RectangleDisplayItem),
    Border(BorderDisplayItem),
    Text(TextDisplayItem, Vec<GlyphInstance>),
    PopStackingContext,
    PushStackingContext(PushStackingContextDisplayItem),
}

thread_local! {
    static CURRENT_RESOURCE_MANAGER: Mutex<ResourceManager> = Mutex::new(ResourceManager::new());
}
