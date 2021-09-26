// thread-safe FFI
// - meant to be safe rather than efficient, it is expected to be used with some kind of vdom
//   to reduce number of calls
// - the main idea here is that we hold all objects in a thread-local list of Rc<dyn Any>
//   and we only return/accept indices to that list so we can avoid (unsafe) pointers
// - each Ref you get has to be dropped (once), even if you are just traversing the tree
// - two Refs can point to the same object/resource but it's possible to get key which is
//   guaranteed to be same (it's pointer-based but not a pointer itself)

// https://github.com/eqrion/cbindgen/issues/385
#![allow(bare_trait_objects)]

use crate::css::CssStyleDeclaration;
use crate::util::SlotMap;
use crate::{App, CharacterData, Document, Element, Event, Node, NodeType, Viewport, WebView, Window};
use crossbeam_channel::Receiver;
use once_cell::sync::Lazy;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::BitXor;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::rc::Rc;
use std::sync::RwLock;

#[repr(transparent)]
pub struct Ref<T: ?Sized>(c_uint, PhantomData<T>);

impl<T: ?Sized> Ref<T> {
    pub const NULL: Self = Ref(0, PhantomData);
}

thread_local! {
    static REFS: Rc<RefCell<SlotMap<c_uint, Rc<dyn Any>>>> = {
        let refs: Rc<RefCell<SlotMap<c_uint, Rc<dyn Any>>>> = Default::default();
        let null = refs.borrow_mut().insert(Rc::new(()));
        assert_eq!(null, Ref::<dyn Any>::NULL.0);
        refs
    }
}

// shouldn't block unless when a window is created/destroyed
static EVENTS: Lazy<RwLock<SlotMap<u32, Receiver<Event>>>> = Lazy::new(Default::default);

#[no_mangle]
pub extern "C" fn gft_Ref_drop(obj: Ref<Any>) {
    REFS.with(|refs| refs.borrow_mut().remove(obj.0));
}

#[no_mangle]
pub extern "C" fn gft_Ref_key(obj: Ref<Any>) -> u64 {
    let ptr: u64 = REFS.with(|refs| Rc::as_ptr(&refs.borrow()[obj.0])) as *const c_void as usize as _;
    let key = ptr.bitxor(0xF0F0F0);

    // TODO: https://github.com/denoland/deno/issues/12212
    assert!(key <= 9007199254740991);

    key
}

#[no_mangle]
pub extern "C" fn gft_Vec_len(vec: Ref<Vec<Rc<Any>>>) -> c_uint {
    get(vec).len() as u32
}

