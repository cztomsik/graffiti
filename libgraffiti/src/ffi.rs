// thread-safe FFI
// - expected to be used with some kind of vdom to reduce number of calls
// - the main idea here is that we hold all objects in a thread-local lists of values
//   and we only return/accept indices to those lists so we can avoid (unsafe) pointers
// - whatever you create should be freed, using respective `gft_Xxx_drop(id)` fn

use crate::util::{Atom, SlotMap};
use crate::{Document, NodeId, NodeType};
use std::cell::RefCell;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::sync::Arc;

// cbindgen hack for Option<Id<T>>
#[cfg(cbindgen)]
#[repr(transparent)]
struct Option<T>(T);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Id<T>(usize, PhantomData<T>);

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
    // let app: Option<Arc<App>>;
    let documents: SlotMap<Id<Document>, Document>;
    // let windows: SlotMap<Id<Window>, Arc<Window>>;
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

    /*
    fn gft_Atom_from(ptr: *const i8) -> Atom {
        Atom::from(to_str(ptr))
    }

    fn gft_App_init() {
        *app = Some(App::init())
    }

    fn gft_App_tick() {
        app.as_ref().unwrap().tick()
    }

    fn gft_App_wake_up() {
        App::current().unwrap().wake_up()
    }

    fn gft_App_drop() {
        app.take();
    }

    fn gft_Window_new(title: *const i8, width: i32, height: i32) -> Id<Window> {
        windows.insert(Window::new(to_str(title), width, height))
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

    fn gft_Window_title(win: Id<Window>) -> *const i8 {
        todo!()
        //windows[win].title()
    }

    fn gft_Window_set_title(win: Id<Window>, title: *const i8) {
        windows[win].set_title(to_str(title))
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
    */

    fn gft_Document_new() -> Id<Document> {
        documents.insert(Document::new())
    }

    fn gft_Document_create_element(doc: Id<Document>, local_name: Atom) -> NodeId {
        documents[doc].create_element(local_name)
    }

    fn gft_Document_create_text_node(doc: Id<Document>, data: *const i8) -> NodeId {
        documents[doc].create_text_node(to_str(data))
    }

    fn gft_Document_node_type(doc: Id<Document>, node: NodeId) -> NodeType {
        documents[doc].node_type(node)
    }

    fn gft_Document_parent_node(doc: Id<Document>, node: NodeId) -> Option<NodeId> {
        documents[doc].parent_node(node)
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

    fn gft_Document_query_selector(doc: Id<Document>, node: NodeId, selector: *const i8) -> Option<NodeId> {
        documents[doc].query_selector(node, to_str(selector))
    }

    // fn gft_Document_query_selector_all(doc: Id<Document>, node: NodeId, selector: *const u8, selector_len: usize) -> Id<Vec<Value>> {
    //     let els = documents[doc].query_selector_all(node, to_str(selector, selector_len));
    //     els.iter()
    //         .map(|el| Value::Node(el.as_node()))
    //         .collect::<Vec<_>>()
    //         .into()
    // }

    fn gft_Document_local_name(doc: Id<Document>, el: NodeId) -> Atom {
        documents[doc].local_name(el)
    }

    // fn gft_Element_attribute_names(doc: Id<Document>, el: NodeId) -> Id<Vec<Value>> {
    //     let names = documents[doc].element(el).attribute_names()
    //     let values: Vec<_> = names.into_iter().map(Value::String).collect();
    //     values.into()
    // }

    // fn gft_Element_attribute(doc: Id<Document>, el: NodeId, att: *const u8, att_len: usize) -> Option<Id<String>> {
    //     strings.insert(documents[doc].element(el).attribute(to_str(att, att_len).into()))
    // }

    fn gft_Element_set_attribute(doc: Id<Document>, el: NodeId, att: Atom, val: *const i8) {
        documents[doc].set_attribute(el, att, to_str(val));
    }

    fn gft_Element_remove_attribute(doc: Id<Document>, el: NodeId, att: Atom) {
        documents[doc].remove_attribute(el, att);
    }

    // fn gft_Element_matches(doc: Id<Document>, el: NodeId, selector: *const u8, selector_len: usize) -> bool {
    //     documents[doc].element(el).matches(to_str(selector, selector_len))
    // }

    // fn gft_Element_style(doc: Id<Document>, el: NodeId) -> Id<CssStyleDeclaration> {
    //     documents[doc].element(el).style().into()
    // }

    fn gft_Document_text(doc: Id<Document>, text_node: NodeId) -> *const i8 {
        todo!()
    }

    fn gft_Document_set_text(doc: Id<Document>, text_node: NodeId, text: *const i8) {
        documents[doc].set_text(text_node, to_str(text));
    }

    fn gft_Document_drop(doc: Id<Document>) {
        documents.remove(doc);
    }
}

unsafe fn to_str<'a>(ptr: *const i8) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap()
}
