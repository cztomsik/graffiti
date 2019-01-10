use std::sync::Mutex;
use crate::rendering::{RenderOperation};

pub struct ResourceManager {
    pub render_ops: Vec<RenderOperation>,
    free_bucket_ids: Vec<BucketId>
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            render_ops: Vec::with_capacity(200),
            free_bucket_ids: vec![]
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
                return OpResource(bucket_id, 1)
            }
        }

        let bucket_ids: Vec<BucketId> = ops.iter().map(|op| self.create_bucket((*op).clone())).collect();

        OpResource(bucket_ids[0], bucket_ids.len() as u32)
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
#[derive(Deserialize)]
pub struct OpResource(pub BucketId, pub u32);

impl Drop for OpResource {
    fn drop(&mut self) {
        ResourceManager::with(|rm| {
            for i in 1..self.1 {
                debug!("release bucket");
                rm.release_bucket(i)
            }
        })
    }
}

thread_local! {
    static CURRENT_RESOURCE_MANAGER: Mutex<ResourceManager> = Mutex::new(ResourceManager::new());
}
