// flat, thread-safe interface
//
// TODO: bindings could be generated
//       (and maybe this could be too)

use crate::app::App;
use crate::document::{NodeId, Tag};
use crate::util::SlotMap;
use crate::window::{Event, Window};
use std::sync::Mutex;

pub struct Api {
    // TODO: deadlocks (thread?)
    app: Mutex<App>,
    wnds: Mutex<SlotMap<WindowId, Window>>,
}

impl Api {
    pub fn new(app: App) -> Self {
        Self {
            app: Mutex::new(app),
            wnds: Mutex::new(SlotMap::new()),
        }
    }

    pub fn create_window(&self, title: &str, width: i32, height: i32, notify_fn: fn(WindowId) -> ()) -> WindowId {
        self.wnds
            .lock()
            .unwrap()
            .insert_with_key(|id| self.app.lock().unwrap().create_window(title, width, height, Box::new(move || notify_fn(id))))
    }

    pub fn take_event(&self, window: WindowId) -> Option<Event> {
        self.wnds.lock().unwrap()[window].take_event()
    }

    pub fn create_text_node(&self, window: WindowId, text: &str) -> NodeId {
        self.wnds.lock().unwrap()[window].document_mut().create_text_node(text)
    }

    pub fn set_text(&self, window: WindowId, text_node: NodeId, text: &str) {
        self.wnds.lock().unwrap()[window].document_mut().set_text(text_node, text);
    }

    pub fn create_element(&self, window: WindowId, local_name_tag: Tag) -> NodeId {
        self.wnds.lock().unwrap()[window].document_mut().create_element(local_name_tag)
    }

    pub fn add_tag(&self, window: WindowId, element: NodeId, tag: Tag) {
        self.wnds.lock().unwrap()[window].document_mut().add_tag(element, tag);
    }

    pub fn remove_tag(&self, window: WindowId, element: NodeId, tag: Tag) {
        self.wnds.lock().unwrap()[window].document_mut().remove_tag(element, tag);
    }

    pub fn insert_child(&self, window: WindowId, parent: NodeId, child: NodeId, index: usize) {
        self.wnds.lock().unwrap()[window].document_mut().insert_child(parent, child, index);
    }

    pub fn remove_child(&self, window: WindowId, parent: NodeId, child: NodeId) {
        self.wnds.lock().unwrap()[window].document_mut().remove_child(parent, child);
    }

    pub fn free_node(&self, window: WindowId, node: NodeId) {
        self.wnds.lock().unwrap()[window].document_mut().free_node(node);
    }

    // TODO main-thread only
    // NEEDS to block
    pub fn tick(&self /*, _timeout: Option<u32/f32>*/) {
        // TOOD: main_thread_queue?
        for (id, w) in &mut self.wnds.lock().unwrap().iter_mut() {
            silly!("render {:?}", id);
            w.render();
        }

        self.app.lock().unwrap().tick();
    }

    pub fn wakeup(&self) {
        todo!();
    }
}

pub type WindowId = u32;

// compile-time check
#[allow(unused)]
fn api_is_thread_safe() {
    // we only need Sync for now
    |api: &'static Api| api as &(dyn /*Send + */Sync);
}
