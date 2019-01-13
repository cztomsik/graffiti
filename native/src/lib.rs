// node-bindings
#[macro_use]
extern crate neon;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
use env_logger;
use serde_json;

mod rendering;
mod resources;
mod surface;
mod window;

use crate::resources::{OpResource, ResourceManager};
use crate::surface::Surface;
use crate::window::{EventSender, Window, WindowEvent};
use neon::prelude::*;
use std::cell::RefCell;
use std::io::Write;
use std::mem::size_of;
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use yoga::FlexStyle;

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
            let mut this = ctx.this();
            let surface = ctx.argument::<JsSurface>(0)?;

            {
                let guard = ctx.lock();
                let mut window = this.borrow_mut(&guard);
                let surface = surface.borrow(&guard);

                ResourceManager::with(|rm| {
                    window.render(&rm.render_ops, &(surface.borrow()))
                })
            }

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

    pub class JsOpResource as RcOpResource for Rc<OpResource> {
        init(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let ops = serde_json::from_str(&data).unwrap();
            let r = ResourceManager::with(|rm| Rc::new(rm.create_op_resource(ops)));

            Ok(r)
        }
    }

    pub class JsSurface as RcRefSurface for Rc<RefCell<Surface>> {
        init(mut _ctx) {
            let s = Surface::new();

            Ok(Rc::new(RefCell::new(s)))
        }

        method appendChild(mut ctx) {
            let mut this = ctx.this();
            let child: Handle<JsSurface> = ctx.argument(0)?;

            {
                let guard = ctx.lock();
                let surface = this.borrow_mut(&guard);
                let child = child.borrow(&guard);

                {
                    let mut surface = surface.borrow_mut();
                    let child = (**child).clone();
                    surface.append_child(child)
                }
            }

            Ok(ctx.undefined().upcast())
        }

        method update(mut ctx) {
            let mut this = ctx.this();
            let brush: Option<Handle<JsOpResource>> = ctx.argument(0).ok();
            let clip: Option<Handle<JsOpResource>> = ctx.argument(1).ok();
            let flex_style: Handle<JsFlexStyle> = ctx.argument(2)?;

            {
                let guard = ctx.lock();
                let surface = this.borrow_mut(&guard);


                {
                    let mut surface = surface.borrow_mut();

                    let brush = brush.map(|b| b.borrow(&guard).clone());
                    let clip = clip.map(|c| c.borrow(&guard).clone());
                    let flex_style = flex_style.borrow(&guard).clone();

                    surface.set_brush(brush);
                    surface.set_clip(clip);
                    surface.apply_flex_style(flex_style);
                }
            }

            Ok(ctx.undefined().upcast())
        }

        method calculateLayout(mut ctx) {
            let mut this = ctx.this();
            let available_width = ctx.argument::<JsNumber>(0)?.value() as f32;
            let available_height = ctx.argument::<JsNumber>(1)?.value() as f32;

            ctx.borrow_mut(&mut this, |s| s.borrow_mut().calculate_layout(available_width, available_height));

            Ok(ctx.undefined().upcast())
        }
    }

    pub class JsFlexStyle as RcFlexStyle for Rc<Vec<FlexStyle>> {
        init(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let s = serde_json::from_str(&data).unwrap();

            debug!("style {:?}", s);

            Ok(Rc::new(s))
        }
    }
}

fn get_event_sender(socket_path: &str) -> Box<EventSender> {
    let socket_path = std::path::Path::new(socket_path);
    let socket = UnixStream::connect(&socket_path).unwrap();

    Box::new(SocketEventSender { socket })
}

register_module!(mut ctx, {
    env_logger::init();

    ctx.export_class::<JsWindow>("Window")?;
    ctx.export_class::<JsOpResource>("OpResource")?;
    ctx.export_class::<JsSurface>("Surface")?;
    ctx.export_class::<JsFlexStyle>("FlexLayout")
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
