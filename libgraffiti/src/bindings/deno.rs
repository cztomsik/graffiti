// deno bindings

use deno_unstable_api::*;
use nanoserde::{DeJson, DeJsonErr, SerJson};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    dbg!();

    macro_rules! export {
        ($($name:ident : $fn:expr),*) => {{
            $(
                fn $name(_: &mut dyn Interface, bufs: &mut [V8Buf]) -> Op { $fn.call_deno(bufs) }
                interface.register_op(concat!("GFT_", stringify!($name)), $name);
            )*
        }}
    }

    export_api!()
}

trait DenoCallable<P> {
    fn call_deno(&self, bufs: &mut [V8Buf]) -> Op;
}

macro_rules! impl_callable {
    (@args $bufs:ident $($param:ident,)*) => {{
        let json = std::str::from_utf8(&$bufs[0]).expect("invalid utf-8");
        <($($param,)*)>::deserialize_json(json).expect("invalid json")
    }};

    ($($param:ident),*) => {
        // Fn(A1: DeJson, ...) -> ()
        #[allow(unused, non_snake_case)]
        impl <$($param,)* F> DenoCallable<(() $(, &$param)*)> for F
        where F: Fn($($param),*), $($param: DeJson,)* {
            fn call_deno(&self, bufs: &mut [V8Buf]) -> Op {
                let ($($param,)*) = impl_callable!(@args bufs $($param,)*);
                self($($param),*);

                // this should not alloc
                Op::Sync(Box::new([]))
            }
        }

        // Fn(A1: DeJson, ...) -> R: SerJson
        #[allow(unused, non_snake_case)]
        impl <$($param,)* R, F> DenoCallable<(&R $(, &$param)*)> for F
        where F: Fn($($param),*) -> R, $($param: DeJson,)* R: SerJson {
            fn call_deno(&self, bufs: &mut [V8Buf]) -> Op {
                let ($($param,)*) = impl_callable!(@args bufs $($param,)*);

                Op::Sync(SerJson::serialize_json(&self($($param),*)).into_bytes().into_boxed_slice())
            }
        }
    }
}

// extends nanoserde, works only for the top level and even that is miracle
// (hopefully it's not a bug in compiler because then I don't know how it could be done)
trait DeJsonExt: Sized {
    fn deserialize_json(_: &str) -> Result<Self, DeJsonErr>;
}

// () is not supported at all
impl DeJsonExt for () {
    fn deserialize_json(_: &str) -> Result<Self, DeJsonErr> {
        Ok(())
    }
}

// (T,) is not supported so we skip initial [ and parse it as T
impl<T: DeJson> DeJsonExt for (T,) {
    fn deserialize_json(input: &str) -> Result<Self, DeJsonErr> {
        T::deserialize_json(&input[1..]).map(|v| (v,))
    }
}

impl_callable!();
impl_callable!(A1);
impl_callable!(A1, A2);
impl_callable!(A1, A2, A3);
impl_callable!(A1, A2, A3, A4);

// subset of deno plugin api so we don't need to compile whole deno
// note this is not safe nor ABI stable between different versions
#[allow(dead_code)]
mod deno_unstable_api {
    pub trait Interface {
        fn register_op(&mut self, name: &str, handler: fn(&mut dyn Interface, &mut [V8Buf]) -> Op);
    }

    pub type OpAsyncFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Box<[u8]>>>>;

    pub enum Op {
        Sync(Box<[u8]>),
        Async(OpAsyncFuture),
        AsyncUnref(OpAsyncFuture),
        NotFound,
    }

    pub struct V8Buf {
        data_ptr_ptr: *const *mut u8,
        control_ptr: usize,
        offset: usize,
        len: usize,
    }

    impl core::ops::Deref for V8Buf {
        type Target = [u8];

        fn deref(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts((*self.data_ptr_ptr).add(self.offset), self.len) }
        }
    }
}
