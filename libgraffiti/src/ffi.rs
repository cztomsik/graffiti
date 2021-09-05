// thread-safe FFI

use crate::css::CssStyleDeclaration;
use crate::util::SlotMap;
use crate::{App, CharacterData, Document, Element, Event, Node, Viewport, WebView, Window};
use crossbeam_channel::Receiver;
use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_double};
use std::rc::Rc;
use std::sync::RwLock;

type ObjId = c_uint;

#[derive(Default)]
struct Ctx {
    refs: SlotMap<ObjId, Rc<dyn Any>>,
    ref_ids: HashMap<*const dyn Any, ObjId>,
}

thread_local! {
    static CTX: Rc<RefCell<Ctx>> = Default::default();
}

// shouldn't block unless when a window is created/destroyed
static EVENTS: Lazy<RwLock<SlotMap<u32, Receiver<Event>>>> = Lazy::new(Default::default);

#[no_mangle]
pub extern "C" fn gft_Rc_drop(obj: ObjId) {
    CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let any = ctx.refs.remove(obj).unwrap();
        ctx.ref_ids.remove(&Rc::as_ptr(&any));
    })
}

#[no_mangle]
pub extern "C" fn gft_Vec_len(vec: ObjId) -> c_uint {
    get::<Vec<Rc<dyn Any>>>(vec).len() as u32
}

#[no_mangle]
pub extern "C" fn gft_Vec_get(vec: ObjId, index: c_uint) -> ObjId {
    to_id(Rc::clone(
        get::<Vec<Rc<dyn Any>>>(vec).get(index as usize).expect("out of bounds"),
    ))
}

#[no_mangle]
pub extern "C" fn gft_App_init() -> ObjId {
    to_id(unsafe { App::init() })
}

#[no_mangle]
pub extern "C" fn gft_App_tick() {
    App::current().expect("no app").tick()
}

#[no_mangle]
pub extern "C" fn gft_App_wake_up() {
    App::wake_up()
}

#[no_mangle]
pub extern "C" fn gft_Window_new(title: *const c_char, width: c_int, height: c_int) -> ObjId {
    let w = Window::new(to_str(title), width, height);
    let events = w.events().clone();

    let id = to_id(Rc::new(w));

    EVENTS.write().unwrap().put(id, events);

    id
}

// Window_next_event: |w| EVENTS.read().unwrap()[w].try_recv().ok().map(event),
// Window_title: |w| get::<Window>(w).title(),

#[no_mangle]
pub extern "C" fn gft_Window_set_title(win: ObjId, title: *const c_char) {
    get::<Window>(win).set_title(to_str(title))
}

#[no_mangle]
pub extern "C" fn gft_Window_width(win: ObjId) -> c_int {
    get::<Window>(win).size().0
}

#[no_mangle]
pub extern "C" fn gft_Window_height(win: ObjId) -> c_int {
    get::<Window>(win).size().1
}

#[no_mangle]
pub extern "C" fn gft_Window_resize(win: ObjId, width: c_int, height: c_int) {
    get::<Window>(win).set_size((width, height))
}

#[no_mangle]
pub extern "C" fn gft_Window_show(win: ObjId) {
    get::<Window>(win).show()
}

#[no_mangle]
pub extern "C" fn gft_Window_hide(win: ObjId) {
    get::<Window>(win).hide()
}

#[no_mangle]
pub extern "C" fn gft_Window_focus(win: ObjId) {
    get::<Window>(win).focus()
}

#[no_mangle]
pub extern "C" fn gft_Window_minimize(win: ObjId) {
    get::<Window>(win).minimize()
}

#[no_mangle]
pub extern "C" fn gft_Window_maximize(win: ObjId) {
    get::<Window>(win).maximize()
}

#[no_mangle]
pub extern "C" fn gft_Window_restore(win: ObjId) {
    get::<Window>(win).restore()
}

#[no_mangle]
pub extern "C" fn gft_WebView_new() -> ObjId {
    to_id(Rc::new(WebView::new()))
}

#[no_mangle]
pub extern "C" fn gft_WebView_attach(webview: ObjId, win: ObjId) {
    get::<WebView>(webview).attach(&get::<Window>(win))
}

#[no_mangle]
pub extern "C" fn gft_WebView_load_url(webview: ObjId, url: *const c_char) {
    get::<WebView>(webview).load_url(to_str(url))
}

#[no_mangle]
pub extern "C" fn gft_WebView_eval(webview: ObjId, script: *const c_char) {
    get::<WebView>(webview).eval(to_str(script))
}

#[no_mangle]
pub extern "C" fn gft_Document_new() -> ObjId {
    to_id(Document::new())
}

#[no_mangle]
pub extern "C" fn gft_Document_create_element(doc: ObjId, local_name: *const c_char) -> ObjId {
    to_id(get::<Document>(doc).create_element(to_str(local_name)))
}

#[no_mangle]
pub extern "C" fn gft_Document_create_text_node(doc: ObjId, data: *const c_char) -> ObjId {
    to_id(get::<Document>(doc).create_text_node(to_str(data)))
}

