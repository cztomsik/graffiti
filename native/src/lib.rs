// node-bindings
#[macro_use]
extern crate neon;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
use env_logger;
use serde_json;

mod resources;
mod window;

use crate::resources::ResourceManager;
use crate::window::{EventSender, Window, WindowEvent};
use neon::prelude::*;
use std::io::Write;
use std::mem::size_of;
use std::os::unix::net::UnixStream;

declare_types! {
    pub class JsWindow for Window {
        init(mut ctx) {
            let title = ctx.argument::<JsString>(0)?.value();
            let width = ctx.argument::<JsNumber>(1)?.value();
            let height = ctx.argument::<JsNumber>(2)?.value();
            let socket_path = ctx.argument::<JsString>(3)?.value();

            let w = Window::new(title, width, height, get_event_sender(&socket_path));

            Ok(w)
        }

        method render(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let request = serde_json::from_str(&data).unwrap();
            let mut this = ctx.this();

            ctx.borrow_mut(&mut this, |mut w| {
                ResourceManager::with(|rm| {
                    w.render(&rm.buckets, request)
                })
            });

            Ok(ctx.undefined().upcast())
        }

        // TODO: remove this once we have UI thread
        method handleEvents(mut ctx) {
            let mut this = ctx.this();
            ctx.borrow_mut(&mut this, |mut w| w.handle_events());

            Ok(ctx.undefined().upcast())
        }

        method getGlyphIndicesAndAdvances(mut ctx) {
            let font_size = ctx.argument::<JsNumber>(0)?.value() as u32;
            let str = ctx.argument::<JsString>(1)?.value();
            let mut this = ctx.this();

            let (glyph_indices, advances) = ctx.borrow(&mut this, |w| w.get_glyph_indices_and_advances(font_size, &str));
            let len = glyph_indices.len() as u32;

            let js_array = JsArray::new(&mut ctx, 2);

            let mut b1 = JsArrayBuffer::new(&mut ctx, len * (size_of::<u32>() as u32)).unwrap();
            let mut b2 = JsArrayBuffer::new(&mut ctx, len * (size_of::<f32>() as u32)).unwrap();

            {
                let guard = ctx.lock();

                let slice = b1.borrow_mut(&guard).as_mut_slice::<u32>();
                slice.copy_from_slice(&glyph_indices[..]);

                let slice = b2.borrow_mut(&guard).as_mut_slice::<f32>();
                slice.copy_from_slice(&advances[..]);
            }

            js_array.set(&mut ctx, 0, b1).unwrap();
            js_array.set(&mut ctx, 1, b2).unwrap();

            Ok(js_array.upcast())
        }
    }
}

fn js_create_bucket(mut ctx: FunctionContext) -> JsResult<JsNumber> {
    let data = ctx.argument::<JsString>(0)?.value();
    let item = serde_json::from_str(&data).unwrap();
    let bucket_id = ResourceManager::with(|rm| rm.create_bucket(item));

    Ok(ctx.number(bucket_id as f64))
}

fn js_update_bucket(mut ctx: FunctionContext) -> JsResult<JsUndefined> {
    let bucket_id = ctx.argument::<JsNumber>(0)?.value() as u32;
    let data = ctx.argument::<JsString>(1)?.value();
    let item = serde_json::from_str(&data).unwrap();

    ResourceManager::with(|rm| rm.update_bucket(bucket_id, item));

    Ok(ctx.undefined())
}

fn get_event_sender(socket_path: &str) -> Box<EventSender> {
    let socket_path = std::path::Path::new(socket_path);
    let socket = UnixStream::connect(&socket_path).unwrap();

    Box::new(SocketEventSender { socket })
}

register_module!(mut ctx, {
    env_logger::init();

    ctx.export_class::<JsWindow>("Window")?;
    ctx.export_function("createBucket", js_create_bucket)?;
    ctx.export_function("updateBucket", js_update_bucket)
});

struct SocketEventSender {
    socket: UnixStream,
}

// TODO: either share memory or at least support windows (NamedPipes)
impl EventSender for SocketEventSender {
    fn send(&mut self, event: WindowEvent) {
        debug!("sending {:?}", event);
        let buf: [u8; 12] = unsafe { std::mem::transmute(event) };

        self.socket.write_all(&buf).unwrap();
    }
}
