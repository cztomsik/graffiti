use std::os::raw::{c_int, c_uint, c_void};
use std::ptr::{null, null_mut};

type NapiStatus = c_int;
type NapiEnv = *const c_void;
type NapiValue = *const c_void;
type NapiCallbackInfo = *const c_void;
type NapiCallback = unsafe extern "C" fn(NapiEnv, NapiCallbackInfo) -> NapiValue;
const NAPI_AUTO_LENGTH: usize = usize::max_value();

#[repr(C)]
struct NapiModule {
    nm_version: c_int,
    nm_flags: c_uint,
    nm_filename: *const u8,
    nm_register_func: unsafe extern "C" fn(NapiEnv, NapiValue) -> NapiValue,
    nm_modname: *const u8,
    nm_priv: *const c_void,
    reserved: [*const c_void; 4],
}

// see build.rs
extern "C" {
    fn napi_module_register(module: *mut NapiModule) -> NapiStatus;

    fn napi_get_null(env: NapiEnv, res: *mut NapiValue) -> NapiStatus;

    fn napi_create_function(
        env: NapiEnv,
        name: *const u8,
        len: usize,
        cb: NapiCallback,
        data: *const c_void,
        res: *mut NapiValue,
    ) -> NapiStatus;

    fn napi_get_cb_info(
        env: NapiEnv,
        cb_info: NapiCallbackInfo,
        argc: *mut usize,
        argv: *mut NapiValue,
        this_arg: *mut NapiValue,
        data: *mut c_void,
    ) -> NapiStatus;

    fn napi_get_arraybuffer_info(
        env: NapiEnv,
        buffer: NapiValue,
        data: *mut *const c_void,
        len: *mut usize,
    ) -> NapiStatus;

    fn napi_create_string_utf8(env: NapiEnv, str: *const u8, len: *mut usize, res: *mut NapiValue) -> NapiStatus;
}

unsafe extern "C" fn init_node_module(env: NapiEnv, _exports: NapiValue) -> NapiValue {
    let mut js_send = null();
    napi_create_function(
        env,
        b"send\0" as _,
        NAPI_AUTO_LENGTH,
        send_wrapper,
        null(),
        &mut js_send,
    );

    js_send
}

unsafe extern "C" fn send_wrapper(env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
    let (mut buffer, mut data, mut len, mut res) = (null(), null(), 0, null());
    napi_get_cb_info(env, cb_info, &mut 1, &mut buffer, null_mut(), null_mut());
    napi_get_arraybuffer_info(env, buffer, &mut data, &mut len);

    let json = crate::ffi::gft_send(data as _, len);

    if json == null() {
        napi_get_null(env, &mut res);
    } else {
        napi_create_string_utf8(env, json, NAPI_AUTO_LENGTH as _, &mut res);
    }

    // println!("{:?}", (buffer, data, len, res, json));

    res
}

#[no_mangle]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static REGISTER_NODE_MODULE: unsafe extern "C" fn() = {
    static mut NAPI_MODULE: NapiModule = NapiModule {
        nm_version: 1,
        nm_flags: 0,
        nm_filename: b"nodejs.rs\0" as _,
        nm_register_func: init_node_module, //js_init,
        nm_modname: b"libgraffiti\0" as _,
        nm_priv: null(),
        reserved: [null(); 4],
    };

    unsafe extern "C" fn register_node_module() {
        // proceed only if we are loaded from nodejs
        if !std::env::var("GFT_NODEJS").is_ok() {
            return;
        }

        napi_module_register(&mut NAPI_MODULE);
    }

    register_node_module
};
