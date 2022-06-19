// thread-safe API
// (TODO: rename from FFI)
// This is not a typical FFI with pointers, C strings and similar things
//
// I was trying different approaches (N-API, FFI) but messaging still feels the best:
// - no worries about FFI-safety (Result<>, Option<>, NonZeroXxx, Vec<>)
// - there is only one unsafe fn which has to be checked
// - there is some overhead but it's fast enough even with JSON ser/de/ser/de

use crate::{App, Document, NodeId, Viewport, WindowId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};
use std::{cell::RefCell, io::Write, slice, str};

thread_local! {
    static STATE: RefCell<(State, LastRes)> = Default::default();
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

    STATE.with(|res| {
        let (state, last_res) = &mut *res.borrow_mut();

        state.handle_msg(msg, last_res);

        // println!("-> {:?}", last_res);

        if last_res.0.is_empty() {
            std::ptr::null()
        } else {
            last_res.0.as_ptr()
        }
    })
}

impl State {
    fn handle_msg(&mut self, msg: ApiMsg, last_res: &mut LastRes) {
        match msg {
            ApiMsg::Init => self.app = Some(App::init()),
            ApiMsg::Tick => self.app.as_mut().unwrap().tick(),
            ApiMsg::WakeUp => App::wake_up(),

            ApiMsg::CreateDocument => last_res.replace(insert(&mut self.documents, Default::default())),
            ApiMsg::DocumentMsg(id, msg) => {
                let mut doc = self.documents.get_mut(&id).unwrap().write().unwrap();

                match msg {
                    DocumentMsg::CreateElement(local_name) => last_res.replace(doc.create_element(local_name)),
                    DocumentMsg::SetAttribute(el, att, val) => doc.set_attribute(el, att, val),
                    DocumentMsg::RemoveAttribute(el, att) => doc.remove_attribute(el, att),
                    DocumentMsg::SetStyle(el, style) => doc.set_style(el, style),
                    DocumentMsg::AppendChild(parent, child) => doc.append_child(parent, child),
                    DocumentMsg::InsertBefore(parent, child, before) => doc.insert_before(parent, child, before),
                    DocumentMsg::RemoveChild(parent, child) => doc.remove_child(parent, child),
                    DocumentMsg::QuerySelector(node, selector) => last_res.replace(doc.query_selector(node, selector)),
                    DocumentMsg::QuerySelectorAll(node, selector) => {
                        last_res.replace(doc.query_selector_all(node, selector).collect::<Vec<_>>())
                    }
                    DocumentMsg::CreateTextNode(ref text) => last_res.replace(doc.create_text_node(text)),
                    DocumentMsg::SetText(node, ref text) => doc.set_text(node, text),
                    DocumentMsg::DropNode(node) => doc.drop_node(node),
                }
            }

            ApiMsg::CreateViewport(size, doc) => {
                last_res.replace(insert(
                    &mut self.viewports,
                    Arc::new(RwLock::new(Viewport::new(size, self.documents.get(&doc).unwrap()))),
                ));
            }
            ApiMsg::ViewportMsg(id, msg) => {
                // let mut vp = self.viewports.get_mut(&id).unwrap().write().unwrap();
                // todo!()
            }

            ApiMsg::CreateWindow(title, width, height) => {
                last_res.replace(self.app.as_mut().unwrap().create_window(title, width, height));
            }
            ApiMsg::WindowMsg(id, msg) => match msg {
                WindowMsg::SetContent(Some(vp)) => {
                    let vp = Arc::clone(&self.viewports[&vp]);

                    App::push_task(move |app| {
                        app.window_mut(id).set_content(Some(vp));
                    })
                }
                _ => {}
            },
        }
    }

    // TODO: will be useful for App/Win messages
    // fn await_task<T: Send + 'static>(&mut self, task: impl FnOnce(&mut App) -> T + 'static + Send) -> T {
    //     if let Some(app) = &mut self.app {
    //         task(app)
    //     } else {
    //         let (tx, rx) = channel();
    //         App::push_task(move |app| tx.send(task(app)).unwrap());
    //         rx.recv().unwrap()
    //     }
    // }
}

#[derive(Debug, Default)]
struct LastRes(Vec<u8>);

impl LastRes {
    fn replace<T: Serialize>(&mut self, res: T) {
        self.0 = serde_json::to_vec(&res).unwrap();
        self.0.write(&[0]).unwrap();
    }
}

fn insert<V>(dest: &mut HashMap<u32, V>, val: V) -> u32 {
    let id = next_id();
    dest.insert(id, val);

    id
}

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);

    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// // fn gft_Window_next_event(win: u32, event_dest: *mut Event) -> bool {
// //     if let Ok(event) = WINDOWS.with(|wins| wins.borrow_mut()[&win].events().try_recv() {)
// //         *event_dest = event;
// //         return true;
// //     }
// //     false
// // }
