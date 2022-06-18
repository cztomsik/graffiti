// (TODO: rename from FFI)
// This is not a FFI, FFI a way how a lib is supposed to be integrated with other languages
// and it's ok to use pointers and everything, this is not the case

use crate::{App, Document, NodeId, NodeType, Viewport, Window};
use serde::Deserialize;
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
    app: Option<App>,
    viewports: HashMap<ViewportId, Arc<RwLock<Viewport>>>,
    documents: HashMap<DocumentId, Arc<RwLock<Document>>>,
}

#[derive(Debug, Deserialize)]
enum ApiMsg<'a> {
    Init,
    Tick,
    WakeUp, // TODO: App_Focus/Show/Hide/Quit/AppMsg?

    CreateWindow(&'a str, i32, i32),
    WindowMsg(WindowId, WindowMsg),

    CreateViewport((f32, f32), DocumentId),
    ViewportMsg(ViewportId, ViewportMsg),

    CreateDocument,
    DocumentMsg(DocumentId, DocumentMsg<'a>),
}

// TODO: add all getters & methods from Window (but add Get/IsXxx prefix)
#[derive(Debug, Deserialize)]
enum WindowMsg {
    SetContent(Option<ViewportId>),
}

#[derive(Debug, Deserialize)]
enum ViewportMsg {
    // TODO: GetRect, GetElementAt, Scroll, ...
}

#[derive(Debug, Deserialize)]
enum DocumentMsg<'a> {
    CreateElement(&'a str),
    CreateTextNode(&'a str),
    AppendChild(NodeId, NodeId),
    InsertBefore(NodeId, NodeId, NodeId),
    RemoveChild(NodeId, NodeId),
}

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);

    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[no_mangle]
pub unsafe extern "C" fn gft_send(data: *const u8, len: usize) -> *const u8 {
    todo!()
}
