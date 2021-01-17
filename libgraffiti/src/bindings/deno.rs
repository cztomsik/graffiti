// deno bindings

use std::sync::Mutex;
use crate::util::Lazy;
use crate::util::SlotMap;
use crate::{App, Window, Viewport};
use core::cell::RefCell;
use deno_unstable_api::*;

type WindowId = u32;

static VIEWPORTS: Lazy<Mutex<SlotMap<WindowId, Viewport>>> = lazy!(|| Mutex::new(SlotMap::new()));

#[derive(Default)]
struct Ctx {
    app: Option<App>,
    windows: SlotMap<WindowId, Window>,
}

thread_local! {
    static CTX: RefCell<Ctx> = Default::default()
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    dbg!();

    macro_rules! op {
        // TODO: generics
        ($name: literal, |$ctx:ident, ($($arg:ident),*)| $body:tt) => {
            interface.register_op($name, |_, bufs| {
                CTX.with(|ctx| {
                    let mut $ctx = ctx.borrow_mut();
                    let mut _bufs = bufs.iter();
                    $(let $arg = FromBytes::from_bytes(_bufs.next().expect("arg missing"));)*

                    ($body).into()
                })
            })
        };
    }

    op!("GFT_INIT", |ctx, ()| {
        ctx.app = Some(unsafe { App::init() });
    });

    op!("GFT_TICK", |ctx, ()| {
        for (id, win) in ctx.windows.iter_mut() {
            if let Some(e) = win.take_event() {
                println!("TODO: {:?}", e);
            }

            let viewport = &mut VIEWPORTS.lock().unwrap()[id];

            viewport.update();
            viewport.render();

            win.swap_buffers();
        }

        ctx.app.as_mut().expect("no app").wait_events_timeout(0.1);
    });

    op!("GFT_CREATE_WINDOW", |ctx, (title, width, height)| {
        let mut window = ctx.app.as_mut().expect("no app").create_window(title, width, height);
        let viewport = window.create_viewport();

        let id = ctx.windows.insert(window);

        VIEWPORTS.lock().unwrap().put(id, viewport);

        id
    });

    op!("GFT_CREATE_TEXT_NODE", |ctx, (win, text)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().create_text_node(text)
    });

    op!("GFT_SET_TEXT", |ctx, (win, node, text)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().set_text(node, text)
    });

    op!("GFT_CREATE_ELEMENT", |ctx, (win, local_name)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().create_element(local_name)
    });

    op!("GFT_SET_ATTRIBUTE", |ctx, (win, el, att, value)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().set_attribute(el, att, value)
    });

    op!("GFT_REMOVE_ATTRIBUTE", |ctx, (win, el, att)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().remove_attribute(el, att)
    });

    op!("GFT_INSERT_CHILD", |ctx, (win, el, child, index)| {
        // TODO: usize arg?
        let index: u32 = index;
        let index = index as _;

        VIEWPORTS.lock().unwrap()[win].document_mut().insert_child(el, child, index)
    });

    op!("GFT_REMOVE_CHILD", |ctx, (win, parent, child)| {
        VIEWPORTS.lock().unwrap()[win].document_mut().remove_child(parent, child)
    });
}

// safe to read from &[u8], any combination MUST HAVE valid meaning
trait Pod: Copy {}
impl Pod for u32 {}
impl Pod for i32 {}
impl Pod for f32 {}
impl Pod for f64 {}

trait FromBytes<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self;
}

impl<'a, T: Pod> FromBytes<'a> for T {
    fn from_bytes(bytes: &'a [u8]) -> T {
        assert!(bytes.len() == std::mem::size_of::<T>());
        unsafe { *(bytes.as_ptr() as *const _) }
    }
}

impl<'a> FromBytes<'a> for &'a str {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        std::str::from_utf8(bytes).expect("utf-8")
    }
}

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

    impl<T: Copy> From<T> for Op {
        fn from(v: T) -> Self {
            let data_ptr = Box::into_raw(Box::new(v));
            let len = std::mem::size_of::<T>();
            let bytes = unsafe { Vec::from_raw_parts(data_ptr as *mut u8, len, len).into_boxed_slice() };

            Self::Sync(bytes)
        }
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
