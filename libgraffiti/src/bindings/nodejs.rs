use crate::util::Dylib;
use std::ptr::null;
use napi::*;

init! {
    // proceed only if we are loaded from nodejs
    if !std::env::var("GFT_NODEJS").is_ok() {
        return
    }

    unsafe {
        // load from current node process (we are dylib)
        let node = Dylib::load(if cfg!(target_os = "windows") { c_str!("node.exe") } else { null() });
        napi::load_with(|s| node.symbol(*c_str!(s)));
    
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
    
        napi_module_register(&mut NAPI_MODULE);
    }
}

unsafe extern fn js_init_module(env: NapiEnv, exports: NapiValue) -> NapiValue {
    println!("TODO: populate exports");

    exports
}

mod napi {
    use std::os::raw::{c_char, c_int, c_uint, c_void};

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

    pub type NapiValue = *const c_void;

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

    dylib! {
        extern "C" {
            pub fn napi_module_register(module: *mut NapiModule) -> NapiStatus;

        }
    }
}
