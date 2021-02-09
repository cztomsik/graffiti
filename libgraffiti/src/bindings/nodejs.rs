use crate::util::Dylib;
use napi::*;
use std::ptr::{null, null_mut};

macro_rules! js_module {
    ($($inner:tt)*) => {
        // napi module needs to be registered when lib is loaded
        init! {
            // proceed only if we are loaded from nodejs
            if !std::env::var("GFT_NODEJS").is_ok() {
                return
            }

            // needs to be static
            static mut NAPI_MODULE: NapiModule = NapiModule {
                nm_version: 1,
                nm_flags: 0,
                nm_filename: c_str!("nodejs.rs"),
                nm_register_func: js_init_module,
                nm_modname: c_str!("libgraffiti"),
                nm_priv: null(),
                reserved: [null(); 4],
            };

            unsafe {
                // load from current node process (we are dylib)
                let node = Dylib::load(if cfg!(target_os = "windows") { c_str!("node.exe") } else { null() });
                napi::load_with(|s| node.symbol(*c_str!(s)));

                // register & call js_init_module (below)
                napi_module_register(&mut NAPI_MODULE);
            }
        }

        unsafe extern "C" fn js_init_module(env: NapiEnv, exports: NapiValue) -> NapiValue {
            // TODO: js_const

            macro_rules! js_fn {
                ($name:literal, $fn:expr) => {{
                    unsafe extern "C" fn fun(env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
                        // TODO: 4 is good for now
                        let mut argv = [std::mem::zeroed(); 4];
                        let mut argc = argv.len();
                        let mut this_arg = std::mem::zeroed();
                        napi_get_cb_info(env, cb_info, &mut argc, &mut argv[0], &mut this_arg, null_mut());

                        println!("TODO: {}", stringify!($name));

                        let mut res = std::mem::zeroed();
                        napi_get_undefined(env, &mut res);

                        res
                    }

                    let mut val = std::mem::zeroed();
                    assert_eq!(napi_create_function(env, null(), NAPI_AUTO_LENGTH, fun, null(), &mut val), NapiStatus::Ok);
                    napi_set_named_property(env, exports, c_str!($name), val);
                }};
            };

            $($inner)*

            exports
        }
    };
}

include!("shared.rs");

mod napi {
    use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct NapiEnv(*const c_void);

    #[repr(C)]
    #[derive(Debug, PartialEq)]
    #[allow(unused)]
    pub enum NapiStatus {
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

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct NapiValue(*const c_void);

    #[repr(C)]
    pub struct NapiModule {
        pub nm_version: c_int,
        pub nm_flags: c_uint,
        pub nm_filename: *const c_char,
        pub nm_register_func: unsafe extern "C" fn(NapiEnv, NapiValue) -> NapiValue,
        pub nm_modname: *const c_char,
        pub nm_priv: *const c_void,
        pub reserved: [*const c_void; 4],
    }

    pub const NAPI_AUTO_LENGTH: usize = usize::max_value();

    #[repr(C)]
    pub struct NapiCallbackInfo(*const c_void);

    pub type NapiCallback = unsafe extern "C" fn(NapiEnv, NapiCallbackInfo) -> NapiValue;

    dylib! {
        extern "C" {
            pub fn napi_module_register(module: *mut NapiModule) -> NapiStatus;
            pub fn napi_set_named_property(env: NapiEnv, object: NapiValue, utf8name: *const c_char, value: NapiValue) -> NapiStatus;

            pub fn napi_get_undefined(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_boolean(env: NapiEnv, value: bool, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_create_uint32(env: NapiEnv, value: c_uint, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_create_int32(env: NapiEnv, value: c_int, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_create_double(env: NapiEnv, value: c_double, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_create_string_utf8(env: NapiEnv, buf: *const c_char, len: usize, result: *mut NapiValue) -> NapiStatus;

            pub fn napi_create_function(env: NapiEnv, utf8name: *const c_char, length: usize, cb: NapiCallback, data: *const c_void, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_cb_info(env: NapiEnv, cb_info: NapiCallbackInfo, argc: *mut usize, argv: *mut NapiValue, this_arg: *mut NapiValue, data: *mut c_void) -> NapiStatus;

            pub fn napi_get_value_bool(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;
            pub fn napi_get_value_uint32(env: NapiEnv, value: NapiValue, result: *mut c_uint) -> NapiStatus;
            pub fn napi_get_value_int32(env: NapiEnv, value: NapiValue, result: *mut c_int) -> NapiStatus;
            pub fn napi_get_value_double(env: NapiEnv, value: NapiValue, result: *mut c_double) -> NapiStatus;
            pub fn napi_get_value_string_utf8(env: NapiEnv, value: NapiValue, buf: *mut c_char, bufsize: usize, result: *mut usize) -> NapiStatus;
        }
    }
}
