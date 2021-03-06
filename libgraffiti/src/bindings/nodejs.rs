use crate::util::Dylib;
use napi::*;
use std::ptr::{null, null_mut};

macro_rules! check {
    ($body:expr) => {
        assert_eq!($body, NapiStatus::Ok)
    };
}

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
                    extern "C" fn fun(env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
                        $fn.call_napi(env, cb_info)
                    }

                    let mut val = std::mem::zeroed();
                    check!(napi_create_function(env, null(), NAPI_AUTO_LENGTH, fun, null(), &mut val));
                    check!(napi_set_named_property(env, exports, c_str!($name), val));
                }};
            };

            $($inner)*

            exports
        }
    };
}

include!("api.rs");

pub trait FromNapi {
    fn from_napi(env: NapiEnv, napi_value: NapiValue) -> Self;
    fn to_napi(&self, env: NapiEnv) -> NapiValue;
}

macro_rules! impl_from_napi {
    ($type:ty, $from:expr, $to:expr) => {
        impl FromNapi for $type {
            fn from_napi(env: NapiEnv, napi_value: NapiValue) -> Self {
                let mut val = Default::default();
                unsafe { check!($from(env, napi_value, &mut val)) }
                val
            }

            fn to_napi(&self, env: NapiEnv) -> NapiValue {
                unsafe {
                    let mut res = std::mem::zeroed();
                    check!($to(env, *self, &mut res));
                    res
                }
            }
        }
    };
}

impl_from_napi!((), |_, _, _| NapiStatus::Ok, |env, _, res| napi_get_undefined(env, res));
impl_from_napi!(bool, napi_get_value_bool, napi_get_boolean);
impl_from_napi!(u32, napi_get_value_uint32, napi_create_uint32);
impl_from_napi!(i32, napi_get_value_int32, napi_create_int32);
impl_from_napi!(f64, napi_get_value_double, napi_create_double);

// &str results needs .to_owned() and closures need to accept String but it's not that common
impl FromNapi for String {
    fn from_napi(env: NapiEnv, napi_value: NapiValue) -> Self {
        unsafe {
            let mut len = Default::default();
            check!(napi_get_value_string_utf8(env, napi_value, null_mut(), 0, &mut len));

            // +1 because of \0
            let mut bytes = Vec::with_capacity(len + 1);
            bytes.set_len(len);
            check!(napi_get_value_string_utf8(
                env,
                napi_value,
                bytes.as_mut_ptr() as *mut _,
                len + 1,
                null_mut()
            ));

            String::from_utf8_unchecked(bytes)
        }
    }

    fn to_napi(&self, env: NapiEnv) -> NapiValue {
        unsafe {
            let mut res = std::mem::zeroed();
            check!(napi_create_string_utf8(
                env,
                self.as_ptr() as *const _,
                self.len(),
                &mut res
            ));
            res
        }
    }
}

impl<T: FromNapi + Clone> FromNapi for Vec<T> {
    fn from_napi(env: NapiEnv, arr: NapiValue) -> Self {
        unsafe {
            let mut len = 0;
            check!(napi_get_array_length(env, arr, &mut len));

            (0..len)
                .map(|i| {
                    let mut v = std::mem::zeroed();
                    check!(napi_get_element(env, arr, i, &mut v));

                    T::from_napi(env, v)
                })
                .collect()
        }
    }

    fn to_napi(&self, env: NapiEnv) -> NapiValue {
        unsafe {
            let mut arr = std::mem::zeroed();
            check!(napi_create_array(env, &mut arr));

            for (i, v) in self.iter().enumerate() {
                check!(napi_set_element(env, arr, i as _, v.to_napi(env)));
            }

            arr
        }
    }
}

// any Fn(A1, A2, ...) -> R can be used as napi callback if values are convertible
// generic because we need to bind arg types somewhere
pub trait NapiCallable<P> {
    fn call_napi(&self, env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue;
}

macro_rules! impl_callable {
    ($len:literal $(, $param:ident)*) => {
        #[allow(unused_parens)]
        impl <$($param,)* R, F> NapiCallable<($(&$param),*)> for F
        where $($param: FromNapi,)* R: FromNapi, F: Fn($($param),*) -> R {
            #[allow(unconditional_panic, non_snake_case)]
            fn call_napi(&self, env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
                unsafe {
                    let mut argv = [std::mem::zeroed(); $len];
                    napi_get_cb_info(env, cb_info, &mut $len, argv.as_mut_ptr(), null_mut(), null_mut());
                    let [$($param),*] = argv;

                    self($($param::from_napi(env, $param)),*).to_napi(env)
                }
            }
        }
    }
}

impl_callable!(0);
impl_callable!(1, A1);
impl_callable!(2, A1, A2);
impl_callable!(3, A1, A2, A3);
impl_callable!(4, A1, A2, A3, A4);

// headers
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
            pub fn napi_get_value_bool(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;

            pub fn napi_create_uint32(env: NapiEnv, value: c_uint, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_value_uint32(env: NapiEnv, value: NapiValue, result: *mut c_uint) -> NapiStatus;

            pub fn napi_create_int32(env: NapiEnv, value: c_int, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_value_int32(env: NapiEnv, value: NapiValue, result: *mut c_int) -> NapiStatus;

            pub fn napi_create_double(env: NapiEnv, value: c_double, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_value_double(env: NapiEnv, value: NapiValue, result: *mut c_double) -> NapiStatus;

            pub fn napi_create_string_utf8(env: NapiEnv, buf: *const c_char, len: usize, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_value_string_utf8(env: NapiEnv, value: NapiValue, buf: *mut c_char, bufsize: usize, result: *mut usize) -> NapiStatus;

            pub fn napi_create_array(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_array_length(env: NapiEnv, napi_value: NapiValue, result: *mut c_uint) -> NapiStatus;
            pub fn napi_get_element(env: NapiEnv, arr: NapiValue, index: c_uint, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_set_element(env: NapiEnv, arr: NapiValue, index: c_uint, value: NapiValue) -> NapiStatus;

            pub fn napi_create_function(env: NapiEnv, utf8name: *const c_char, length: usize, cb: NapiCallback, data: *const c_void, result: *mut NapiValue) -> NapiStatus;
            pub fn napi_get_cb_info(env: NapiEnv, cb_info: NapiCallbackInfo, argc: *mut usize, argv: *mut NapiValue, this_arg: *mut NapiValue, data: *mut c_void) -> NapiStatus;
        }
    }
}
