// thread-safe FFI
// - the main idea here is that we hold all objects in a thread-local list of Rc<dyn Any>
//   and we only return/accept indices to that list so we can avoid (unsafe) pointers
// - in order to be able to do quick-lookups we also maintain HashMap<ptr, id>
//   this is useful for QSA but also for anything which returns Rc<> and hence could/should
//   result in some already existing id/index

use crate::css::CssStyleDeclaration;
use crate::util::SlotMap;
use crate::{App, CharacterData, Document, Element, Event, Node, Viewport, WebView, Window};
use crossbeam_channel::Receiver;
use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_double, c_int, c_uint};
use std::rc::Rc;
use std::sync::RwLock;

#[repr(transparent)]
pub struct ObjId<T: ?Sized>(c_uint, PhantomData<T>);

#[derive(Default)]
struct Ctx {
    refs: SlotMap<c_uint, Rc<dyn Any>>,
    ref_ids: HashMap<*const dyn Any, c_uint>,
}

thread_local! {
    static CTX: Rc<RefCell<Ctx>> = Default::default();
}

// shouldn't block unless when a window is created/destroyed
static EVENTS: Lazy<RwLock<SlotMap<u32, Receiver<Event>>>> = Lazy::new(Default::default);

#[no_mangle]
pub extern "C" fn gft_Rc_drop(obj: ObjId<dyn Any>) {
    CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let any = ctx.refs.remove(obj.0).unwrap();
        ctx.ref_ids.remove(&Rc::as_ptr(&any));
    })
}

#[no_mangle]
pub extern "C" fn gft_Vec_len(vec: ObjId<Vec<Rc<dyn Any>>>) -> c_uint {
    get(vec).len() as u32
}

