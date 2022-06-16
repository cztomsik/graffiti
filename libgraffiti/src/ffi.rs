// (TODO: rename from FFI)
// This is not a FFI, FFI a way how a lib is supposed to be integrated with other languages
// and it's ok to use pointers and everything, this is not the case

use crate::{App, Document, NodeId, NodeType, Viewport, Window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

pub type WindowId = u32;
pub type ViewportId = u32;
pub type DocumentId = u32;

thread_local! {
    static STATE: RefCell<State> = Default::default();
}

#[derive(Debug, Default)]
struct State {
    app: Option<Rc<App>>,
    windows: HashMap<WindowId, Window>,
    viewports: HashMap<ViewportId, Arc<RwLock<Viewport>>>,
    documents: HashMap<DocumentId, Arc<RwLock<Document>>>,
}

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);

    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
