// thread-safe FFI
// - meant to be safe rather than efficient, it is expected to be used with some kind of vdom
//   to reduce number of calls
// - the main idea here is that we hold all objects in a thread-local list of values
//   and we only return/accept indices to that list so we can avoid (unsafe) pointers
// - each Ref you get has to be dropped (once), even if you are just traversing the tree
// - two Refs can technically point to the same Rc<>

use crate::util::SlotMap;
use crate::{
    App, CharacterDataRef, CssStyleDeclaration, DocumentRef, ElementRef, Event, NodeId, NodeRef, NodeType, Renderer,
    WebView, Window, WindowId,
};
use std::any::Any;
use std::cell::RefCell;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::Index;
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::rc::Rc;
use std::sync::Arc;

// cbindgen hack for Option<Ref<T>>
#[cfg(cbindgen)]
#[repr(transparent)]
struct Option<T>(T);

#[repr(transparent)]
pub struct Ref<T: ?Sized>(NonZeroU32, PhantomData<*const T>);

#[derive(Debug, Clone)]
pub enum Value {
    Node(NodeRef),
    Rc(Rc<dyn Any>),
    Arc(Arc<dyn Any>),
    String(String),
    Vec(Vec<Value>),
}

thread_local! {
    static REFS: RefCell<SlotMap<NonZeroU32, Value>> = Default::default();
}

#[no_mangle]
pub extern "C" fn gft_Ref_drop(obj: Ref<Value>) {
    with_tls(|tls| tls.remove(obj.0));
}

#[no_mangle]
pub extern "C" fn gft_Vec_len(vec: Ref<Vec<Value>>) -> c_uint {
    with_tls(|tls| tls[&vec].len()) as _
}

#[no_mangle]
pub extern "C" fn gft_Vec_get(vec: Ref<Vec<Value>>, index: c_uint) -> Ref<Value> {
    let val = with_tls(|tls| tls[&vec].get(index as usize).expect("out of bounds").clone());
    new_ref(val)
}

#[no_mangle]
pub extern "C" fn gft_String_bytes_len(string: Ref<String>) -> c_uint {
    with_tls(|tls| tls[&string].bytes().len() as _)
}

#[no_mangle]
pub unsafe extern "C" fn gft_String_copy(string: Ref<String>, dest_buf: *mut u8) {
    with_tls(|tls| {
        let bytes = tls[&string].as_bytes();
        dest_buf.copy_from(bytes.as_ptr(), bytes.len())
    })
}

#[no_mangle]
pub unsafe extern "C" fn gft_App_init() -> Ref<App> {
    App::init().into()
}

