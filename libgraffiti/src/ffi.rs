// TODO: u32/NonZeroU32/usize/K: Key/Vec<T>

// thread-safe FFI
// - pointers are only allowed for strings
// - Atoms are safe because deref is bounds checked
// - everything else is using indices or other primitive types
// - expected to be used with some kind of vdom to reduce number of calls
// - the main idea here is that we hold all objects in a thread-local lists of values
//   and we only return/accept indices to those lists so we can avoid (unsafe) pointers
// - whatever you create should be freed, using respective `gft_Xxx_drop(id)` fn

use crate::util::Atom;
use crate::{App, Document, NodeId, NodeType, Window};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

pub type DocumentId = u32;
pub type WindowId = u32;

thread_local! {
    static APP: RefCell<Option<Rc<App>>> = Default::default();
    // TODO: windows are main-thread only but we can use App::await_task()
    //       to access TLS, so almost everything should work from any thread
    static WINDOWS: RefCell<HashMap<WindowId, Window>> = Default::default();
}

static STATE: Lazy<Mutex<State>> = Lazy::new(Default::default);

#[derive(Debug, Default)]
struct State {
    documents: HashMap<DocumentId, Document>,
}

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);

    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[no_mangle]
pub unsafe extern "C" fn gft_Atom_from(ptr: *const i8) -> Atom {
    Atom::from(to_str(ptr))
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_init() {
    APP.with(|dest| *dest.borrow_mut() = Some(App::init()))
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_tick() {
    App::current().unwrap().tick()
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_wake_up() {
    App::wake_up()
}

// TODO: App_focus/show/hide/quit()

// fn gft_Window_next_event(win: WindowId, event_dest: *mut Event) -> bool {
//     if let Ok(event) = WINDOWS.with(|wins| wins.borrow_mut()[&win].events().try_recv() {)
//         *event_dest = event;
//         return true;
//     }
//     false
// }

#[no_mangle]
pub unsafe extern "C" fn gft_Window_new(title: *const i8, width: i32, height: i32) -> WindowId {
    let id = next_id();
    WINDOWS.with(|wins| wins.borrow_mut().insert(id, Window::new(to_str(title), width, height)));
    id
}

#[no_mangle]
pub extern "C" fn gft_Window_title(win: WindowId) -> *const i8 {
    todo!()
    //WINDOWS.with(|wins| wins.borrow_mut()[&win].title())
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_set_title(win: WindowId, title: *const i8) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].set_title(to_str(title)))
}

#[no_mangle]
pub extern "C" fn gft_Window_width(win: WindowId) -> i32 {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].size().0)
}

#[no_mangle]
pub extern "C" fn gft_Window_height(win: WindowId) -> i32 {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].size().1)
}

#[no_mangle]
pub extern "C" fn gft_Window_resize(win: WindowId, width: i32, height: i32) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].set_size((width, height)))
}

#[no_mangle]
pub extern "C" fn gft_Window_should_close(win: WindowId) -> bool {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].should_close())
}

#[no_mangle]
pub extern "C" fn gft_Window_show(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].show())
}

#[no_mangle]
pub extern "C" fn gft_Window_hide(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].hide())
}

#[no_mangle]
pub extern "C" fn gft_Window_focus(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].focus())
}

#[no_mangle]
pub extern "C" fn gft_Window_minimize(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].minimize())
}

#[no_mangle]
pub extern "C" fn gft_Window_maximize(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].maximize())
}

#[no_mangle]
pub extern "C" fn gft_Window_restore(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut()[&win].restore())
}

#[no_mangle]
pub extern "C" fn gft_Window_drop(win: WindowId) {
    WINDOWS.with(|wins| wins.borrow_mut().remove(&win));
}

#[no_mangle]
pub extern "C" fn gft_Document_new() -> DocumentId {
    let id = next_id();
    STATE.lock().unwrap().documents.insert(id, Document::new());
    id
}

#[no_mangle]
pub extern "C" fn gft_Document_create_element(doc: DocumentId, local_name: Atom) -> NodeId {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .create_element(local_name)
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_text_node(doc: DocumentId, data: *const i8) -> NodeId {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .create_text_node(to_str(data))
}

#[no_mangle]
pub extern "C" fn gft_Document_node_type(doc: DocumentId, node: NodeId) -> NodeType {
    STATE.lock().unwrap().documents.get(&doc).unwrap().node_type(node)
}

#[no_mangle]
pub extern "C" fn gft_Document_parent_node(doc: DocumentId, node: NodeId) -> Option<NodeId> {
    STATE.lock().unwrap().documents.get(&doc).unwrap().parent_node(node)
}

#[no_mangle]
pub extern "C" fn gft_Document_append_child(doc: DocumentId, parent: NodeId, child: NodeId) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .append_child(parent, child);
}

#[no_mangle]
pub extern "C" fn gft_Document_insert_before(doc: DocumentId, parent: NodeId, child: NodeId, before: NodeId) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .insert_before(parent, child, before);
}

#[no_mangle]
pub extern "C" fn gft_Document_remove_child(doc: DocumentId, parent: NodeId, child: NodeId) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .remove_child(parent, child);
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_query_selector(
    doc: DocumentId,
    node: NodeId,
    selector: *const i8,
) -> Option<NodeId> {
    STATE
        .lock()
        .unwrap()
        .documents
        .get(&doc)
        .unwrap()
        .query_selector(node, to_str(selector))
}

#[no_mangle]
pub extern "C" fn gft_Document_local_name(doc: DocumentId, el: NodeId) -> Atom {
    STATE.lock().unwrap().documents.get(&doc).unwrap().local_name(el)
}

#[no_mangle]
pub extern "C" fn gft_Document_attribute_names(doc: DocumentId, el: NodeId) {
    todo!()
}

#[no_mangle]
pub extern "C" fn gft_Document_attribute(doc: DocumentId, el: NodeId, att: *const u8, att_len: usize) {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_set_attribute(doc: DocumentId, el: NodeId, att: Atom, val: *const i8) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .set_attribute(el, att, to_str(val));
}

#[no_mangle]
pub extern "C" fn gft_Document_remove_attribute(doc: DocumentId, el: NodeId, att: Atom) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .remove_attribute(el, att);
}

#[no_mangle]
pub extern "C" fn gft_Document_text(doc: DocumentId, text_node: NodeId) -> *const i8 {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_set_text(doc: DocumentId, text_node: NodeId, text: *const i8) {
    STATE
        .lock()
        .unwrap()
        .documents
        .get_mut(&doc)
        .unwrap()
        .set_text(text_node, to_str(text));
}

#[no_mangle]
pub extern "C" fn gft_Document_drop(doc: DocumentId) {
    STATE.lock().unwrap().documents.remove(&doc);
}

unsafe fn to_str<'a>(ptr: *const i8) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap()
}
