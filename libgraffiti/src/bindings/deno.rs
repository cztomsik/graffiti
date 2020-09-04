// TODO: there was segfault AFTER the end of the script (but it's fine now so it could be something with glfw)

use makepad_microserde::{DeJson, SerJson};
use deno_unstable_api::*;
use super::API;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    println!("deno_plugin_init");

    // TODO: SerBin (+ encode/decode in JS)
    // TODO: reuse/rewrite same buffer for many bin messages
    // TODO: for ops with fixed result size, response can be written back in the buffer (to avoid alloc)
    macro_rules! json_op {
        ($fun: expr) => {
            |_, bufs| {
                let fun = $fun;
                let json = std::str::from_utf8(&bufs[0]).expect("invalid utf-8");
                let input = DeJson::deserialize_json(json).expect("invalid json");
                let res: Option<_> = fun(input).into();
                let bytes = SerJson::serialize_json(&res).into_bytes();

                Op::Sync(bytes.into_boxed_slice())
            }
        }
    }

    interface.register_op("GFT_CREATE_WINDOW", json_op!(|(title, width, height): (String, _, _)| API.create_window(&title, width, height)));
    interface.register_op("GFT_UPDATE_WINDOW_DOCUMENT", json_op!(|(window, changes): (_, Vec<_>)| { API.update_window_document(window, &changes); true }));
    interface.register_op("GFT_TICK", json_op!(|timeout| { API.tick(timeout); true }));
}

// so we don't need to compile whole deno
// note this is not safe nor ABI stable between different versions
#[allow(dead_code)]
mod deno_unstable_api {
    pub trait Interface {
        fn register_op(&mut self, name: &str, handler: fn(&mut dyn Interface, &mut [V8Buf]) -> Op);
    }

    pub type Buf = Box<[u8]>;

    pub type OpAsyncFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Buf>>>;

    pub enum Op {
        Sync(Buf),
        Async(OpAsyncFuture),
        AsyncUnref(OpAsyncFuture)
    }

    pub struct V8Buf {
        data_ptr_ptr: *const *mut u8,
        control_ptr: usize,
        offset: usize,
        len: usize
    }

    impl core::ops::Deref for V8Buf {
        type Target = [u8];

        fn deref(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts((*self.data_ptr_ptr).add(self.offset), self.len) }
        }
    }

    impl core::ops::DerefMut for V8Buf {
        fn deref_mut(&mut self) -> &mut [u8] {
            unsafe { std::slice::from_raw_parts_mut((*self.data_ptr_ptr).add(self.offset), self.len) }
        }
    }
}
