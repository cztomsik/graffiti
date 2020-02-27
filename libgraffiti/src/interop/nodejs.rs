// node.js bindings

#![allow(dead_code)]

use crate::app::{App};
use crate::interop::{InteropApi};
use std::os::raw::{c_int, c_uint, c_char, c_void};
use std::ptr;
use std::mem;

dylib! {
    #[load_napi]
    extern "C" {
        fn napi_module_register(module: *mut NapiModule) -> NapiStatus;
        fn napi_get_undefined(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
        fn napi_set_named_property(env: NapiEnv, object: NapiValue, utf8name: *const c_char, value: NapiValue) -> NapiStatus;
        fn napi_create_function(env: NapiEnv, utf8name: *const c_char, length: usize, cb: NapiCallback, data: *const c_void, result: *mut NapiValue) -> NapiStatus;
        fn napi_get_cb_info(env: NapiEnv, cb_info: NapiCallbackInfo, argc: *mut usize, argv: *mut NapiValue, this_arg: *mut NapiValue, data: *mut c_void) -> NapiStatus;
        fn napi_get_element(env: NapiEnv, arr: NapiValue, index: u32, result: *mut NapiValue) -> NapiStatus;
        fn napi_set_element(env: NapiEnv, arr: NapiValue, index: u32, value: NapiValue) -> NapiStatus;
        fn napi_get_value_uint32(env: NapiEnv, napi_value: NapiValue, result: *mut u32) -> NapiStatus;
        fn napi_get_value_int32(env: NapiEnv, napi_value: NapiValue, result: *mut i32) -> NapiStatus;
        fn napi_get_value_double(env: NapiEnv, napi_value: NapiValue, result: *mut f64) -> NapiStatus;
        fn napi_get_value_bool(env: NapiEnv, napi_value: NapiValue, result: *mut bool) -> NapiStatus;
        fn napi_get_array_length(env: NapiEnv, napi_value: NapiValue, result: *mut u32) -> NapiStatus;
        fn napi_get_value_string_utf8(env: NapiEnv, napi_value: NapiValue, buf: *mut c_char, bufsize: usize, result: *mut usize) -> NapiStatus;
        fn napi_typeof(env: NapiEnv, napi_value: NapiValue, result: *mut NapiValueType) -> NapiStatus;
        fn napi_create_uint32(env: NapiEnv, value: u32, result: *mut NapiValue) -> NapiStatus;
        fn napi_create_int32(env: NapiEnv, value: i32, result: *mut NapiValue) -> NapiStatus;
        fn napi_create_double(env: NapiEnv, value: f64, result: *mut NapiValue) -> NapiStatus;
        fn napi_get_boolean(env: NapiEnv, value: bool, result: *mut NapiValue) -> NapiStatus;
        fn napi_create_array(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum NapiValueType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Function,
    External,
    Bigint,
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
const NAPI_AUTO_LENGTH: usize = usize::max_value();

// opaque types
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
        silly!("loading napi");

        // can't use ternary because of c_str!
        #[cfg(target_family = "unix")]
        load_napi(std::ptr::null());
        #[cfg(target_os = "windows")]
        load_napi(c_str!("node.exe"));

        NAPI_MODULE = Some(NapiModule {
            nm_version: 1,
            nm_flags: 0,
            nm_filename: c_str!(file!()),
            nm_register_func: init_node_module,
            nm_modname: c_str!("libgraffiti"),
            nm_priv: ptr::null(),
            reserved: [ptr::null(); 4]
        });

        silly!("registering napi module");
        napi_module_register(NAPI_MODULE.as_mut().unwrap() as *mut NapiModule);
    }

    register_node_module    
};

// - call napi fn with env & uninitialized mem space for the result
// - check if it was ok
// - return the result
macro_rules! get_res {
    ($napi_fn:ident $($arg:tt)*) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let mut res_value = mem::MaybeUninit::uninit().assume_init();
            let res = $napi_fn(ENV $($arg)*, &mut res_value);

            assert_eq!(res, NapiStatus::Ok);

            res_value
        }
    }}
}

unsafe extern "C" fn init_node_module(env: NapiEnv, exports: NapiValue) -> NapiValue {
    silly!("initializing app");

    APP = Box::into_raw(Box::new(App::init()));
    ENV = env;

    let method = get_res!(napi_create_function, c_str!("libgraffitiSend"), NAPI_AUTO_LENGTH, send_wrapper, ptr::null());
    napi_set_named_property(env, exports, c_str!("nativeSend"), method);

    exports
}

