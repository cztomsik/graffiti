// node.js bindings

// note it should be possible to wrap just NodeId (it fits in usize/ptr) in js nodes,
// and then we'd only need some own WeakRef (*const RcBox) as finalizer_hint
// and finalize could check if the doc is still alive and free the node
// but it would be arguably much more magic and the perf would be the same,
// maybe even worse (because of one extra js_doc we'd need to pass/unwrap)
// so I guess it's not worth, at least not until there's some other reason
//
// BTW: that Rc is basically free because it always points to the same place
// so the only cost is incrementing/decrementing one counter for each
// create/finalize

#![allow(non_camel_case_types, unused)]

use std::rc::Rc;
use std::cell::RefCell;
use crate::document::{Document, NodeId};


extern fn js_init_module(env: napi_env, exports: napi_value) -> napi_value {
    silly!("init native module");

    //unsafe { crate::window::init() };
    //start_wakeup_thread();

    env.set_prop(exports, "initDocument", env.js_fn(js_init_document));
    env.set_prop(exports, "querySelector", env.js_fn(js_query_selector));
    //env.set_prop(exports, "querySelectorAll", env.js_fn(js_query_selector_all));
    env.set_prop(exports, "makeRoot", env.js_fn(js_make_root));

    env.set_prop(exports, "initElement", env.js_fn(js_init_element));
    env.set_prop(exports, "setAttribute", env.js_fn(js_set_attribute));
    env.set_prop(exports, "insertChildAt", env.js_fn(js_insert_child_at));
    env.set_prop(exports, "removeChild", env.js_fn(js_remove_child));

    env.set_prop(exports, "initTextNode", env.js_fn(js_init_text_node));
    env.set_prop(exports, "setText", env.js_fn(js_set_text));

    exports
}

extern fn js_init_document(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_doc, ..] = env.args(cb_info);

    let doc: RcDoc = Rc::new(RefCell::new(Document::new()));
    env.wrap(js_doc, doc);

    env.js_undefined()
}

unsafe extern fn js_query_selector(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_doc, js_selector, opt_js_element, ..] = env.args(cb_info);

    let doc = env.unwrap::<RcDoc>(js_doc);
    let selector = env.string(js_selector);
    let element = env.opt_unwrap::<NodeRef>(opt_js_element).map(|n| n.node);

    match doc.borrow().query_selector(&selector, element) {
        Some(node) => env.ref_value(doc.borrow().expando(node).expect("no js ref")),
        None => env.js_undefined()
    }
}

unsafe extern fn js_make_root(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_el, ..] = env.args(cb_info);

    let el = env.unwrap::<NodeRef>(js_el);

    el.doc.borrow_mut().set_root(el.node);

    env.js_undefined()
}

unsafe extern fn js_init_element(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_doc, js_tag_name, js_el, ..] = env.args(cb_info);

    let tag_name = env.string(js_tag_name);
    let doc = env.unwrap::<RcDoc>(js_doc);
    let el = doc.borrow_mut().create_element(&tag_name);

    let js_el_ref = env.wrap(js_el, NodeRef::new(doc.clone(), el));
    doc.borrow_mut().set_expando(el, Some(js_el_ref));

    env.js_undefined()
}

unsafe extern fn js_set_attribute(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_el, js_att_name, js_att_value, ..] = env.args(cb_info);

    let el = env.unwrap::<NodeRef>(js_el);
    let att_name = env.string(js_att_name);
    let att_value = env.string(js_att_value);

    el.doc.borrow_mut().set_attribute(el.node, &att_name, &att_value);

    env.js_undefined()
}

unsafe extern fn js_insert_child_at(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_parent, js_child, js_index, ..] = env.args(cb_info);

    let parent = env.unwrap::<NodeRef>(js_parent);
    let child = env.unwrap::<NodeRef>(js_child);
    let index = env.i32(js_index) as usize;

    parent.doc.borrow_mut().insert_child(parent.node, index, child.node);

    env.js_undefined()
}