#[no_mangle]
pub extern "C" fn gft_App_current() -> Option<Ref<App>> {
    App::current().map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_App_tick(app: Ref<App>) {
    with_tls(|tls| tls[&app].tick())
}

#[no_mangle]
pub extern "C" fn gft_App_wake_up(app: Ref<App>) {
    with_tls(|tls| tls[&app].wake_up())
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_new(
    title: *const c_char,
    title_len: u32,
    width: c_int,
    height: c_int,
) -> Ref<Window> {
    Window::new(to_str(title, title_len), width, height).into()
}

#[no_mangle]
pub extern "C" fn gft_Window_id(win: Ref<Window>) -> WindowId {
    with_tls(|tls| tls[&win].id())
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_find_by_id(id: WindowId) -> Option<Ref<Window>> {
    Window::find_by_id(id).map(From::from)
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_next_event(win: Ref<Window>, event_dest: *mut Event) -> bool {
    with_tls(|tls| {
        if let Ok(event) = tls[&win].events().try_recv() {
            *event_dest = event;
            return true;
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn gft_Window_title(win: Ref<Window>) -> Ref<String> {
    with_tls(|tls| tls[&win].title().into())
}

#[no_mangle]
pub unsafe extern "C" fn gft_Window_set_title(win: Ref<Window>, title: *const c_char, title_len: u32) {
    with_tls(|tls| tls[&win].set_title(to_str(title, title_len)))
}

#[no_mangle]
pub extern "C" fn gft_Window_width(win: Ref<Window>) -> c_int {
    with_tls(|tls| tls[&win].size().0)
}

#[no_mangle]
pub extern "C" fn gft_Window_height(win: Ref<Window>) -> c_int {
    with_tls(|tls| tls[&win].size().1)
}

#[no_mangle]
pub extern "C" fn gft_Window_resize(win: Ref<Window>, width: c_int, height: c_int) {
    with_tls(|tls| tls[&win].set_size((width, height)))
}

#[no_mangle]
pub extern "C" fn gft_Window_should_close(win: Ref<Window>) -> bool {
    with_tls(|tls| tls[&win].should_close())
}

#[no_mangle]
pub extern "C" fn gft_Window_show(win: Ref<Window>) {
    with_tls(|tls| tls[&win].show())
}

#[no_mangle]
pub extern "C" fn gft_Window_hide(win: Ref<Window>) {
    with_tls(|tls| tls[&win].hide())
}

#[no_mangle]
pub extern "C" fn gft_Window_focus(win: Ref<Window>) {
    with_tls(|tls| tls[&win].focus())
}

#[no_mangle]
pub extern "C" fn gft_Window_minimize(win: Ref<Window>) {
    with_tls(|tls| tls[&win].minimize())
}

#[no_mangle]
pub extern "C" fn gft_Window_maximize(win: Ref<Window>) {
    with_tls(|tls| tls[&win].maximize())
}

#[no_mangle]
pub extern "C" fn gft_Window_restore(win: Ref<Window>) {
    with_tls(|tls| tls[&win].restore())
}

#[no_mangle]
pub extern "C" fn gft_WebView_new() -> Ref<WebView> {
    Rc::new(WebView::new()).into()
}

#[no_mangle]
pub extern "C" fn gft_WebView_attach(webview: Ref<WebView>, win: Ref<Window>) {
    with_tls(|tls| tls[&webview].attach(&tls[&win]))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_load_url(webview: Ref<WebView>, url: *const c_char, url_len: u32) {
    with_tls(|tls| tls[&webview].load_url(to_str(url, url_len)))
}

#[no_mangle]
pub unsafe extern "C" fn gft_WebView_eval(webview: Ref<WebView>, script: *const c_char, script_len: u32) {
    with_tls(|tls| tls[&webview].eval(to_str(script, script_len)))
}

#[no_mangle]
pub extern "C" fn gft_Document_new() -> Ref<DocumentRef> {
    DocumentRef::new().as_node().into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_element(
    doc: Ref<DocumentRef>,
    local_name: *const c_char,
    local_name_len: u32,
) -> Ref<ElementRef> {
    with_tls(|tls| tls[&doc].create_element(to_str(local_name, local_name_len)))
        .as_node()
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_text_node(
    doc: Ref<DocumentRef>,
    data: *const c_char,
    data_len: u32,
) -> Ref<CharacterDataRef> {
    with_tls(|tls| tls[&doc].create_text_node(to_str(data, data_len)))
        .as_node()
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Document_create_comment(
    doc: Ref<DocumentRef>,
    data: *const c_char,
    data_len: u32,
) -> Ref<CharacterDataRef> {
    with_tls(|tls| tls[&doc].create_comment(to_str(data, data_len)))
        .as_node()
        .into()
}

#[no_mangle]
pub extern "C" fn gft_Node_id(node: Ref<NodeRef>) -> NodeId {
    with_tls(|tls| tls[&node].id())
}

#[no_mangle]
pub extern "C" fn gft_Node_node_type(node: Ref<NodeRef>) -> NodeType {
    with_tls(|tls| tls[&node].node_type())
}

#[no_mangle]
pub extern "C" fn gft_Node_parent_node(node: Ref<NodeRef>) -> Option<Ref<NodeRef>> {
    with_tls(|tls| tls[&node].parent_node()).map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_Node_first_child(node: Ref<NodeRef>) -> Option<Ref<NodeRef>> {
    with_tls(|tls| tls[&node].first_child()).map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_Node_last_child(node: Ref<NodeRef>) -> Option<Ref<NodeRef>> {
    with_tls(|tls| tls[&node].last_child()).map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_Node_previous_sibling(node: Ref<NodeRef>) -> Option<Ref<NodeRef>> {
    with_tls(|tls| tls[&node].previous_sibling()).map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_Node_next_sibling(node: Ref<NodeRef>) -> Option<Ref<NodeRef>> {
    with_tls(|tls| tls[&node].next_sibling()).map(Ref::from)
}

#[no_mangle]
pub extern "C" fn gft_Node_append_child(parent: Ref<NodeRef>, child: Ref<NodeRef>) {
    with_tls(|tls| tls[&parent].append_child(&tls[&child]))
}

#[no_mangle]
pub extern "C" fn gft_Node_insert_before(parent: Ref<NodeRef>, child: Ref<NodeRef>, before: Ref<NodeRef>) {
    with_tls(|tls| tls[&parent].insert_before(&tls[&child], &tls[&before]))
}

#[no_mangle]
pub extern "C" fn gft_Node_remove_child(parent: Ref<NodeRef>, child: Ref<NodeRef>) {
    with_tls(|tls| tls[&parent].remove_child(&tls[&child]))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Node_query_selector(
    node: Ref<NodeRef>,
    selector: *const c_char,
    selector_len: u32,
) -> Option<Ref<ElementRef>> {
    with_tls(|tls| tls[&node].query_selector(to_str(selector, selector_len)))
        .map(|el| el.as_node())
        .map(Ref::from)
}

#[no_mangle]
pub unsafe extern "C" fn gft_Node_query_selector_all(
    node: Ref<NodeRef>,
    selector: *const c_char,
    selector_len: u32,
) -> Ref<Vec<Value>> {
    let els = with_tls(|tls| tls[&node].query_selector_all(to_str(selector, selector_len)));
    // TODO: map(Value::from)???
    //       and maybe we could use it as part of Ref::from() or maybe replace Ref::from entirely with short snippet
    //       which is repeated over again but also explicit about TLS usage
    els.iter()
        .map(|el| Value::Node(el.as_node()))
        .collect::<Vec<_>>()
        .into()
}

#[no_mangle]
pub extern "C" fn gft_CharacterData_data(node: Ref<CharacterDataRef>) -> Ref<String> {
    with_tls(|tls| tls[&node].data()).into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_CharacterData_set_data(node: Ref<CharacterDataRef>, data: *const c_char, data_len: u32) {
    with_tls(|tls| tls[&node].set_data(to_str(data, data_len)))
}

#[no_mangle]
pub extern "C" fn gft_Element_local_name(el: Ref<ElementRef>) -> Ref<String> {
    with_tls(|tls| tls[&el].local_name().to_string()).into()
}

#[no_mangle]
pub extern "C" fn gft_Element_attribute_names(el: Ref<ElementRef>) -> Ref<Vec<Value>> {
    let names = with_tls(|tls| tls[&el].attribute_names());
    let values: Vec<_> = names.into_iter().map(Value::String).collect();

    values.into()
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_attribute(
    el: Ref<ElementRef>,
    att: *const c_char,
    att_len: u32,
) -> Option<Ref<String>> {
    with_tls(|tls| tls[&el].attribute(to_str(att, att_len))).map(Ref::from)
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_set_attribute(
    el: Ref<ElementRef>,
    att: *const c_char,
    att_len: u32,
    val: *const c_char,
    val_len: u32,
) {
    with_tls(|tls| tls[&el].set_attribute(to_str(att, att_len), to_str(val, val_len)))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_remove_attribute(el: Ref<ElementRef>, att: *const c_char, att_len: u32) {
    with_tls(|tls| tls[&el].remove_attribute(to_str(att, att_len)))
}

#[no_mangle]
pub unsafe extern "C" fn gft_Element_matches(el: Ref<ElementRef>, selector: *const c_char, selector_len: u32) -> bool {
    with_tls(|tls| tls[&el].matches(to_str(selector, selector_len)))
}

#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_length(style: Ref<CssStyleDeclaration>) -> c_uint {
    with_tls(|tls| tls[&style].length() as _)
}

#[no_mangle]
pub unsafe extern "C" fn gft_CssStyleDeclaration_property_value(
    style: Ref<CssStyleDeclaration>,
    prop: *const c_char,
    prop_len: u32,
) -> Option<Ref<String>> {
    with_tls(|tls| tls[&style].property_value(to_str(prop, prop_len)).map(Ref::from))
}

#[no_mangle]
pub extern "C" fn gft_CssStyleDeclaration_set_property(
    style: Ref<CssStyleDeclaration>,
    prop: *const c_char,
    prop_len: u32,
    val: *const c_char,
    val_len: u32,
) {
    todo!()
    //with_tls(|tls| tls[&style].set_property(to_str(prop, prop_len), to_str(val, val_len)))
}

#[no_mangle]
pub extern "C" fn gft_Renderer_new(doc: Ref<DocumentRef>, win: Ref<Window>) -> Ref<Renderer> {
    let renderer = with_tls(|tls| Renderer::new(tls[&doc].clone(), &tls[&win]));
    Rc::new(renderer).into()
}

#[no_mangle]
pub extern "C" fn gft_Renderer_render(renderer: Ref<Renderer>) {
    with_tls(|tls| tls[&renderer].render())
}

#[no_mangle]
pub extern "C" fn gft_Renderer_resize(renderer: Ref<Renderer>, width: f32, height: f32) {
    with_tls(|tls| tls[&renderer].resize(width, height))
}

// Viewport_element_from_point: |vp, x: f64, y: f64| get(vp).element_from_point((x as _, y as _))

// TODO: make it more explicit, into() hides TLS
impl<T: 'static> From<NodeRef> for Ref<T> {
    fn from(node: NodeRef) -> Self {
        new_ref(Value::Node(node))
    }
}

impl<T: 'static> From<Rc<T>> for Ref<T> {
    fn from(rc: Rc<T>) -> Self {
        new_ref(Value::Rc(rc))
    }
}

impl<T: 'static> From<Arc<T>> for Ref<T> {
    fn from(arc: Arc<T>) -> Self {
        new_ref(Value::Arc(arc))
    }
}

impl From<String> for Ref<String> {
    fn from(string: String) -> Self {
        new_ref(Value::String(string))
    }
}

impl From<Vec<Value>> for Ref<Vec<Value>> {
    fn from(vec: Vec<Value>) -> Self {
        new_ref(Value::Vec(vec))
    }
}

fn new_ref<T: ?Sized>(value: Value) -> Ref<T> {
    REFS.with(|refs| Ref(refs.borrow_mut().insert(value), PhantomData))
}

unsafe fn to_str<'a>(ptr: *const c_char, len: u32) -> &'a str {
    // match len {
    //     0 => CStr::from_ptr(ptr).to_str().expect("invalid string"),
    //     len => std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr as _, len as _)),
    // }
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr as _, len as _))
}

fn with_tls<T>(mut fun: impl FnMut(&mut SlotMap<NonZeroU32, Value>) -> T) -> T {
    REFS.with(|refs| {
        let mut refs = refs.borrow_mut();
        fun(&mut *refs)
    })
}

impl<T: 'static> Index<&Ref<T>> for SlotMap<NonZeroU32, Value> {
    type Output = T;

    fn index(&self, index: &Ref<T>) -> &T {
        match &self[index.0] {
            Value::Node(node) => node.downcast_ref::<T>(),
            Value::Rc(rc) => rc.downcast_ref::<T>(),
            Value::Arc(arc) => arc.downcast_ref::<T>(),
            Value::String(string) => <dyn Any>::downcast_ref(string),
            Value::Vec(vec) => <dyn Any>::downcast_ref(vec),
        }
        .expect("invalid object type")
    }
}
