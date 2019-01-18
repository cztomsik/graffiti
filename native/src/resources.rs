use crate::rendering::RenderOperation;
use std::sync::Mutex;

pub struct ResourceManager {
    pub render_ops: Vec<RenderOperation>,
    free_bucket_ids: Vec<BucketId>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            render_ops: Vec::with_capacity(200),
            free_bucket_ids: vec![],
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

    pub fn create_op_resource(&mut self, ops: Vec<RenderOperation>) -> OpResource {
        // reuse buckets for 1-size resources
        if ops.len() == 1 {
            if let Some(bucket_id) = self.free_bucket_ids.pop() {
                self.update_bucket(bucket_id, ops[0].clone());

                debug!("OpRes {} {:?}", bucket_id, &ops);
                return OpResource::new(bucket_id, 1);
            }
        }

        let bucket_ids: Vec<BucketId> = ops
            .iter()
            .map(|op| self.create_bucket((*op).clone()))
            .collect();

        debug!("OpRes {} {:?}", bucket_ids[0], &ops);
        OpResource::new(bucket_ids[0], bucket_ids.len() as u32)
    }

    pub fn create_bucket(&mut self, op: RenderOperation) -> BucketId {
        // TODO: panic if not u32
        let index = self.render_ops.len() as u32;

        self.render_ops.push(op);

        index
    }

    pub fn update_bucket(&mut self, bucket_id: BucketId, op: RenderOperation) {
        match self.render_ops.get_mut(bucket_id as usize) {
            None => panic!("bucket not found"),
            Some(bucket) => *bucket = op,
        }
    }

    pub fn release_bucket(&mut self, bucket_id: BucketId) {
        self.free_bucket_ids.push(bucket_id)
    }
}

pub type BucketId = u32;

// slice in global render_ops
// must not be serialized/deserialized
pub struct OpResource {
    pub start: BucketId,
    pub length: u32,
}

impl OpResource {
    pub fn new(start: BucketId, length: u32) -> Self {
        OpResource { start, length }
    }
}

impl Drop for OpResource {
    fn drop(&mut self) {
        debug!("drop op_res {:?} {:?}", self.start, self.length);

        ResourceManager::with(|rm| {
            for i in self.start..(self.start + self.length) {
                rm.release_bucket(i)
            }
        })
    }
}

impl std::fmt::Debug for OpResource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OpRes {}", self.start)
    }
}

thread_local! {
    static CURRENT_RESOURCE_MANAGER: Mutex<ResourceManager> = Mutex::new(ResourceManager::new());
}