unsafe extern "C" fn send_wrapper(env: NapiEnv, cb_info: NapiCallbackInfo) -> NapiValue {
    // get args
    let mut argc = 2;
    let mut argv = [mem::MaybeUninit::uninit().assume_init(); 2];
    let mut this_arg = mem::MaybeUninit::uninit().assume_init();
    napi_get_cb_info(env, cb_info, &mut argc, &mut argv[0], &mut this_arg, ptr::null_mut());

    ENV = env;

    let msg = argv[0].into2();
    debug!("msg {:?}", &msg);
    
    (*APP).send(msg).into()
}

static mut APP: *mut App = ptr::null_mut();
static mut ENV: NapiEnv = NapiEnv(ptr::null_mut());

// hack our own From, Into traits
// because of orphaning and also because of conflicting trait impl
// (somebody put T -> Option<T> conversion to the stdlib)
trait FromNapi {
    fn from(napi_value: NapiValue) -> Self;
}

trait Into2<T> {
    fn into2(self) -> T;
}

impl <T> Into2<T> for NapiValue where T: FromNapi {
    fn into2(self) -> T {
        FromNapi::from(self)
    }
}

impl <T> FromNapi for Vec<T> where T: FromNapi {
    fn from(napi_value: NapiValue) -> Self {
        let len = get_res!(napi_get_array_length, napi_value);

        (0..len).map(|i| get_res!(napi_get_element, napi_value, i).into2()).collect()
    }
}

impl <T> From<Vec<T>> for NapiValue where T: Into<NapiValue> + Clone {
    fn from(value: Vec<T>) -> Self {
        value.iter().map(|v| v.clone().into()).collect()
    }
}

// Iter<NapiValue> -> js array
// &[v.into(), v.into()].iter().collect() returns NapiValue too
impl std::iter::FromIterator<NapiValue> for NapiValue {
    fn from_iter<I: IntoIterator<Item=NapiValue>>(iter: I) -> Self {
        let arr = get_res!(napi_create_array);

        for (i, it) in iter.into_iter().enumerate() {
            unsafe { napi_set_element(ENV, arr, i as u32, it); }
        }

        arr
    }
}

impl <T> FromNapi for Option<T> where T: FromNapi {
    fn from(napi_value: NapiValue) -> Self {
        match get_res!(napi_typeof, napi_value) {
            NapiValueType::Undefined => None,
            _ => Some(napi_value.into2())
        }
    }
}

impl <T> From<Option<T>> for NapiValue where T: Into<NapiValue> {
    fn from(value: Option<T>) -> Self {
        match value {
            None => get_res!(napi_get_undefined),
            Some(v) => v.into()
        }
    }
}

impl From<f32> for NapiValue {
    fn from(value: f32) -> Self {
        (value as f64).into()
    }
}

impl From<f64> for NapiValue {
    fn from(value: f64) -> Self {
        get_res!(napi_create_double, value)
    }
}

impl From<String> for NapiValue {
    fn from(_value: String) -> Self {
        panic!("TODO");
    }
}

impl From<bool> for NapiValue {
    fn from(value: bool) -> Self {
        get_res!(napi_get_boolean, value)
    }
}

impl From<u8> for NapiValue {
    fn from(value: u8) -> Self {
        (value as u32).into()
    }
}

impl From<u16> for NapiValue {
    fn from(value: u16) -> Self {
        (value as u32).into()
    }
}

impl From<usize> for NapiValue {
    fn from(value: usize) -> Self {
        (value as u32).into()
    }
}

impl From<u32> for NapiValue {
    fn from(value: u32) -> Self {
        get_res!(napi_create_uint32, value as u32)
    }
}

impl From<i32> for NapiValue {
    fn from(value: i32) -> Self {
        get_res!(napi_create_int32, value as i32)
    }
}

// TODO: color could fit in V8 smallint and maybe we dont need this then
impl FromNapi for u8 {
    fn from(napi_value: NapiValue) -> Self {
        let v: u32 = napi_value.into2();
        v as u8
    }
}

impl FromNapi for u16 {
    fn from(napi_value: NapiValue) -> Self {
        let v: u32 = napi_value.into2();
        v as u16
    }
}

impl FromNapi for usize {
    fn from(napi_value: NapiValue) -> Self {
        let v: u32 = napi_value.into2();
        v as usize
    }
}

impl FromNapi for u32 {
    fn from(napi_value: NapiValue) -> Self {
        get_res!(napi_get_value_uint32, napi_value)
    }
}

impl FromNapi for i32 {
    fn from(napi_value: NapiValue) -> Self {
        get_res!(napi_get_value_int32, napi_value)
    }
}

