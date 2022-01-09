// thread-safe FFI
// - expected to be used with some kind of vdom to reduce number of calls
// - the main idea here is that we hold all objects in a thread-local lists of values
//   and we only return/accept indices to those lists so we can avoid (unsafe) pointers
// - whatever you create should be freed, using respective `gft_Xxx_drop(id)` fn

use crate::util::{Id, SlotMap};
use crate::{App, Document, NodeId, NodeKind, Window};
use std::cell::RefCell;
use std::num::NonZeroU32;
use std::sync::Arc;

// cbindgen hack for Option<Id<T>>
#[cfg(cbindgen)]
#[repr(transparent)]
struct Option<T>(T);

// wrap all fns in `#[no_mangle] pub extern "C"` with access to defined thread-local vars
macro_rules! ffi {
    (
        $(let $tls:ident: $tls_ty:ty;)*
        fn $($rest:tt)*
    ) => (
        thread_local! { static TLS: RefCell<($($tls_ty),*)> = RefCell::default(); }
        ffi!(@inner ($($tls),*), fn $($rest)*);
    );

    (@inner $tls:pat, $(fn $fn:ident( $($arg:ident: $arg_ty:ty),* $(,)* ) $(-> $res:ty)* { $($body:tt)* })*) => (
        $(#[no_mangle] pub unsafe extern "C" fn $fn($($arg:$arg_ty),*) $( -> $res)* {
            TLS.with(|tls| {
                let $tls = &mut *tls.borrow_mut();
                $($body)*
            })
        })*
    );
}

// enum DocHandle { Document(Document), Window(Id<Window>) }

