// deno bindings

use super::API;
use crate::api::WindowId;
use crate::util::Lazy;
use crate::window::Event;
use core::future::Future;
use deno_unstable_api::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::task::Waker;
use std::task::{Context, Poll};

// temporary hack
static WAKERS: Lazy<Mutex<Vec<Waker>>> = lazy!(|| Mutex::new(Vec::new()));

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    silly!("deno_plugin_init");

    macro_rules! op {
        ($name: expr, $handler: expr) => {
            interface.register_op($name, |_, bufs| {
                silly!("[deno] {}", $name);
                let handler: fn(&mut ArgsReader) -> _ = $handler;
                handler(&mut ArgsReader::new(bufs.iter().map(|b| &**b))).into()
            })
        };
    }

    op!("GFT_TICK", |_| API.tick());

    op!("GFT_CREATE_WINDOW", |arg| API.create_window(arg.str(), arg.i32(), arg.i32(), |win| {
        WAKERS.lock().unwrap().drain(..).for_each(Waker::wake);
    }));

    op!("GFT_CREATE_TEXT_NODE", |arg| API.create_text_node(arg.u32(), arg.str()));
    op!("GFT_SET_TEXT", |arg| API.set_text(arg.u32(), arg.u32(), arg.str()));

    op!("GFT_CREATE_ELEMENT", |arg| API.create_element(arg.u32(), arg.u32()));
    op!("GFT_SET_STYLE", |arg| API.set_style(arg.u32(), arg.u32(), arg.str(), arg.str()));
    op!("GFT_ADD_TAG", |arg| API.add_tag(arg.u32(), arg.u32(), arg.u32()));
    op!("GFT_REMOVE_TAG", |arg| API.remove_tag(arg.u32(), arg.u32(), arg.u32()));
    op!("GFT_INSERT_CHILD", |arg| API.insert_child(arg.u32(), arg.u32(), arg.u32(), arg.u32() as usize));
    op!("GFT_REMOVE_CHILD", |arg| API.remove_child(arg.u32(), arg.u32(), arg.u32()));

    op!("GFT_TAKE_EVENT", |arg| {
        struct NextEvent(WindowId);

        impl Future for NextEvent {
            type Output = Box<[u8]>;

            fn poll(self: std::pin::Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<<Self>::Output> {
                match API.take_event(self.0) {
                    Some(event) => Poll::Ready(bytes(event)),
                    None => {
                        WAKERS.lock().unwrap().push(ctx.waker().clone());

                        Poll::Pending
                    }
                }
            }
        }

        Op::Async(Box::pin(NextEvent(arg.u32())))
    })
}

use std::cell::Cell;

struct ArgsReader<'a> {
    bin: &'a [u8],
    offset: Cell<usize>,
    strs: Vec<&'a str>,
    str_index: Cell<usize>,
}

impl<'a> ArgsReader<'a> {
    fn new(mut bufs: impl Iterator<Item = &'a [u8]>) -> Self {
        Self {
            bin: bufs.next().unwrap_or(&[]),
            offset: Cell::new(0),
            strs: bufs.map(|b| unsafe { std::str::from_utf8_unchecked(b) }).collect(),
            str_index: Cell::new(0),
        }
    }

    fn u32(&self) -> u32 {
        self.pod()
    }

    fn i32(&self) -> i32 {
        self.pod()
    }

    fn pod<T: Copy>(&self) -> T {
        let res = unsafe { *(&self.bin[self.offset.get()] as *const _ as *const T) };

        self.offset.set(self.offset.get() + std::mem::size_of::<T>());

        res
    }

    fn str(&self) -> &str {
        let res = self.strs.get(self.str_index.get()).expect("str missing");

        self.str_index.set(self.str_index.get() + 1);

        res
    }
}

impl<T: Copy> From<T> for Op {
    fn from(v: T) -> Self {
        Self::Sync(bytes(v))
    }
}

fn bytes<T: Copy>(v: T) -> Box<[u8]> {
    let data_ptr = Box::into_raw(Box::new(v));
    let len = std::mem::size_of::<T>();

    unsafe { Vec::from_raw_parts(data_ptr as *mut u8, len, len).into_boxed_slice() }
}

// subset of deno plugin api so we don't need to compile whole deno
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
        AsyncUnref(OpAsyncFuture),
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

    impl core::ops::DerefMut for V8Buf {
        fn deref_mut(&mut self) -> &mut [u8] {
            unsafe { std::slice::from_raw_parts_mut((*self.data_ptr_ptr).add(self.offset), self.len) }
        }
    }
}
