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
use std::{slice, str};

pub type WindowId = u32;
pub type ViewportId = u32;
pub type DocumentId = u32;

thread_local! {
    static STATE: RefCell<State> = Default::default();
}

pub type ViewportId = u32;
pub type DocumentId = u32;

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

#[derive(Debug, Deserialize)]
enum WindowMsg {
    SetContent(Option<ViewportId>),
    GetTitle,
    SetTitle,
    GetSize,
    SetSize,
    IsResizable,
    SetResizable,
    Opacity,
    SetOpacity,
    IsVisible,
    Show,
    Hide,
    IsFocused,
    Focus,
    IsMinimized,
    Minimize,
    IsMaximized,
    Maximize,
    Restore,
    RequestAttention,
}

#[derive(Debug, Deserialize)]
enum ViewportMsg {
    // TODO: GetRect, GetElementAt, Scroll, ...
}

#[derive(Debug, Deserialize)]
enum DocumentMsg<'a> {
    CreateElement(&'a str),
    SetAttribute(NodeId, &'a str, &'a str),
    RemoveAttribute(NodeId, &'a str),
    // TODO: is this right? or { UpdateStyle: { SetCssText: text } }?
    SetStyle(NodeId, &'a str),

    AppendChild(NodeId, NodeId),
    InsertBefore(NodeId, NodeId, NodeId),
    RemoveChild(NodeId, NodeId),

    QuerySelector(NodeId, &'a str),
    QuerySelectorAll(NodeId, &'a str),

    // TODO: switch to bincode https://github.com/serde-rs/serde/issues/1413#issuecomment-494892266
    CreateTextNode(String),
    SetText(NodeId, String),

    DropNode(NodeId),
}

#[no_mangle]
pub unsafe extern "C" fn gft_send(data: *const u8, len: usize) -> *const u8 {
    // get slice of bytes & try to deserialize
    let msg = str::from_utf8(slice::from_raw_parts(data, len)).unwrap();
    let msg: ApiMsg = serde_json::from_str(msg).unwrap_or_else(|err| panic!("invalid msg {msg} {err}"));

    println!("{:?}", &msg);

    // TODO: this is just to get JS<->serde working
    b"1\0" as _
}
