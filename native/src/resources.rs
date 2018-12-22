use std::sync::Mutex;

use webrender::api::{
    BorderDisplayItem, GlyphInstance, PushStackingContextDisplayItem, RectangleDisplayItem,
    TextDisplayItem,
};

pub struct ResourceManager {
    pub display_items: Vec<DisplayItem>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            display_items: Vec::with_capacity(100),
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

    pub fn create_bucket(&mut self, item: DisplayItem) -> BucketId {
        // TODO: panic if not u32
        let index = self.display_items.len() as u32;

        self.display_items.push(item);

        index
    }

    pub fn update_bucket(&mut self, bucket_id: BucketId, item: DisplayItem) {
        match self.display_items.get_mut(bucket_id as usize) {
            None => panic!("bucket not found"),
            Some(bucket) => *bucket = item,
        }
    }
}

pub type BucketId = u32;

// like SpecificDisplayItem::* but the Text actually holds glyphs
#[derive(Deserialize)]
pub enum DisplayItem {
    // this was hack at first but it could be useful for hitSlop (hitBox can be bigger than clipBox)
    HitTest(u32),
    SaveRect,
    PushScrollClip,
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
