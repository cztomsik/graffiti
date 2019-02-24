//! temp helpers so that they don't clutter the rest of the code
//! all of this has to be refactored before we can support multiple windows
//! TODO: software renderer (RendererKind)

use glutin::{EventsLoop, GlContext, GlWindow, WindowBuilder};
use std::sync::mpsc::{channel, Receiver, Sender};
use webrender::api::{DocumentId, RenderApi, RenderNotifier};
use webrender::{Renderer, RendererOptions};

static mut TEMP: Option<Temp> = None;

pub fn init() {
    unsafe {
        // so that we can block until the frame is actually rendered
        let (tx, rx) = channel();

        // so that we can handle events (and stay responsive)
        let events_loop = glutin::EventsLoop::new();

        // get & init native window with gl support
        let (gl_window, gl) = {
            let gl_window = GlWindow::new(
                WindowBuilder::new().with_dimensions(glutin::dpi::LogicalSize::new(200., 200.)),
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
                device_pixel_ratio: 96.0,
                ..RendererOptions::default()
            },
            None,
        )
        .expect("couldn't create renderer");
        let render_api = sender.create_api();

        TEMP = Some(Temp {
            events_loop,
            gl_window,
            render_api,
            renderer,
            rx,
        });
    }
}

pub fn handle_events() {
    unsafe {
        match TEMP {
            None => {}
            Some(ref mut temp) => temp.events_loop.poll_events(|_e| {}),
        }
    }
}

struct Temp {
    events_loop: EventsLoop,
    gl_window: GlWindow,
    render_api: RenderApi,
    renderer: Renderer,
    rx: Receiver<()>,
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
