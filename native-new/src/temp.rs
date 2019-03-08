//! temp helpers so that they don't clutter the rest of the code
//! all of this has to be refactored before we can support multiple windows
//! TODO: software renderer (RendererKind)

use env_logger;
use glutin::{EventsLoop, GlContext, GlWindow, WindowBuilder};
use std::sync::mpsc::{channel, Receiver, Sender};
use webrender::api::{
    DeviceIntSize, DisplayListBuilder, DocumentId, Epoch, LayoutSize, PipelineId, RenderApi,
    RenderNotifier, Transaction, FontInstanceKey
};
use webrender::{Renderer, RendererOptions};

static mut TEMP: Option<Temp> = None;

// proper multi-window support is rather big task (> week)
// in the meantime we can access render_api this way
pub fn with_api<F, SomeRes>(f: F) -> SomeRes
    where
        F: FnOnce(&mut RenderApi) -> SomeRes,
{
    unsafe {
        match &mut TEMP {
            None => panic!("not initialized"),
            Some(temp) => f(&mut temp.inner.render_api),
        }
    }
}

pub fn init() {
    env_logger::init();

    let fb_size = DeviceIntSize::new(300, 300);
    let layout_size = LayoutSize::new(fb_size.width as f32, fb_size.height as f32);

    unsafe {
        // so that we can block until the frame is actually rendered
        let (tx, rx) = channel();

        // so that we can handle events (and stay responsive)
        let events_loop = glutin::EventsLoop::new();

        // get & init native window with gl support
        let (gl_window, gl) = {
            let gl_window = GlWindow::new(
                WindowBuilder::new().with_dimensions(glutin::dpi::LogicalSize::new(
                    layout_size.width as f64,
                    layout_size.height as f64,
                )),
                glutin::ContextBuilder::new(),
                &events_loop,
            )
            .expect("couldn't create GlWindow");
            let gl = gleam::gl::GlFns::load_with(|s| gl_window.get_proc_address(s) as *const _);

            gl_window.make_current().ok();
            gl_window.show();

            (gl_window, gl)
        };

        // init webrender
        let (renderer, sender) = Renderer::new(
            gl,
            Box::new(Notifier(events_loop.create_proxy(), tx)),
            RendererOptions {
                device_pixel_ratio: 1.0,
                ..RendererOptions::default()
            },
            None,
        )
        .expect("couldn't create renderer");
        let render_api = sender.create_api();

        let document_id = render_api.add_document(fb_size, 0);
        let pipeline_id = PipelineId::dummy();

        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family("Arial")
            .build();
        let (font, font_index) = font_loader::system_fonts::get(&property).unwrap();
        let font_key = render_api.generate_font_key();

        let mut tx = Transaction::new();
        tx.set_root_pipeline(pipeline_id);
        tx.add_raw_font(font_key, font, font_index as u32);

        // TODO: support any size
        for font_size in [10, 12, 14, 16, 20, 24, 34, 40, 48].iter() {
            tx.add_font_instance(
                FontInstanceKey(font_key.0, *font_size),
                font_key,
                app_units::Au::from_px(*font_size as i32),
                None,
                None,
                Vec::new(),
            );
        }

        tx.generate_frame();
        render_api.send_transaction(document_id, tx);
        rx.recv().ok();

        TEMP = Some(Temp {
            events_loop,
            inner: Inner {
                fb_size,
                layout_size,
                gl_window,
                render_api,
                renderer,
                rx,
                document_id,
                pipeline_id,
            },
        });
    }
}

pub fn handle_events() {
    unsafe {
        match &mut TEMP {
            None => {}
            Some(Temp {
                events_loop,
                inner: temp,
            }) => events_loop.poll_events(move |e| {
                match e {
                    glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            // some events are going to be handled in rust and javascript will only get notified
                            // one such case is resize (web does the same)
                            glutin::WindowEvent::Resized(size) => {
                                let dpi = 1.0;
                                let real_size = size.to_physical(dpi);

                                temp.layout_size =
                                    LayoutSize::new(size.width as f32, size.height as f32);
                                temp.fb_size = DeviceIntSize::new(
                                    real_size.width as i32,
                                    real_size.height as i32,
                                );

                                temp.render_api.set_window_parameters(
                                    temp.document_id,
                                    temp.fb_size,
                                    temp.fb_size.into(),
                                    dpi as f32,
                                );
                                temp.gl_window.resize(real_size);

                                temp.renderer.update();
                                temp.renderer
                                    .render(temp.fb_size)
                                    .expect("resize re-render failed");
                                temp.gl_window.swap_buffers().ok();

                                // TODO: notify js
                                // TODO: recalculate layout & redraw
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }),
        }
    }
}

pub fn get_layout_size() -> LayoutSize {
    unsafe {
        match &TEMP {
            None => panic!("not initialized"),
            Some(temp) => temp.inner.layout_size.clone(),
        }
    }
}

pub fn send_frame(builder: DisplayListBuilder) {
    unsafe {
        match &mut TEMP {
            None => {}
            Some(temp) => {
                let temp = &mut temp.inner;
                let mut tx = Transaction::new();

                tx.set_root_pipeline(temp.pipeline_id);
                tx.set_display_list(Epoch(0), None, temp.layout_size, builder.finalize(), true);
                tx.generate_frame();

                temp.render_api.send_transaction(temp.document_id, tx);

                // TODO: we could return early and let notifier do the rest
                debug!("waiting for frame");
                temp.rx.recv().ok();

                temp.renderer.update();
                temp.renderer.render(temp.fb_size).ok();
                temp.gl_window.swap_buffers().ok();
            }
        }
    }
}

struct Temp {
    events_loop: EventsLoop,
    inner: Inner,
}

// so that it can be borrowed independently on events_loop
struct Inner {
    fb_size: DeviceIntSize,
    layout_size: LayoutSize,
    gl_window: GlWindow,
    render_api: RenderApi,
    renderer: Renderer,
    rx: Receiver<()>,
    document_id: DocumentId,
    pipeline_id: PipelineId,
}

struct Notifier(glutin::EventsLoopProxy, Sender<()>);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone(), self.1.clone()));
    }

    fn wake_up(&self) {
        let _ = self.0.wakeup();
    }

    fn new_frame_ready(
        &self,
        _document_id: DocumentId,
        _scrolled: bool,
        _composite_needed: bool,
        _render_time_ns: Option<u64>,
    ) {
        let _ = self.1.send(());
        self.wake_up();
    }
}
