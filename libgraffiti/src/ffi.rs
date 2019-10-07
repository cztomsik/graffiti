// bridge

use crate::app::{TheApp, WindowId};
use crate::window::{Event, UpdateSceneMsg};
use miniserde::{json, Deserialize, Serialize};
use std::io::prelude::Write;

static mut APP: Option<TheApp> = None;

#[no_mangle]
pub extern "C" fn init() {
    unsafe { APP = Some(TheApp::init()) }
}

// returning the value would require javascript to copy it to the heap,
// we can avoid this simply by providing mutable ref to the already allocated
// (and possibly reused) memory
//
// we dont need ffi anymore but it might be useful for future targets
#[no_mangle]
pub unsafe extern "C" fn send(data: *const u8, len: size_t, res_data: *mut u8, res_maxlen: size_t) {
    // get slice of bytes & try to deserialize
    let msg = std::slice::from_raw_parts(data, len as usize);
    let msg: FfiMsg = json::from_str(std::str::from_utf8(msg).expect("not string")).expect("invalid message");

    silly!("Msg {:#?}", &msg);

    // try to handle the message
    let maybe_err = std::panic::catch_unwind(|| {
        match APP {
            None => panic!("no app"),
            Some(ref mut app) => handle_msg(app, &msg),
        }
    });

    let result = maybe_err.unwrap_or_else(|err| {
        let err = err
            .downcast::<String>()
            .unwrap_or(Box::new("Unknown".into()))
            .to_string();

        error!("err {:?}", err);

        FfiResult {
            events: Vec::new(),
            error: Some(err)
        }
    });

    let mut res_buf = std::slice::from_raw_parts_mut(res_data, res_maxlen);
    res_buf.write(json::to_string(&result).as_bytes()).expect("write result");
}

fn handle_msg(app: &mut TheApp, msg: &FfiMsg) -> FfiResult {
    // TODO: think more about windows, support closing

    let window_id = msg.window.unwrap_or_else(|| app.create_window());
    let events;

    // TODO: maybe we can both update and get events
    // but it would need some changes in js
    if let Some(update_msg) = &msg.update {
        app.update_window_scene(window_id, update_msg);
        events = Vec::new();
    } else {
        events = app.get_events(false);
    }

    FfiResult {
        events,
        error: None,
    }
}

// some ffi-specific glue

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiMsg {
    window: Option<WindowId>,
    update: Option<UpdateSceneMsg>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FfiResult {
    // TODO: multi-window
    events: Vec<Event>,
    error: Option<String>
}

// node-specific
// we can drop node-ffi dependency then

use libc::{c_int, c_uint, c_char, c_void, size_t};
use std::ptr;

// note that special link args are needed (see /build.js)
extern "C" {
    fn napi_module_register(module: *mut NapiModule);
    fn napi_get_undefined(env: NapiEnv, result: *mut NapiValue);
    fn napi_set_named_property(env: NapiEnv, object: NapiValue, utf8name: *const c_char, value: NapiValue);
    fn napi_create_function(env: NapiEnv, utf8name: *const c_char, length: size_t, cb: NapiCallback, data: *const c_void, result: *mut NapiValue);
    fn napi_get_cb_info(env: NapiEnv, cb_info: NapiCallbackInfo, argc: *mut size_t, argv: *mut NapiValue, this_arg: *mut NapiValue, data: *mut c_void);
    fn napi_get_buffer_info(env: NapiEnv, buf: NapiValue, data: *mut *mut c_void, len: *mut size_t);
}

#[repr(C)]
pub struct NapiModule {
    nm_version: c_int,
    nm_flags: c_uint,
    nm_filename: *const c_char,
    nm_register_func: unsafe extern "C" fn(NapiEnv, NapiValue) -> NapiValue,
    nm_modname: *const c_char,
    nm_priv: *const c_void,
    reserved: [*const c_void; 4],
}

pub type NapiCallback = unsafe extern "C" fn(NapiEnv, NapiCallbackInfo) -> NapiValue;
const NAPI_AUTO_LENGTH: size_t = size_t::max_value();

// opque types
#[derive(Clone, Copy)] #[repr(C)] pub struct NapiValue(*const c_void);
#[derive(Clone, Copy)] #[repr(C)] pub struct NapiEnv(*const c_void);
#[repr(C)] pub struct NapiCallbackInfo(*const c_void);

#[no_mangle]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static REGISTER_NODE_MODULE: unsafe extern "C" fn() = {
    static mut NAPI_MODULE: Option<NapiModule> = None;

    unsafe extern "C" fn register_node_module() {
        silly!("register_node_module");

        NAPI_MODULE = Some(NapiModule {
            nm_version: 1,
            nm_flags: 0,
            nm_filename: c_str!(file!()),
            nm_register_func: init_node_module,
            nm_modname: c_str!("libgraffiti"),
            nm_priv: ptr::null(),
            reserved: [ptr::null(); 4]
        });

        napi_module_register(NAPI_MODULE.as_mut().unwrap() as *mut NapiModule);
    }

    register_node_module    
};

unsafe extern "C" fn init_node_module(env: NapiEnv, exports: NapiValue) -> NapiValue {
    silly!("init_node_module");

    init();

    let mut method = std::mem::uninitialized();
    napi_create_function(env, c_str!("libgraffitiSend"), NAPI_AUTO_LENGTH, send_wrapper, ptr::null(), &mut method);
    napi_set_named_property(env, exports, c_str!("nativeSend"), method);

    exports
}

unsafe extern "C" fn send_wrapper(env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
    let mut argc = 2;
    let mut argv = [std::mem::uninitialized(); 2];
    let mut this_arg = std::mem::uninitialized();
    napi_get_cb_info(env, cb_info, &mut argc, &mut argv[0], &mut this_arg, ptr::null_mut());

    let mut msg_data = ptr::null_mut();
    let mut msg_len = 0;
    napi_get_buffer_info(env, argv[0], &mut msg_data, &mut msg_len);

    let mut res_data = ptr::null_mut();
    let mut res_maxlen = 0;
    napi_get_buffer_info(env, argv[1], &mut res_data, &mut res_maxlen);

    send(std::mem::transmute(msg_data), msg_len, std::mem::transmute(res_data), res_maxlen);

    let mut undefined = std::mem::uninitialized();
    napi_get_undefined(env, &mut undefined);

    undefined
}
