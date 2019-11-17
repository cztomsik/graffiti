// node.js bindings

use crate::{Api, ApiMsg, init_api};
use std::os::raw::{c_int, c_uint, c_char, c_void};
use std::ptr;
use std::mem;

// note that special link args are needed (see /build.js)
extern "C" {
    fn napi_module_register(module: *mut NapiModule) -> NapiStatus;
    fn napi_get_undefined(env: NapiEnv, result: *mut NapiValue) -> NapiStatus;
    fn napi_set_named_property(env: NapiEnv, object: NapiValue, utf8name: *const c_char, value: NapiValue) -> NapiStatus;
    fn napi_create_function(env: NapiEnv, utf8name: *const c_char, length: usize, cb: NapiCallback, data: *const c_void, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_cb_info(env: NapiEnv, cb_info: NapiCallbackInfo, argc: *mut usize, argv: *mut NapiValue, this_arg: *mut NapiValue, data: *mut c_void) -> NapiStatus;
    fn napi_get_element(env: NapiEnv, napi_value: NapiValue, index: u32, result: *mut NapiValue) -> NapiStatus;
    fn napi_get_value_uint32(env: NapiEnv, napi_value: NapiValue, result: *mut u32) -> NapiStatus;
    fn napi_get_value_int32(env: NapiEnv, napi_value: NapiValue, result: *mut i32) -> NapiStatus;
    fn napi_get_value_double(env: NapiEnv, napi_value: NapiValue, result: *mut f64) -> NapiStatus;
    fn napi_get_value_bool(env: NapiEnv, napi_value: NapiValue, result: *mut bool) -> NapiStatus;
    fn napi_get_array_length(env: NapiEnv, napi_value: NapiValue, result: *mut u32) -> NapiStatus;
    fn napi_typeof(env: NapiEnv, napi_value: NapiValue, result: *mut NapiValueType) -> NapiStatus;
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
    silly!("init_node_module");

    API = Box::into_raw(Box::new(init_api()));
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

    (*API).send(argv[0].into());

    // TODO: res/events/...

    // TODO: could be static & shared?
    get_res!(napi_get_undefined)
}

static mut API: *mut Api = ptr::null_mut();
static mut ENV: NapiEnv = NapiEnv(ptr::null_mut());


// `Into` because of orphanage
impl <T> Into<Vec<T>> for NapiValue where T: From<NapiValue> {
    fn into(self) -> Vec<T> {
        let len = get_res!(napi_get_array_length, self);

        (0..len).map(|i| get_res!(napi_get_element, self, i).into()).collect()
    }
}

impl <T> Into<Option<T>> for NapiValue where T: From<NapiValue> {
    fn into(self) -> Option<T> {
        panic!("TODO")
    }
}

// TODO: color could fit in V8 smallint and maybe we dont need this then
impl From<NapiValue> for u8 {
    fn from(napi_value: NapiValue) -> u8 {
        get_res!(napi_get_value_uint32, napi_value) as u8
    }
}

impl From<NapiValue> for u32 {
    fn from(napi_value: NapiValue) -> u32 {
        get_res!(napi_get_value_uint32, napi_value)
    }
}

impl From<NapiValue> for usize {
    fn from(napi_value: NapiValue) -> usize {
        get_res!(napi_get_value_uint32, napi_value) as usize
    }
}

impl From<NapiValue> for i32 {
    fn from(napi_value: NapiValue) -> i32 {
        get_res!(napi_get_value_int32, napi_value)
    }
}

impl From<NapiValue> for f64 {
    fn from(napi_value: NapiValue) -> f64 {
        get_res!(napi_get_value_double, napi_value)
    }
}

impl From<NapiValue> for bool {
    fn from(napi_value: NapiValue) -> bool {
        get_res!(napi_get_value_bool, napi_value)
    }
}

// TODO: js only has doubles but we want f32 for GPU
// so somewhere it has to be converted but it shouldn't happen often
// and we probably shouldnt have this either
impl From<NapiValue> for f32 {
    fn from(napi_value: NapiValue) -> f32 {
        get_res!(napi_get_value_double, napi_value) as f32
    }
}

// impl. conversion between javascript and rust
// this is a bit like poorman's serde to interop with node
//
// - only named fields are supported
// - not a proc macro because this is simpler
//   - but it could generate TS too so maybe in future
//
// note that we dont know repetition index in expansion so
// we need to have a mutable variable for that purpose
macro_rules! interop {
    // js [a, b, ...] -> SomeRustType { a, b, ... }
    ($rust_type:ident [$($field:ident),+] $($rest:tt)*) => (
        impl From<NapiValue> for $rust_type {
            #[allow(unused_assignments)]
            fn from(napi_value: NapiValue) -> $rust_type {
                let mut i = 0;

                $(
                    let $field = get_res!(napi_get_element, napi_value, i).into();
                    i += 1;
                )*

                $rust_type { $($field),* }
            }
        }

        interop! { $($rest)* }
    );

    // tagged union
    // js [0, a, b, ...] -> SomeEnum::FirstVariant { a, b, ... }
    ($rust_type:ident { $($variant:tt { $($field:ident),* }),+ } $($rest:tt)*) => (
        impl From<NapiValue> for $rust_type {
            #[allow(unused_assignments)]
            fn from(napi_value: NapiValue) -> $rust_type {
                let mut i = 0;
                let mut variant_i = 0;

                let tag: u32 = get_res!(napi_get_element, napi_value, i).into();
                i += 1;

                $(
                    if tag == variant_i {
                        $(
                            let $field = get_res!(napi_get_element, napi_value, i).into();
                            i += 1;
                        )*

                        return $rust_type::$variant { $($field),* }
                    }
                    variant_i += 1;
                )*

                panic!("unknown variant {} for enum {}", tag, stringify!($rust_type))
            }
        }

        interop! { $($rest)* }
    );

    () => ();

}


use crate::commons::{Pos, Color, BoxShadow, Border, BorderSide, BorderRadius};
use crate::window::{SceneChange};
use crate::box_layout::{DimensionProp, Dimension, AlignProp, Align, FlexWrap, FlexDirection};

interop! {
    ApiMsg {
        CreateWindow { width, height },
        GetEvents { poll },
        UpdateScene { window, changes }
    }

    SceneChange {
        Alloc {},
        InsertAt { parent, child, index },
        RemoveChild { parent, child },

        Dimension { surface, prop, value },
        Align { surface, prop, value },
        FlexWrap { surface, value },
        FlexDirection { surface, value },

        BackgroundColor { surface, value },
        Border { surface, value },
        BoxShadow { surface, value },
        TextColor { surface, value },
        BorderRadius { surface, value },
        Image { surface, value },

        Text { surface, text }
    }

    Color [r, g, b, a]
    BorderRadius [top, right, bottom, left]
    Border [top, right, bottom, left]
    BorderSide [width, style, color]
    BoxShadow [color, offset, blur, spread]
    Pos [x, y]
}
