// deno bindings

use crate::util::SlotMap;
use crate::{App, Viewport, Window};
use core::cell::RefCell;
use deno_unstable_api::*;
use std::rc::Rc;

type WindowId = u32;
type ViewportId = u32;

#[derive(Default)]
struct Ctx {
    app: Option<Rc<App>>,
    windows: SlotMap<WindowId, Window>,
    viewports: SlotMap<ViewportId, Viewport>,
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

    op!("GFT_NEXT_EVENT", |ctx, ()| {
        // TODO: return win id + event or wait or undefined
    });

    op!("GFT_CREATE_WINDOW", |ctx, (title, width, height)| {
        let window = ctx.app.as_ref().expect("no app").create_window(title, width, height);
        ctx.windows.insert(window)
    });

    op!("GFT_CREATE_VIEWPORT", |_ctx, ()| {
        //state.viewports.insert(Viewport::new(GlBackend::new()));
    });

    op!("GFT_CREATE_TEXT_NODE", |ctx, (viewport, text)| {
        ctx.viewports[viewport].document_mut().create_text_node(text)
    });

    op!("GFT_SET_TEXT", |ctx, (viewport, node, text)| {
        ctx.viewports[viewport].document_mut().set_text(node, text)
    });

    op!("GFT_CREATE_ELEMENT", |ctx, (viewport, local_name)| {
        ctx.viewports[viewport].document_mut().create_element(local_name)
    });

    op!("GFT_SET_ATTRIBUTE", |ctx, (viewport, el, att, value)| {
        ctx.viewports[viewport].document_mut().set_attribute(el, att, value)
    });

    op!("GFT_REMOVE_ATTRIBUTE", |ctx, (viewport, el, att)| {
        ctx.viewports[viewport].document_mut().remove_attribute(el, att)
    });

    op!("GFT_INSERT_CHILD", |ctx, (viewport, el, child, index)| {
        // TODO: usize arg?
        let index: u32 = index;
        let index = index as _;

        ctx.viewports[viewport].document_mut().insert_child(el, child, index)
    });

    op!("GFT_REMOVE_CHILD", |ctx, (viewport, parent, child)| {
        ctx.viewports[viewport].document_mut().remove_child(parent, child)
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