#[no_mangle]
pub extern "C" fn gft_Document_create_comment(doc: ObjId, data: *const c_char) -> ObjId {
    to_id(get::<Document>(doc).create_comment(to_str(data)))
}

#[no_mangle]
pub extern "C" fn gft_Node_node_type(node: ObjId) -> u32 {
    get_node(node).node_type() as u32
}

#[no_mangle]
pub extern "C" fn gft_Node_append_child(parent: ObjId, child: ObjId) {
    get_node(parent).append_child(get_node(child))
}

#[no_mangle]
pub extern "C" fn gft_Node_insert_before(parent: ObjId, child: ObjId, before: ObjId) {
    get_node(parent).insert_before(get_node(child), get_node(before))
}

#[no_mangle]
pub extern "C" fn gft_Node_remove_child(parent: ObjId, child: ObjId) {
    get_node(parent).remove_child(get_node(child))
}

// TODO: to_id() + map(to_id())?
// //Node_query_selector: |node, sel: String| get_node(node).query_selector(&sel),
// //Node_query_selector_all: |node, sel: String| get_node(node).query_selector_all(&sel),

// CharacterData_data: |node| get::<CharacterData>(node).data(),
#[no_mangle]
pub extern "C" fn gft_CharacterData_set_data(node: ObjId, data: *const c_char) {
    get::<CharacterData>(node).set_data(to_str(data))
}

// Element_local_name: |el| get::<Element>(el).local_name().to_string(),
// Element_attribute_names: |el| get::<Element>(el).attribute_names(),
// Element_attribute: |el, att: String| get::<Element>(el).attribute(&att),
#[no_mangle]
pub extern "C" fn gft_Element_set_attribute(el: ObjId, att: *const c_char, val: *const c_char) {
    get::<Element>(el).set_attribute(to_str(att), to_str(val))
}

#[no_mangle]
pub extern "C" fn gft_Element_remove_attribute(el: ObjId, att: *const c_char) {
    get::<Element>(el).remove_attribute(to_str(att))
}

#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_length(style: ObjId) -> c_uint {
    get::<CssStyleDeclaration>(style).length() as _
}

// CssStyleDeclaration_property_value: |style, prop: String| get::<Element>(el).style_property_value(&prop),
#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_set_property(style: ObjId, prop: *const c_char, val: *const c_char) {
    //get::<CssStyleDeclaration>(style).set_property(to_str(prop), to_str(val))
    println!("TODO: style.set_property()")
}

// Viewport_new: |w: f64, h: f64, doc: u32| to_id(Rc::new(Viewport::new((w as _, h as _), get::<Document>(doc)))),
// Viewport_render: |w: u32, vp: u32| println!("TODO: Viewport_render"),

#[no_mangle]
pub extern "C" fn gft_Viewport_resize(viewport: ObjId, width: c_double, height: c_double) {
    get::<Viewport>(viewport).resize((width as _, height as _))
}

// Viewport_element_from_point: |vp, x: f64, y: f64| get::<Viewport>(vp).element_from_point((x as _, y as _))

fn to_id(any: Rc<dyn Any>) -> ObjId {
    CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let ptr = Rc::as_ptr(&any);

        if let Some(&id) = ctx.ref_ids.get(&ptr) {
            return id;
        }

        let id = ctx.refs.insert(any);
        ctx.ref_ids.insert(ptr, id);

        id
    })
}

fn any(obj: ObjId) -> Rc<dyn Any> {
    CTX.with(|ctx| Rc::clone(&ctx.borrow().refs[obj]))
}

fn get<T: 'static>(obj: ObjId) -> Rc<T> {
    Rc::downcast::<T>(any(obj)).expect("invalid object type")
}

fn get_node(obj: ObjId) -> Rc<dyn Node> {
    let any = any(obj);
    let type_id = (&*any).type_id();

    if type_id == TypeId::of::<Element>() {
        Rc::downcast::<Element>(any).unwrap()
    } else if type_id == TypeId::of::<CharacterData>() {
        Rc::downcast::<CharacterData>(any).unwrap()
    } else if type_id == TypeId::of::<Document>() {
        Rc::downcast::<Document>(any).unwrap()
    } else {
        panic!("not a node")
    }
}

fn to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(ptr).to_str().expect("invalid string") }
}

// TODO: ?
fn event(ev: Event) -> (String, Option<(f64, f64)>, Option<u32>) {
    let res = match ev {
        Event::CursorPos(x, y) => ("mousemove", Some((x, y)), None),
        Event::MouseDown => ("mousedown", None, None),
        Event::MouseUp => ("mouseup", None, None),
        Event::Scroll(x, y) => ("scroll", Some((x, y)), None),

        // JS e.which
        Event::KeyDown(code) => ("keydown", None, Some(code)),
        Event::KeyUp(code) => ("keyup", None, Some(code)),
        Event::KeyPress(ch) => ("keypress", None, Some(ch)),
        Event::Resize(w, h) => ("resize", Some((w as _, h as _)), None),
        Event::FramebufferSize(w, h) => ("fbsize", Some((w as _, h as _)), None),
        Event::Close => ("close", None, None),
    };

    (res.0.to_owned(), res.1, res.2)
}