// TODO: js only has doubles but we want f32 for GPU
// so somewhere it has to be converted but it shouldn't happen often
// and we probably shouldnt have this either
impl FromNapi for f32 {
    fn from(napi_value: NapiValue) -> Self {
        let v: f64 = napi_value.into2();
        v as f32
    }
}

impl FromNapi for f64 {
    fn from(napi_value: NapiValue) -> Self {
        get_res!(napi_get_value_double, napi_value)
    }
}

impl FromNapi for bool {
    fn from(napi_value: NapiValue) -> Self {
        get_res!(napi_get_value_bool, napi_value)
    }
}

// V8 strings can be encoded in many ways so we NEED to convert them
// (https://stackoverflow.com/questions/40512393/understanding-string-heap-size-in-javascript-v8)
//
// TODO: for text we only need Vec<char> so maybe there's a better way
impl FromNapi for String {
    fn from(napi_value: NapiValue) -> Self {
        // +1 because of NULL-termination
        let bufsize = get_res!(napi_get_value_string_utf8, napi_value, ptr::null_mut(), 0) + 1;

        let mut bytes = Vec::with_capacity(bufsize);
        let written = get_res!(napi_get_value_string_utf8, napi_value, bytes.as_mut_ptr() as *mut c_char, bufsize);

        unsafe { 
            bytes.set_len(written);

            String::from_utf8_unchecked(bytes)
        }
    }
}

// impl. conversion between javascript and rust
// this is a bit like poorman's serde to interop with node
//
// - only named fields are supported
// - not a proc macro because this is simpler
//
// note that we dont know repetition index in expansion so
// we need to have a mutable variable for that purpose
// (but I think compiler should be able to pre-evaluate this)
macro_rules! interop {
    // js number -> #[repr(u8)] SomeEnum
    ($rust_type:ident(u8) $($rest:tt)*) => (
        impl FromNapi for $rust_type {
            fn from(napi_value: NapiValue) -> Self {
                let num = get_res!(napi_get_value_uint32, napi_value) as u8;

                unsafe { std::mem::transmute(num) }
            }
        }

        impl From<$rust_type> for NapiValue {
            fn from(value: $rust_type) -> Self {
                get_res!(napi_create_uint32, unsafe { value as u32 })
            }
        }

        interop! { $($rest)* }
    );

    // js [a, b, ...] -> SomeRustType { a, b, ... }
    ($rust_type:ident [$($field:ident),+] $($rest:tt)*) => (
        impl FromNapi for $rust_type {
            #[allow(unused_assignments)]
            fn from(napi_value: NapiValue) -> Self {
                let mut i = 0;

                $(
                    let $field = get_res!(napi_get_element, napi_value, i).into2();
                    i += 1;
                )*

                $rust_type { $($field),* }
            }
        }

        impl From<$rust_type> for NapiValue {
            #[allow(unused_assignments)]
            fn from(value: $rust_type) -> Self {
                let slice: Vec<NapiValue> = vec![
                    $(
                        value.$field.into()
                    ),*
                ];
                slice.into_iter().collect()
            }
        }

        interop! { $($rest)* }
    );

    // tagged union
    // js [0, a, b, ...] -> SomeEnum::FirstVariant { a, b, ... }
    ($rust_type:ident { $($variant_i:literal $variant:tt { $($field:ident),* }),+ } $($rest:tt)*) => (
        impl FromNapi for $rust_type {
            #[allow(unused_assignments)]
            fn from(napi_value: NapiValue) -> Self {
                let tag: u32 = get_res!(napi_get_element, napi_value, 0).into2();
                let mut i = 1;

                match tag {
                    $(
                        $variant_i => {
                            $(
                                let $field = get_res!(napi_get_element, napi_value, i).into2();
                                i += 1;
                            )*

                            $rust_type::$variant { $($field),* }
                        }
                    )*
                    _ => panic!("unknown variant {} for enum {}", tag, stringify!($rust_type))
                }
            }
        }

        // TODO: we have to clone because of String
        // but maybe we could define all traits on refs to avoid extra cloning
        // (From<&MyEnum>, From<&String>, From<&u32>, ...)
        impl From<$rust_type> for NapiValue {
            #[allow(unused_assignments)]
            #[allow(unused_unsafe)]
            fn from(value: $rust_type) -> Self {
                unsafe {
                    match value {
                        $(
                            $rust_type::$variant { $($field),* } => {
                                let slice: Vec<NapiValue> = vec![
                                    $variant_i.into(),
                                    $(
                                        $field.into()
                                    ),*
                                ];
                                slice.into_iter().collect()
                            }
                        )*
                    }
                }
            }
        }

        interop! { $($rest)* }
    );

    () => ();

}

include!("generated.rs");