#[no_mangle]
pub extern "C" fn gft_Vec_get(vec: Ref<Vec<Rc<Any>>>, index: c_uint) -> Ref<Any> {
    new_ref(Rc::clone(get(vec).get(index as usize).expect("out of bounds")))
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_init() -> Ref<App> {
    App::init().into()
}

#[no_mangle]
pub extern "C" fn gft_App_current() -> Ref<App> {
    App::current().into()
}

#[no_mangle]
pub extern "C" fn gft_App_tick(app: Ref<App>) {
    get(app).tick()
}

#[no_mangle]
pub extern "C" fn gft_App_wake_up() {
    App::wake_up()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_new(title: *const c_char, width: c_int, height: c_int) -> Ref<Window> {
    let w = Window::new(to_str(title), width, height);
    let events = w.events().clone();

    let id: Ref<Window> = Rc::new(w).into();

    EVENTS.write().unwrap().put(id.0, events);

    id
}

// Window_next_event: |w| EVENTS.read().unwrap()[w].try_recv().ok().map(event),

#[no_mangle]
pub extern "C" fn gft_Window_title(win: Ref<Window>) -> *mut c_char {
    CString::new(get(win).title()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_set_title(win: Ref<Window>, title: *const c_char) {
    get(win).set_title(to_str(title))
}

#[no_mangle]
pub extern "C" fn gft_Window_width(win: Ref<Window>) -> c_int {
    get(win).size().0
}

#[no_mangle]
pub extern "C" fn gft_Window_height(win: Ref<Window>) -> c_int {
    get(win).size().1
}

#[no_mangle]
pub extern "C" fn gft_Window_resize(win: Ref<Window>, width: c_int, height: c_int) {
    get(win).set_size((width, height))
}

#[no_mangle]
pub extern "C" fn gft_Window_should_close(win: Ref<Window>) -> bool {
    get(win).should_close()
}

#[no_mangle]
pub extern "C" fn gft_Window_show(win: Ref<Window>) {
    get(win).show()
}

#[no_mangle]
pub extern "C" fn gft_Window_hide(win: Ref<Window>) {
    get(win).hide()
}

#[no_mangle]
pub extern "C" fn gft_Window_focus(win: Ref<Window>) {
    get(win).focus()
}

#[no_mangle]
pub extern "C" fn gft_Window_minimize(win: Ref<Window>) {
    get(win).minimize()
}

#[no_mangle]
pub extern "C" fn gft_Window_maximize(win: Ref<Window>) {
    get(win).maximize()
}

#[no_mangle]
pub extern "C" fn gft_Window_restore(win: Ref<Window>) {
    get(win).restore()
}

#[no_mangle]
pub extern "C" fn gft_WebView_new() -> Ref<WebView> {
    Rc::new(WebView::new()).into()
}

#[no_mangle]
pub extern "C" fn gft_WebView_attach(webview: Ref<WebView>, win: Ref<Window>) {
    get(webview).attach(&get(win))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_load_url(webview: Ref<WebView>, url: *const c_char) {
    get(webview).load_url(to_str(url))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_eval(webview: Ref<WebView>, script: *const c_char) {
    get(webview).eval(to_str(script))
}

#[no_mangle]
pub extern "C" fn gft_Document_new() -> Ref<Document> {
    Document::new().into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_element(doc: Ref<Document>, local_name: *const c_char) -> Ref<Element> {
    get(doc).create_element(to_str(local_name)).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_text_node(doc: Ref<Document>, data: *const c_char) -> Ref<CharacterData> {
    get(doc).create_text_node(to_str(data)).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_comment(doc: Ref<Document>, data: *const c_char) -> Ref<CharacterData> {
    get(doc).create_comment(to_str(data)).into()
}

#[no_mangle]
pub extern "C" fn gft_Node_node_type(node: Ref<Node>) -> NodeType {
    get_node(node).node_type()
}

#[no_mangle]
pub extern "C" fn gft_Node_parent_node(node: Ref<Node>) -> Ref<Node> {
    get_node(node).parent_node().into()
}

#[no_mangle]
pub extern "C" fn gft_Node_first_child(node: Ref<Node>) -> Ref<Node> {
    get_node(node).first_child().into()
}

#[no_mangle]
pub extern "C" fn gft_Node_last_child(node: Ref<Node>) -> Ref<Node> {
    get_node(node).last_child().into()
}

#[no_mangle]
pub extern "C" fn gft_Node_previous_sibling(node: Ref<Node>) -> Ref<Node> {
    get_node(node).previous_sibling().into()
}

#[no_mangle]
pub extern "C" fn gft_Node_next_sibling(node: Ref<Node>) -> Ref<Node> {
    get_node(node).next_sibling().into()
}

#[no_mangle]
pub extern "C" fn gft_Node_append_child(parent: Ref<Node>, child: Ref<Node>) {
    get_node(parent).append_child(get_node(child))
}

#[no_mangle]
pub extern "C" fn gft_Node_insert_before(parent: Ref<Node>, child: Ref<Node>, before: Ref<Node>) {
    get_node(parent).insert_before(get_node(child), get_node(before))
}

#[no_mangle]
pub extern "C" fn gft_Node_remove_child(parent: Ref<Node>, child: Ref<Node>) {
    get_node(parent).remove_child(get_node(child))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Node_query_selector(node: Ref<Node>, selector: *const c_char) -> Ref<Element> {
    get_node(node).query_selector(to_str(selector)).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Node_query_selector_all(
    node: Ref<Node>,
    selector: *const c_char,
) -> Ref<Vec<Rc<Any>>> {
    let anys: Vec<_> = get_node(node)
        .query_selector_all(to_str(selector))
        .iter()
        .map(|r| r.clone() as Rc<dyn Any>)
        .collect();
    Rc::new(anys).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_CharacterData_data(node: Ref<CharacterData>) -> *mut c_char {
    CString::new(get(node).data()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn gft_CharacterData_set_data(node: Ref<CharacterData>, data: *const c_char) {
    get(node).set_data(to_str(data))
}

#[no_mangle]
pub extern "C" fn gft_Element_local_name(el: Ref<Element>) -> *mut c_char {
    CString::new(get(el).local_name().as_str()).unwrap().into_raw()
}

// Element_attribute_names: |el| get(el).attribute_names(),

#[no_mangle]
pub unsafe extern "C" fn gft_Element_attribute(el: Ref<Element>, att: *const c_char) -> *mut c_char {
    match get(el).attribute(to_str(att)) {
        Some(s) => CString::new(s).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_set_attribute(el: Ref<Element>, att: *const c_char, val: *const c_char) {
    get(el).set_attribute(to_str(att), to_str(val))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_remove_attribute(el: Ref<Element>, att: *const c_char) {
    get(el).remove_attribute(to_str(att))
}

#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_length(style: Ref<CssStyleDeclaration>) -> c_uint {
    get(style).length() as _
}

// CssStyleDeclaration_property_value: |style, prop: String| get(el).style_property_value(&prop),
#[no_mangle]
pub unsafe extern "C" fn gft_CssStyleDeclaration_set_property(
    style: Ref<CssStyleDeclaration>,
    prop: *const c_char,
    val: *const c_char,
) {
    //get(style).set_property(to_str(prop), to_str(val))
    println!("TODO: style.set_property()")
}

// Viewport_new: |w: f64, h: f64, doc: u32| to_id(Rc::new(Viewport::new((w as _, h as _), get(doc)))),
// Viewport_render: |w: u32, vp: u32| println!("TODO: Viewport_render"),

#[no_mangle]
pub extern "C" fn gft_Viewport_resize(viewport: Ref<Viewport>, width: c_double, height: c_double) {
    get(viewport).resize((width as _, height as _))
}

// Viewport_element_from_point: |vp, x: f64, y: f64| get(vp).element_from_point((x as _, y as _))

impl<T: ?Sized> From<Option<Rc<T>>> for Ref<T>
where
    Rc<T>: Into<Ref<T>>,
{
    fn from(opt: Option<Rc<T>>) -> Self {
        match opt {
            Some(obj) => obj.into(),
            None => Ref::NULL,
        }
    }
}

impl<T: 'static> From<Rc<T>> for Ref<T> {
    fn from(rc: Rc<T>) -> Self {
        new_ref(rc)
    }
}

// TODO: add to_node() to Node trait
impl From<Rc<dyn Node>> for Ref<dyn Node> {
    fn from(node: Rc<dyn Node>) -> Ref<dyn Node> {
        match node.node_type() {
            NodeType::Document => new_ref(node.downcast::<Document>().unwrap()),
            NodeType::Element => new_ref(node.downcast::<Element>().unwrap()),
            NodeType::Text => new_ref(node.downcast::<CharacterData>().unwrap()),
            NodeType::Comment => new_ref(node.downcast::<CharacterData>().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn new_ref<T: ?Sized>(any: Rc<dyn Any>) -> Ref<T> {
    REFS.with(|refs| Ref(refs.borrow_mut().insert(any), PhantomData))
}

fn any<T: ?Sized>(obj: Ref<T>) -> Rc<dyn Any> {
    REFS.with(|refs| Rc::clone(&refs.borrow()[obj.0]))
}

fn get<T: 'static>(obj: Ref<T>) -> Rc<T> {
    Rc::downcast::<T>(any(obj)).expect("invalid object type")
}

fn get_node(node: Ref<Node>) -> Rc<dyn Node> {
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