#[no_mangle]
pub extern "C" fn gft_Vec_get(vec: ObjId<Vec<Rc<dyn Any>>>, index: c_uint) -> ObjId<Vec<Rc<dyn Any>>> {
    to_id(Rc::clone(get(vec).get(index as usize).expect("out of bounds")))
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_init() -> ObjId<App> {
    App::init().into()
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
pub unsafe extern "C" fn gft_Window_new(title: *const c_char, width: c_int, height: c_int) -> ObjId<Window> {
    let w = Window::new(to_str(title), width, height);
    let events = w.events().clone();

    let id = to_id(Rc::new(w));

    EVENTS.write().unwrap().put(id.0, events);

    id
}

// Window_next_event: |w| EVENTS.read().unwrap()[w].try_recv().ok().map(event),

#[no_mangle]
pub extern "C" fn gft_Window_title(win: ObjId<Window>) -> *mut c_char {
    CString::new(get(win).title()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_set_title(win: ObjId<Window>, title: *const c_char) {
    get(win).set_title(to_str(title))
}

#[no_mangle]
pub extern "C" fn gft_Window_width(win: ObjId<Window>) -> c_int {
    get(win).size().0
}

#[no_mangle]
pub extern "C" fn gft_Window_height(win: ObjId<Window>) -> c_int {
    get(win).size().1
}

#[no_mangle]
pub extern "C" fn gft_Window_resize(win: ObjId<Window>, width: c_int, height: c_int) {
    get(win).set_size((width, height))
}

#[no_mangle]
pub extern "C" fn gft_Window_show(win: ObjId<Window>) {
    get(win).show()
}

#[no_mangle]
pub extern "C" fn gft_Window_hide(win: ObjId<Window>) {
    get(win).hide()
}

#[no_mangle]
pub extern "C" fn gft_Window_focus(win: ObjId<Window>) {
    get(win).focus()
}

#[no_mangle]
pub extern "C" fn gft_Window_minimize(win: ObjId<Window>) {
    get(win).minimize()
}

#[no_mangle]
pub extern "C" fn gft_Window_maximize(win: ObjId<Window>) {
    get(win).maximize()
}

#[no_mangle]
pub extern "C" fn gft_Window_restore(win: ObjId<Window>) {
    get(win).restore()
}

#[no_mangle]
pub extern "C" fn gft_WebView_new() -> ObjId<WebView> {
    to_id(Rc::new(WebView::new()))
}

#[no_mangle]
pub extern "C" fn gft_WebView_attach(webview: ObjId<WebView>, win: ObjId<Window>) {
    get(webview).attach(&get(win))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_load_url(webview: ObjId<WebView>, url: *const c_char) {
    get(webview).load_url(to_str(url))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_eval(webview: ObjId<WebView>, script: *const c_char) {
    get(webview).eval(to_str(script))
}

#[no_mangle]
pub extern "C" fn gft_Document_new() -> ObjId<Document> {
    Document::new().into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_element(
    doc: ObjId<Document>,
    local_name: *const c_char,
) -> ObjId<Element> {
    get(doc).create_element(to_str(local_name)).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_text_node(
    doc: ObjId<Document>,
    data: *const c_char,
) -> ObjId<CharacterData> {
    get(doc).create_text_node(to_str(data)).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_comment(
    doc: ObjId<Document>,
    data: *const c_char,
) -> ObjId<CharacterData> {
    get(doc).create_comment(to_str(data)).into()
}

#[no_mangle]
pub extern "C" fn gft_Node_node_type(node: ObjId<dyn Node>) -> u32 {
    get_node(node).node_type() as u32
}

#[no_mangle]
pub extern "C" fn gft_Node_append_child(parent: ObjId<dyn Node>, child: ObjId<dyn Node>) {
    get_node(parent).append_child(get_node(child))
}

#[no_mangle]
pub extern "C" fn gft_Node_insert_before(parent: ObjId<dyn Node>, child: ObjId<dyn Node>, before: ObjId<dyn Node>) {
    get_node(parent).insert_before(get_node(child), get_node(before))
}

#[no_mangle]
pub extern "C" fn gft_Node_remove_child(parent: ObjId<dyn Node>, child: ObjId<dyn Node>) {
    get_node(parent).remove_child(get_node(child))
}

// TODO: to_id() + map(to_id())?
// //Node_query_selector: |node, sel: String| get_node(node).query_selector(&sel),
// //Node_query_selector_all: |node, sel: String| get_node(node).query_selector_all(&sel),

#[no_mangle]
pub unsafe extern "C" fn gft_CharacterData_data(node: ObjId<CharacterData>) -> *mut c_char {
    CString::new(get(node).data()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn gft_CharacterData_set_data(node: ObjId<CharacterData>, data: *const c_char) {
    get(node).set_data(to_str(data))
}

// Element_local_name: |el| get(el).local_name().to_string(),
#[no_mangle]
pub extern "C" fn gft_Element_local_name(el: ObjId<Element>) -> *mut c_char {
    CString::new(get(el).local_name().as_str()).unwrap().into_raw()
}

// Element_attribute_names: |el| get(el).attribute_names(),

#[no_mangle]
pub unsafe extern "C" fn gft_Element_attribute(el: ObjId<Element>, att: *const c_char) -> *mut c_char {
    match get(el).attribute(to_str(att)) {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_set_attribute(el: ObjId<Element>, att: *const c_char, val: *const c_char) {
    get(el).set_attribute(to_str(att), to_str(val))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_remove_attribute(el: ObjId<Element>, att: *const c_char) {
    get(el).remove_attribute(to_str(att))
}

#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_length(style: ObjId<CssStyleDeclaration>) -> c_uint {
    get(style).length() as _
}

// CssStyleDeclaration_property_value: |style, prop: String| get(el).style_property_value(&prop),
#[no_mangle]
pub unsafe extern "C" fn gft_CssStyleDeclaration_set_property(
    style: ObjId<CssStyleDeclaration>,
    prop: *const c_char,
    val: *const c_char,
) {
    //get(style).set_property(to_str(prop), to_str(val))
    println!("TODO: style.set_property()")
}

// Viewport_new: |w: f64, h: f64, doc: u32| to_id(Rc::new(Viewport::new((w as _, h as _), get(doc)))),
// Viewport_render: |w: u32, vp: u32| println!("TODO: Viewport_render"),

#[no_mangle]
pub extern "C" fn gft_Viewport_resize(viewport: ObjId<Viewport>, width: c_double, height: c_double) {
    get(viewport).resize((width as _, height as _))
}

// Viewport_element_from_point: |vp, x: f64, y: f64| get(vp).element_from_point((x as _, y as _))

impl<T: 'static> From<Rc<T>> for ObjId<T> {
    fn from(rc: Rc<T>) -> Self {
        to_id(rc)
    }
}

fn to_id<T>(any: Rc<dyn Any>) -> ObjId<T> {
    CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let ptr = Rc::as_ptr(&any);

        if let Some(&id) = ctx.ref_ids.get(&ptr) {
            return ObjId(id, PhantomData);
        }

        let id = ctx.refs.insert(any);
        ctx.ref_ids.insert(ptr, id);

        ObjId(id, PhantomData)
    })
}

fn any<T: ?Sized>(obj: ObjId<T>) -> Rc<dyn Any> {
    CTX.with(|ctx| Rc::clone(&ctx.borrow().refs[obj.0]))
}

fn get<T: 'static>(obj: ObjId<T>) -> Rc<T> {
    Rc::downcast::<T>(any(obj)).expect("invalid object type")
}

fn get_node(node: ObjId<dyn Node>) -> Rc<dyn Node> {
    let any = any(node);
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

unsafe fn to_str<'a>(ptr: *const c_char) -> &'a str {
    CStr::from_ptr(ptr).to_str().expect("invalid string")
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