unsafe extern fn js_remove_child(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_parent, js_child, ..] = env.args(cb_info);

    let parent = env.unwrap::<NodeRef>(js_parent);
    let child = env.unwrap::<NodeRef>(js_child);

    parent.doc.borrow_mut().remove_child(parent.node, child.node);

    env.js_undefined()
}

unsafe extern fn js_init_text_node(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_doc, js_text, js_text_node, ..] = env.args(cb_info);

    let doc = env.unwrap::<RcDoc>(js_doc);
    let text = env.string(js_text);
    let text_node = doc.borrow_mut().create_text_node(&text);

    env.wrap(js_text_node, NodeRef::new(doc.clone(), text_node));

    env.js_undefined()
}

unsafe extern fn js_set_text(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [js_text_node, js_text, ..] = env.args(cb_info);

    let text_node = env.unwrap::<NodeRef>(js_text_node);
    let text = env.string(js_text);

    text_node.doc.borrow_mut().set_text(text_node.node, &text);

    env.js_undefined()
}


type RcDoc = Rc<RefCell<Document<Option<napi_ref>>>>;

struct NodeRef {
    doc: RcDoc,
    node: NodeId
}

impl NodeRef {
    fn new(doc: RcDoc, node: NodeId) -> Self {
        Self { doc, node }
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        self.doc.borrow_mut().free_node(self.node)
    }
}











/*


extern fn js_wait_events(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    // wait/poll depending on how far is the next "tick"
    let timeout_ms = match unsafe { uv_backend_timeout(uv_default_loop()) } {
        -1 => None,
        n => Some(n)
    };

    unsafe { crate::window::wait_events(timeout_ms) };

    env.js_undefined()
}

extern fn js_create_window(env: napi_env, cb_info: napi_callback_info) -> napi_value {
    let [title, width, height, ..] = env.args(cb_info);

    unsafe { crate::window::create_window(&env.string(title), env.i32(width), env.i32(height)) };

    env.js_undefined()
}

*/


// wait for I/O and awake the main thread which should in turn
// return back to node and handle it
//
// I think electron is doing something similar but their approach
// seems to be much more complicated (and maybe better)
//
// TODO: windows, linux
fn start_wakeup_thread() {
    std::thread::spawn(move || {
        let node_fd = unsafe { uv_backend_fd(uv_default_loop()) };
        assert_ne!(node_fd, -1, "couldnt get uv_loop fd");

        loop {
            let mut ev = unsafe { std::mem::zeroed::<kevent>() };

            match unsafe { kevent(node_fd, std::ptr::null(), 0, &mut ev, 1, null()) } {
                // shouldn't happen
                0 => eprintln!("kevent returned early"),

                -1 => {
                    eprintln!("kevent err");
                    return;
                }

                // something's pending (res is NOT number of pending events)
                _ => {
                    silly!("pending I/O, waking up UI thread");
                    unsafe { crate::window::wakeup() };

                    // let nodejs handle it first then we can wait again
                    std::thread::sleep(std::time::Duration::from_millis(100))
                }
            }
        }
    });

    extern {
      fn kevent(kq: c_int, changelist: *const kevent, nchanges: c_int, eventlist: *mut kevent, nevents: c_int, timeout: *const timespec) -> c_int;
    }

    #[repr(C)]
    struct kevent {
        pub ident: usize,
        pub filter: i16,
        pub flags: u16,
        pub fflags: u32,
        pub data: isize,
        pub udata: *mut c_void,
    }

    #[repr(C)]
    struct timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }
}











use std::ptr::{null, null_mut};
use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum napi_status {
    Ok,
    InvalidArg,
    ObjectExpected,
    StringExpected,
    NameExpected,
    FunctionExpected,
    NumberExpected,
    BooleanExpected,
    ArrayExpected,
    GenericFailure,
    PendingException,
    Cancelled,
    EscapeCalledTwice,
    HandleScopeMismatch,
}

type napi_value = *const c_void;
type napi_callback = unsafe extern "C" fn(napi_env, napi_callback_info) -> napi_value;
type napi_callback_info = *const c_void;
type napi_finalize = unsafe extern "C" fn(napi_env, *mut c_void, *mut c_void);
type napi_ref = *const c_void;