ffi! {
    let app: Option<Arc<App>>;
    let documents: SlotMap<Id<Document>, Document>;
    let windows: SlotMap<Id<Window>, Arc<Window>>;
    let strings: SlotMap<Id<String>, String>;

    // fn gft_Vec_len(vec: Ref<Vec<Value>>) -> c_uint {
    //     tls[&vec].len() as _
    // }

    // fn gft_Vec_get(vec: Ref<Vec<Value>>, index: c_uint) -> Ref<Value> {
    //     let val = tls[&vec].get(index as usize).expect("out of bounds").clone();
    //     new_ref(val)
    // }

    fn gft_String_bytes_len(string: Id<String>) -> usize {
        strings[string].bytes().len() as _
    }

    fn gft_String_copy(string: Id<String>, dest_buf: *mut u8) {
        let bytes = strings[string].as_bytes();
        dest_buf.copy_from(bytes.as_ptr(), bytes.len());
    }

    fn gft_String_drop(string: Id<String>) {
        strings.remove(string);
    }

    fn gft_App_init() {
        *app = Some(App::init())
    }

    fn gft_App_tick(app: Id<App>) {
        app.as_ref().unwrap().tick()
    }

    fn gft_App_wake_up(app: Id<App>) {
        app.as_ref().unwrap().wake_up()
    }

    fn gft_App_drop(app: Id<App>) {
        app.take();
    }

    fn gft_Window_new(title: *const u8, title_len: usize, width: i32, height: i32) -> Id<Window> {
        windows.insert(Window::new(to_str(title, title_len), width, height))
    }

    // fn gft_Window_id(win: Id<Window>) -> WindowId {
    //     windows[win].id()
    // }

    // fn gft_Window_find_by_id(id: WindowId) -> Option<Id<Window>> {
    //     Window::find_by_id(id).map(From::from)
    // }

    // fn gft_Window_next_event(win: Id<Window>, event_dest: *mut Event) -> bool {
    //     if let Ok(event) = windows[win].events().try_recv() {
    //         *event_dest = event;
    //         return true;
    //     }
    //     false
    // }

    fn gft_Window_title(win: Id<Window>) -> Id<String> {
        strings.insert(windows[win].title())
    }

    fn gft_Window_set_title(win: Id<Window>, title: *const u8, title_len: usize) {
        windows[win].set_title(to_str(title, title_len))
    }

    fn gft_Window_width(win: Id<Window>) -> i32 {
        windows[win].size().0
    }

    fn gft_Window_height(win: Id<Window>) -> i32 {
        windows[win].size().1
    }

    fn gft_Window_resize(win: Id<Window>, width: i32, height: i32) {
        windows[win].set_size((width, height))
    }

    fn gft_Window_should_close(win: Id<Window>) -> bool {
        windows[win].should_close()
    }

    fn gft_Window_show(win: Id<Window>) {
        windows[win].show()
    }

    fn gft_Window_hide(win: Id<Window>) {
        windows[win].hide()
    }

    fn gft_Window_focus(win: Id<Window>) {
        windows[win].focus()
    }

    fn gft_Window_minimize(win: Id<Window>) {
        windows[win].minimize()
    }

    fn gft_Window_maximize(win: Id<Window>) {
        windows[win].maximize()
    }

    fn gft_Window_restore(win: Id<Window>) {
        windows[win].restore()
    }

    fn gft_Window_drop(window: Id<Window>) {
        windows.remove(window);
    }

    fn gft_Document_new() -> Id<Document> {
        documents.insert(Document::new())
    }

    fn gft_Document_root(doc: Id<Document>) -> NodeId {
        documents[doc].root()
    }

    fn gft_Document_create_element(doc: Id<Document>, local_name: *const u8, local_name_len: usize) -> NodeId {
        documents[doc].create_element(to_str(local_name, local_name_len))
    }

    fn gft_Document_create_text_node(doc: Id<Document>, data: *const u8, data_len: usize) -> NodeId {
        documents[doc].create_text_node(to_str(data, data_len))
    }

    fn gft_Document_node_kind(doc: Id<Document>, node: NodeId) -> NodeKind {
        documents[doc].node(node).kind()
    }

    fn gft_Document_node_parent_node(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].node(node).parent_node()
    }

    fn gft_Document_node_first_child(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].node(node).first_child()
    }

    fn gft_Document_node_last_child(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].node(node).last_child()
    }

    fn gft_Document_node_previous_sibling(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].node(node).previous_sibling()
    }

    fn gft_Document_node_next_sibling(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].node(node).next_sibling()
    }

    fn gft_Document_append_child(doc: Id<Document>, parent: NodeId, child: NodeId) {
        documents[doc].append_child(parent, child);
    }

    fn gft_Document_insert_before(doc: Id<Document>, parent: NodeId, child: NodeId, before: NodeId) {
        documents[doc].insert_before(parent, child, before);
    }

    fn gft_Document_remove_child(doc: Id<Document>, parent: NodeId, child: NodeId) {
        documents[doc].remove_child(parent, child);
    }

    fn gft_Document_query_selector(doc: Id<Document>, node: NodeId, selector: *const u8, selector_len: usize) -> Option<NodeId> {
        documents[doc].query_selector(node, to_str(selector, selector_len))
    }

    // fn gft_Document_query_selector_all(doc: Id<Document>, node: NodeId, selector: *const u8, selector_len: usize) -> Id<Vec<Value>> {
    //     let els = documents[doc].query_selector_all(node, to_str(selector, selector_len));
    //     els.iter()
    //         .map(|el| Value::Node(el.as_node()))
    //         .collect::<Vec<_>>()
    //         .into()
    // }

    // fn gft_Element_local_name(doc: Id<Document>, el: NodeId) -> Id<String> {
    //     strings.insert(documents[doc].element(el).local_name().to_string())
    // }

    // fn gft_Element_attribute_names(doc: Id<Document>, el: NodeId) -> Id<Vec<Value>> {
    //     let names = documents[doc].element(el).attribute_names()
    //     let values: Vec<_> = names.into_iter().map(Value::String).collect();
    //     values.into()
    // }

    // fn gft_Element_attribute(doc: Id<Document>, el: NodeId, att: *const u8, att_len: usize) -> Option<Id<String>> {
    //     strings.insert(documents[doc].element(el).attribute(to_str(att, att_len).into()))
    // }

    // fn gft_Element_set_attribute(doc: Id<Document>, el: NodeId, att: *const u8, att_len: usize, val: *const u8, val_len: usize) {
    //     documents[doc].element(el).set_attribute(to_str(att, att_len), to_str(val, val_len))
    // }

    // fn gft_Element_remove_attribute(doc: Id<Document>, el: NodeId, att: *const u8, att_len: usize) {
    //     documents[doc].element(el).remove_attribute(to_str(att, att_len))
    // }

    // fn gft_Element_matches(doc: Id<Document>, el: NodeId, selector: *const u8, selector_len: usize) -> bool {
    //     documents[doc].element(el).matches(to_str(selector, selector_len))
    // }

    // fn gft_Element_style(doc: Id<Document>, el: NodeId) -> Id<CssStyleDeclaration> {
    //     documents[doc].element(el).style().into()
    // }

    // fn gft_Text_data(doc: Id<Document>, node: Id<TextRef>) -> Id<String> {
    //     documents[doc].data().into()
    // }

    // fn gft_Text_set_data(doc: Id<Document>, node: Id<TextRef>, data: *const u8, data_len: usize) {
    //     documents[doc].set_data(to_str(data, data_len))
    // }

    fn gft_Document_drop(doc: Id<Document>) {
        documents.remove(doc);
    }

    // fn gft_CssStyleDeclaration_length(style: Id<CssStyleDeclaration>) -> c_uint {
    //     tls[&style].length() as _
    // }

    // fn gft_CssStyleDeclaration_property_value(
    //     style: Id<CssStyleDeclaration>,
    //     prop: *const u8,
    //     prop_len: usize,
    // ) -> Option<Id<String>> {
    //     tls[&style].property_value(to_str(prop, prop_len)).map(Ref::from)
    // }

    // fn gft_CssStyleDeclaration_set_property(style: Id<CssStyleDeclaration>, prop: *const u8, prop_len: usize, val: *const u8, val_len: usize) {
    //     tls[&style].set_property(to_str(prop, prop_len), to_str(val, val_len))
    // }

    // fn gft_CssStyleDeclaration_remove_property(style: Id<CssStyleDeclaration>, prop: *const u8, prop_len: usize) {
    //     tls[&style].remove_property(to_str(prop, prop_len))
    // }

    // fn gft_CssStyleDeclaration_css_text(style: Id<CssStyleDeclaration>) -> Id<String> {
    //     tls[&style].css_text().into()
    // }

    // fn gft_CssStyleDeclaration_set_css_text(style: Id<CssStyleDeclaration>, css_text: *const u8, css_text_len: usize) {
    //     tls[&style].set_css_text(to_str(css_text, css_text_len))
    // }
}

unsafe fn to_str<'a>(ptr: *const u8, len: usize) -> &'a str {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
}
