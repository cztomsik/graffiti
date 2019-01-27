/*
    ? tx priorities (scroll)
*/

use app_units;
use euclid;
use font_loader;
use gleam;
use glutin;
use webrender;

use crate::rendering::{LayoutHelpers, RenderContext, RenderOperation};
use crate::surface::Surface;
use glutin::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use glutin::{EventsLoop, GlContext, GlWindow};
use std::cell::RefCell;
use std::os::raw::c_int;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use webrender::api::{
    ColorF, DeviceIntPoint, DeviceIntRect, DisplayListBuilder, DocumentId, Epoch, FontInstanceKey,
    FontKey, GlyphDimensions, GlyphIndex, HitTestFlags, HitTestResult, LayoutVector2D, PipelineId,
    RenderApi, RenderNotifier, ScrollLocation, Transaction, WorldPoint,
};
use webrender::Renderer;
use yoga::Layout;
use serde::Serialize;

pub struct Application {
    events_loop: EventsLoop,
    windows: Vec<Rc<RefCell<Window>>>,
}

impl Application {
    pub fn new() -> Self {
        let el = EventsLoop::new();
        let proxy = el.create_proxy();
        let duration = std::time::Duration::from_millis(30);

        std::thread::spawn(move ||loop {
            std::thread::sleep(duration);
            let _ = proxy.wakeup();
        });

        Application {
            events_loop: el,
            windows: vec![],
        }
    }

    pub fn create_window(
        &mut self,
        title: String,
        w: f64,
        h: f64,
        event_handler: Box<EventHandler>,
    ) -> Rc<RefCell<Window>> {
        let w = Window::new(title, w, h, &self.events_loop, event_handler);
        let c = Rc::new(RefCell::new(w));

        self.windows.push(c.clone());

        c
    }

    pub fn loop_a_bit(&mut self) {
        let windows = &mut self.windows;

        self.events_loop.run_forever(move |e| match e {
            glutin::Event::Awakened => glutin::ControlFlow::Break,
            glutin::Event::WindowEvent { window_id, event } => {
                let w = windows
                    .iter_mut()
                    .find(|ref mut w| w.borrow().id == window_id)
                    .expect("got event from unknown window");

                w.borrow_mut().handle_event(&event);

                glutin::ControlFlow::Continue
            }
            _ => glutin::ControlFlow::Continue,
        });
    }
}

// TODO: split
// TODO: drop (from app)
pub struct Window {
    // win
    id: glutin::WindowId,
    gl_window: GlWindow,
    dpi: f64,

    // rendering
    api: RenderApi,
    renderer: Renderer,
    document_id: DocumentId,
    pipeline_id: PipelineId,
    epoch: Epoch,
    rx: Receiver<Msg>,

    // font
    font: Vec<u8>,
    font_index: c_int,
    font_key: FontKey,

    // events
    mouse_position: LogicalPosition,
    event_handler: Box<EventHandler>,
}

impl Window {
    pub fn new(
        title: String,
        width: f64,
        height: f64,
        events_loop: &EventsLoop,
        event_handler: Box<EventHandler>,
    ) -> Self {
        let gl_window = Window::create_gl_window(title, width, height, events_loop);

        let dpi = gl_window.get_hidpi_factor();
        let (tx, rx) = channel();
        let (mut api, renderer) = Window::create_api(&gl_window, &events_loop, tx, dpi);
        let (document_id, pipeline_id, epoch) = Window::create_document(&mut api, &gl_window, dpi);
        let (font, font_index, font_key) = Window::load_font(&mut api);

        let mut w = Window {
            id: gl_window.id(),
            gl_window,
            dpi,

            api,
            renderer,
            document_id,
            pipeline_id,
            epoch,
            rx,

            font,
            font_index,
            font_key,

            mouse_position: LogicalPosition::new(0., 0.),
            event_handler,
        };

        w.send_initial_frame();

        w
    }

    pub fn handle_event(&mut self, event: &glutin::WindowEvent) {
        let mut should_redraw = false;

        match *event {
            glutin::WindowEvent::HiDpiFactorChanged(dpi) => self.dpi = dpi,

            glutin::WindowEvent::CloseRequested => self.handle_close(),

            glutin::WindowEvent::Resized(size) => {
                self.handle_resize(size);
                should_redraw = true
            }

            glutin::WindowEvent::CursorMoved { position, .. } => self.mouse_position = position,

            glutin::WindowEvent::MouseInput { state, .. } => self.handle_mouse(state),

            // TODO: scroll x
            glutin::WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    glutin::MouseScrollDelta::PixelDelta(point) => (point.y as f32),
                    glutin::MouseScrollDelta::LineDelta(_, dy) => 30. * dy,
                };

                self.scroll(y);
            }