const NAPI_AUTO_LENGTH: usize = usize::max_value();

#[repr(C)]
#[derive(Clone, Copy)]
struct napi_env(*const c_void);

// call napi with uninitialized buffer, check status & return result
// it should be safe but putting unsafe around it would supress
// unsafe warnings for arg expressions too
macro_rules! get_res {
    ($env:expr, $napi_fn:ident $($arg:tt)*) => {{
        let mut res_value = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        let res = $napi_fn($env $($arg)*, &mut res_value);

        assert_eq!(res, napi_status::Ok);

        res_value
    }}
}

// most of here is safe but can panic
// the only notable exception are unwraps
impl napi_env {
    #[inline]
    fn i32(&self, v: napi_value) -> i32 {
        unsafe { get_res!(*self, napi_get_value_int32, v) }
    }

    // V8 strings can be encoded in many ways so we NEED to convert them
    // (https://stackoverflow.com/questions/40512393/understanding-string-heap-size-in-javascript-v8)
    fn string(&self, v: napi_value) -> String {
        unsafe {
            let len = get_res!(*self, napi_get_value_string_utf8, v, null_mut(), 0);

            // +1 because of \0
            let mut bytes = Vec::with_capacity(len + 1);
            get_res!(*self, napi_get_value_string_utf8, v, bytes.as_mut_ptr() as *mut c_char, len + 1);

            // (capacity vs len)
            bytes.set_len(len);

            String::from_utf8_unchecked(bytes)
        }
    }

    #[inline]
    fn js_undefined(&self) -> napi_value {
        unsafe { get_res!(*self, napi_get_undefined) }
    }

    fn js_fn(&self, f: napi_callback) -> napi_value {
        unsafe { get_res!(*self, napi_create_function, null(), NAPI_AUTO_LENGTH, f, null()) }
    }

    fn js_array(&self, values: impl IntoIterator<Item = napi_value>) -> napi_value {
        let js_arr = unsafe { get_res!(*self, napi_create_array) };

        for (i, it) in values.into_iter().enumerate() {
            unsafe { napi_set_element(*self, js_arr, i as u32, it); }
        }

        js_arr
    }

    fn wrap<T>(&self, js_object: napi_value, native_object: T) -> napi_ref {
        // generic, compiler will create impl for each type we pass to `wrap()`
        unsafe extern fn drop_wrap<T>(env: napi_env, data: *mut c_void, hint: *mut c_void) {
            Box::from_raw(data as *mut T);
        }

        unsafe { get_res!(*self, napi_wrap, js_object, Box::into_raw(Box::new(native_object)) as *const c_void, drop_wrap::<T>, null()) }
    }

    // unwraps are unsafe - any native extension can wrap something and pass it then
    // to our native part
    //
    // but maybe we can make calls to nativeApi private (symbols) and then we could
    // guarantee the data was created by us and maybe even use Box<dyn Any> but that
    // could also be over-engineering
    #[inline]
    unsafe fn unwrap<T>(&self, js_object: napi_value) -> &mut T {
        let ptr = unsafe { get_res!(*self, napi_unwrap, js_object) as *mut T };

        std::mem::transmute(ptr)
    }

    unsafe fn opt_unwrap<T>(&self, opt_js_object: napi_value) -> Option<&mut T> {
        if opt_js_object == self.js_undefined() {
            return None
        }

        Some(self.unwrap(opt_js_object))
    }

    fn ref_value(&self, js_ref: napi_ref) -> napi_value {
        unsafe { get_res!(*self, napi_get_reference_value, js_ref) }
    }

    // for simplicity, we always expect 3 args
    // (it's easy to leave any of them and hopefully 3 could be enough)
    fn args(&self, cb_info: napi_callback_info) -> [napi_value; 3] {
        unsafe {
            let mut argv = [std::mem::zeroed(); 3];
            let mut argc = argv.len();
            let mut this_arg = std::mem::zeroed();
            napi_get_cb_info(*self, cb_info, &mut argc, &mut argv[0], &mut this_arg, null_mut());

            argv
        }
    }