            glutin::WindowEvent::ReceivedCharacter(ch) => self.handle_char(ch),

            _ => {}
        }

        if should_redraw {
            self.redraw();
        }
    }

    pub fn render(&mut self, ops: &Vec<RenderOperation>, surface: &Surface) {
        let framebuffer_size = get_frame_buffer_size(&self.gl_window, self.dpi);
        let content_size = framebuffer_size.to_f32() / euclid::TypedScale::new(self.dpi as f32);

        let res = {
            let mut builder = DisplayListBuilder::new(self.pipeline_id, content_size);
            let mut ctx = RenderContext {
                depth: 0,
                offset: (0., 0.),
                ops: &ops,
                builder: &mut builder,
                pipeline_id: self.pipeline_id,
                saved_rect: Layout::new(0., 0., 0., 0., 0., 0.).to_layout_rect(),
            };

            ctx.render_surface(surface);

            builder.finalize()
        };

        let mut tx = Transaction::new();
        tx.set_display_list(self.epoch, None, content_size, res, true);
        tx.set_root_pipeline(self.pipeline_id);
        tx.generate_frame();

        self.send_tx(tx);
    }

    fn handle_close(&mut self) {
        self.event_handler.handle_event(WindowEvent::Close);
    }

    fn handle_resize(&mut self, size: LogicalSize) {
        let fb_size = get_frame_buffer_size(&self.gl_window, self.dpi as f64);
        let inner_rect = DeviceIntRect::new(DeviceIntPoint::new(0, 0), fb_size.clone());

        self.api
            .set_window_parameters(self.document_id, fb_size, inner_rect, self.dpi as f32);
        self.gl_window
            .context()
            .resize(PhysicalSize::from_logical(size, self.dpi));

        self.event_handler
            .handle_event(WindowEvent::Resize(size.width as f32, size.height as f32));
    }

    fn handle_char(&mut self, ch: char) {
        self.event_handler.handle_event(WindowEvent::KeyPress(ch))
    }

    fn handle_mouse(&mut self, state: glutin::ElementState) {
        /*self.event_handler.send(WindowEvent::MouseInput(
            self.mouse_position.x,
            self.mouse_position.y,
        ))*/
        match state {
            glutin::ElementState::Released => {
                let cursor = self.get_cursor();
                let res = self.hit_test(cursor);

                for it in res.items {
                    self.event_handler.handle_event(WindowEvent::Click(it.tag.0 as u32))
                }
            }

            _ => {}
        }
    }

    pub fn hit_test(&mut self, point: WorldPoint) -> HitTestResult {
        // pipeline_id is not needed
        self.api
            .hit_test(self.document_id, None, point, HitTestFlags::FIND_ALL)
    }

    pub fn scroll(&mut self, y: f32) {
        let mut tx = Transaction::new();
        let scroll_location = ScrollLocation::Delta(LayoutVector2D::new(0., y));
        tx.scroll(scroll_location, self.get_cursor());
        debug!("scroll {:?} {:?}", scroll_location, self.get_cursor());
        tx.generate_frame();
        self.send_tx(tx);
    }

    pub fn send_tx(&mut self, tx: Transaction) {
        self.api.send_transaction(self.document_id, tx);

        self.epoch.0 += 1;

        // TODO: async
        debug!("waiting until frame ready");

        let _msg = self.rx.recv();

        self.redraw();
    }

    pub fn get_cursor(&self) -> WorldPoint {
        let LogicalPosition { x, y } = self.mouse_position;

        WorldPoint::new(x as f32, y as f32)
    }

    pub fn get_glyph_indices_and_advances(
        &self,
        font_size: u32,
        text: &str,
    ) -> (Vec<GlyphIndex>, Vec<f32>) {
        // we could also return a string of what we actually found
        // but it's **much** easier to just insert a space
        const SPACE_INDEX: u32 = 1;

        // TODO: should not happen?
        const EMPTY_DIMENSIONS: GlyphDimensions = GlyphDimensions {
            left: 0,
            top: 0,
            width: 0,
            height: 0,
            advance: 0.,
        };

        let glyph_indices: Vec<GlyphIndex> = self
            .api
            .get_glyph_indices(self.font_key, text)
            .iter()
            .map(|glyph_index| glyph_index.unwrap_or(SPACE_INDEX))
            .collect();

        let advances: Vec<f32> = self
            .api
            .get_glyph_dimensions(
                FontInstanceKey(self.font_key.0, font_size),
                glyph_indices.clone(),
            )
            .iter()
            .map(|dims| dims.unwrap_or(EMPTY_DIMENSIONS).advance)
            .collect();

        (glyph_indices, advances)
    }

    fn create_gl_window(
        title: String,
        width: f64,
        height: f64,
        events_loop: &EventsLoop,
    ) -> GlWindow {
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let context_builder =
            glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            });
        let gl_window = GlWindow::new(window_builder, context_builder, events_loop).unwrap();

        unsafe { gl_window.make_current().ok() };
        gl_window.show();

        gl_window
    }

    fn create_api(
        gl_window: &GlWindow,
        events_loop: &EventsLoop,
        tx: Sender<Msg>,
        dpi: f64,
    ) -> (RenderApi, Renderer) {
        let gl = match gl_window.get_api() {
            glutin::Api::OpenGl => unsafe {
                gleam::gl::GlFns::load_with(|s| gl_window.get_proc_address(s) as *const _)
            },
            glutin::Api::OpenGlEs => unsafe {
                gleam::gl::GlesFns::load_with(|s| gl_window.get_proc_address(s) as *const _)
            },
            glutin::Api::WebGl => unimplemented!(),
        };

        let options = webrender::RendererOptions {
            clear_color: Some(ColorF::new(1., 1., 1., 1.)),
            device_pixel_ratio: dpi as f32,
            ..webrender::RendererOptions::default()
        };

        let notifier = Notifier(events_loop.create_proxy(), tx);

        let (renderer, sender) = Renderer::new(gl, Box::new(notifier), options, None).unwrap();
        let api = sender.create_api();

        (api, renderer)
    }

    fn create_document(
        api: &mut RenderApi,
        gl_window: &GlWindow,
        dpi: f64,
    ) -> (DocumentId, PipelineId, Epoch) {
        let document_id = api.add_document(get_frame_buffer_size(gl_window, dpi), 0);
        let pipeline_id = PipelineId(0, 0);
        let epoch = Epoch(0);

        (document_id, pipeline_id, epoch)
    }

    fn load_font(api: &mut RenderApi) -> (Vec<u8>, c_int, FontKey) {
        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family("Arial")
            .build();
        let (font, font_index) = font_loader::system_fonts::get(&property).unwrap();
        let font_key = api.generate_font_key();

        debug!("font_key = {:?}", font_key);

        (font, font_index, font_key)
    }

    fn send_initial_frame(&mut self) {
        let size = get_frame_buffer_size(&self.gl_window, self.dpi).to_f32()
            / euclid::TypedScale::new(self.dpi as f32);
        let background = Some(ColorF::new(0., 1.0, 0., 1.));

        // initial tx
        let mut tx = Transaction::new();
        let b = webrender::api::DisplayListBuilder::new(self.pipeline_id, size);

        tx.add_raw_font(self.font_key, self.font.clone(), self.font_index as u32);

        // TODO: support any size
        for font_size in [10, 12, 14, 16, 20, 24, 34, 40, 48].iter() {
            tx.add_font_instance(
                FontInstanceKey(self.font_key.0, *font_size),
                self.font_key,
                app_units::Au::from_px(*font_size as i32),
                None,
                None,
                Vec::new(),
            );
        }

        tx.set_display_list(self.epoch, background, size, b.finalize(), true);
        tx.set_root_pipeline(self.pipeline_id);
        self.api.send_transaction(self.document_id, tx);
    }

    fn redraw(&mut self) {
        let framebuffer_size = get_frame_buffer_size(&self.gl_window, self.dpi);
        let renderer = &mut self.renderer;

        renderer.update();
        renderer.render(framebuffer_size).unwrap();

        self.gl_window.swap_buffers().ok();
    }
}

fn get_frame_buffer_size(
    gl_window: &GlWindow,
    dpi: f64,
) -> euclid::TypedSize2D<i32, webrender::api::DevicePixel> {
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    webrender::api::DeviceIntSize::new(size.width as i32, size.height as i32)
}

struct Notifier(glutin::EventsLoopProxy, Sender<Msg>);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone(), self.1.clone()));
    }

    fn wake_up(&self) {
        let _ = self.0.wakeup();
    }

    fn new_frame_ready(
        &self,
        _doc_id: DocumentId,
        _scrolled: bool,
        _composite_needed: bool,
        _render_time_ns: Option<u64>,
    ) {
        let _ = self.1.send(Msg {});
        self.wake_up();
    }
}

pub struct Msg {}

#[derive(Debug, Serialize)]
pub enum WindowEvent {
    Close,

    // yoga is f32 so we are too
    Resize(f32, f32),

    // TODO not yet sure what should be supported
    // KeyUp(u32), KeyDown(u32),

    // "repeatable" press: characters, including accents, upper-case, backspace, etc.
    KeyPress(char),

    Click(u32),
}

pub trait EventHandler {
    fn handle_event(&mut self, event: WindowEvent);
}