    fn set_prop(&self, target: napi_value, key: &str, value: napi_value) {
        assert_eq!(unsafe { napi_set_named_property(*self, target, c_str!(key), value) }, napi_status::Ok)
    }
}



dylib! {
    #[load_node_api]
    extern "C" {
        fn napi_module_register(module: *mut napi_module) -> napi_status;
        fn napi_set_named_property(env: napi_env, object: napi_value, utf8name: *const c_char, value: napi_value) -> napi_status;

        fn napi_get_undefined(env: napi_env, result: *mut napi_value) -> napi_status;
        fn napi_get_value_int32(env: napi_env, value: napi_value, result: *mut c_int) -> napi_status;
        fn napi_get_value_string_utf8(env: napi_env, value: napi_value, buf: *mut c_char, bufsize: usize, result: *mut usize) -> napi_status;

        fn napi_create_function(env: napi_env, utf8name: *const c_char, length: usize, cb: napi_callback, data: *const c_void, result: *mut napi_value) -> napi_status;
        fn napi_create_array(env: napi_env, result: *mut napi_value) -> napi_status;
        fn napi_set_element(env: napi_env, arr: napi_value, index: c_uint, value: napi_value) -> napi_status;

        fn napi_get_cb_info(env: napi_env, cb_info: napi_callback_info, argc: *mut usize, argv: *mut napi_value, this_arg: *mut napi_value, data: *mut c_void) -> napi_status;


        fn napi_wrap(env: napi_env, js_object: napi_value, native_object: *const c_void, finalize_cb: napi_finalize, finalize_hint: *const c_void, result: *mut napi_ref) -> napi_status;
        fn napi_unwrap(env: napi_env, value: napi_value, result: *mut *mut c_void) -> napi_status;
        fn napi_get_reference_value(env: napi_env, js_ref: napi_ref, result: *mut napi_value) -> napi_status;


        fn uv_default_loop() -> *const c_void;
        fn uv_backend_fd(uv_loop: *const c_void) -> c_int;
        fn uv_backend_timeout(uv_loop: *const c_void) -> c_int;


        fn napi_get_value_uint32(env: napi_env, napi_value: napi_value, result: *mut c_uint) -> napi_status;
        fn napi_get_value_double(env: napi_env, napi_value: napi_value, result: *mut f64) -> napi_status;
        fn napi_get_value_bool(env: napi_env, napi_value: napi_value, result: *mut bool) -> napi_status;

        fn napi_create_uint32(env: napi_env, value: c_uint, result: *mut napi_value) -> napi_status;
        fn napi_create_int32(env: napi_env, value: c_int, result: *mut napi_value) -> napi_status;
        fn napi_create_double(env: napi_env, value: f64, result: *mut napi_value) -> napi_status;
    }
}

#[repr(C)]
struct napi_module {
    nm_version: c_int,
    nm_flags: c_uint,
    nm_filename: *const c_char,
    nm_register_func: unsafe extern "C" fn(napi_env, napi_value) -> napi_value,
    nm_modname: *const c_char,
    nm_priv: *const c_void,
    reserved: [*const c_void; 4],
}

#[no_mangle]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static REGISTER_NODE_MODULE: unsafe extern "C" fn() = {
    static mut NAPI_MODULE: napi_module = napi_module {
        nm_version: 1,
        nm_flags: 0,
        nm_filename: c_str!("nodejs.rs"),
        nm_register_func: js_init_module,
        nm_modname: c_str!("libgraffiti"),
        nm_priv: null(),
        reserved: [null(); 4],
    };

    unsafe extern "C" fn register_node_module() {
        silly!("loading node api");
        load_node_api(if cfg!(target_os = "windows") { c_str!("node.exe") } else { null() });

        silly!("calling napi_module_register");
        napi_module_register(&mut NAPI_MODULE);
    }

    register_node_module
};
