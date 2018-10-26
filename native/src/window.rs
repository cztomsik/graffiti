/*
    ? tx priorities (scroll)
*/

extern crate glutin;
extern crate gleam;
extern crate webrender;
extern crate euclid;
extern crate app_units;
extern crate font_loader;
extern crate serde;
extern crate serde_json;

use glutin::{GlWindow, EventsLoop, GlContext};
use webrender::api::{RenderApi, RenderNotifier, Transaction, DisplayListBuilder, LayoutPrimitiveInfo, LayoutRect, LayoutPoint, LayoutSize, ColorF, DocumentId, PipelineId, Epoch, FontKey, FontInstanceKey};
use webrender::{Renderer};
use std::os::raw::c_int;


pub struct Window {
    gl_window: GlWindow,

    api: RenderApi,
    renderer: Renderer,

    document_id: DocumentId,
    pipeline_id: PipelineId,
    epoch: Epoch,

    font: Vec<u8>,
    font_index: c_int,
    font_key: FontKey,
    font_instance_key: FontInstanceKey
}

impl Window {
    pub fn new() -> Self {
        let (gl_window, events_loop) = Window::create_gl_window();
        let (mut api, renderer) = Window::create_api(&gl_window, events_loop);
        let (document_id, pipeline_id, epoch) = Window::create_document(&mut api, &gl_window);
        let (font, font_index, font_key, font_instance_key) = Window::load_font(&mut api);

        let mut w = Window {
            gl_window,
            api, renderer,
            document_id, pipeline_id, epoch,

            font, font_index, font_key, font_instance_key
        };

        w.send_initial_frame();

        w
    }

    pub fn send_frame(&mut self, data: &str) {
        let ops = parse_ops(&data);
        let framebuffer_size = self.get_size();
        let content_size = framebuffer_size.to_f32() / euclid::TypedScale::new(1.0);

        let mut b = DisplayListBuilder::new(self.pipeline_id, content_size);

        for op in ops {
            let info = LayoutPrimitiveInfo::new(LayoutRect::new(
                LayoutPoint::new(op.xy.0, op.xy.1),
                LayoutSize::new(op.wh.0, op.wh.1),
            ));

            let color = ColorF::new(op.color.0, op.color.1, op.color.2, 1.0);

            match op.kind.as_ref() {
                "rect" => { b.push_rect(&info, color) },
                _ => {}
            }
        }

        let mut tx = Transaction::new();
        tx.set_display_list(self.epoch, None, content_size, b.finalize(), true);
        tx.set_root_pipeline(self.pipeline_id);
        tx.generate_frame();

        self.api.send_transaction(self.document_id, tx);

        self.epoch.0 += 1
    }

    pub fn redraw(&mut self) {
        let framebuffer_size = self.get_size();
        let renderer = &mut self.renderer;

        renderer.update();
        renderer.render(framebuffer_size).unwrap();

        self.gl_window.swap_buffers().ok();
    }

    fn create_gl_window() -> (GlWindow, EventsLoop){
        let window_builder = glutin::WindowBuilder::new().with_title("Window").with_dimensions(glutin::dpi::LogicalSize::new(800., 600.));
        let context_builder = glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0)
        });
        let events_loop = EventsLoop::new();
        let gl_window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

        unsafe { gl_window.make_current().ok() };
        gl_window.show();

        (gl_window, events_loop)
    }

    fn create_api(gl_window: &GlWindow, events_loop: EventsLoop) -> (RenderApi, Renderer) {
        let gl = match gl_window.get_api() {
            glutin::Api::OpenGl => unsafe { gleam::gl::GlFns::load_with(|s| gl_window.get_proc_address(s) as *const _) },
            glutin::Api::OpenGlEs => unsafe { gleam::gl::GlesFns::load_with(|s| gl_window.get_proc_address(s) as *const _) },
            glutin::Api::WebGl => unimplemented!()
        };

        let options = webrender::RendererOptions {
            clear_color: Some(ColorF::new(0., 0., 0., 1.)),
            ..webrender::RendererOptions::default()
        };

        let notifier = Notifier(events_loop.create_proxy());

        let (renderer, sender) = Renderer::new(gl, Box::new(notifier), options, None).unwrap();
        let api = sender.create_api();

        (api, renderer)
    }

    fn create_document(api: &mut RenderApi, gl_window: &GlWindow) -> (DocumentId, PipelineId, Epoch) {
        let document_id = api.add_document(get_gl_window_size(gl_window), 0);
        let pipeline_id = PipelineId(0, 0);
        let epoch = Epoch(0);

        (document_id, pipeline_id, epoch)
    }

    fn load_font(api: &mut RenderApi) -> (Vec<u8>, c_int, FontKey, FontInstanceKey) {
        let property = font_loader::system_fonts::FontPropertyBuilder::new().family("Arial").build();
        let (font, font_index) = font_loader::system_fonts::get(&property).unwrap();
        let font_key = api.generate_font_key();
        let font_instance_key = api.generate_font_instance_key();

        (font, font_index, font_key, font_instance_key)
    }

    fn send_initial_frame(&mut self) {
        let size = self.get_size().to_f32() / euclid::TypedScale::new(1.);
        let background = Some(ColorF::new(0., 0., 0., 1.));

        // initial tx
        let mut tx = Transaction::new();
        let mut b = webrender::api::DisplayListBuilder::new(self.pipeline_id, size);

        tx.add_raw_font(self.font_key, self.font.clone(), self.font_index as u32);
        tx.add_font_instance(self.font_instance_key, self.font_key, app_units::Au::from_px(32), None, None, Vec::new());

        tx.set_display_list(self.epoch, background, size, b.finalize(), true);
        tx.set_root_pipeline(self.pipeline_id);
        self.api.send_transaction(self.document_id, tx);
    }

    fn get_size(&self) -> euclid::TypedSize2D<u32, webrender::api::DevicePixel> {
        get_gl_window_size(&self.gl_window)
    }
}

fn get_gl_window_size(gl_window: &GlWindow) -> euclid::TypedSize2D<u32, webrender::api::DevicePixel> {
    let size = gl_window.get_inner_size().unwrap();
    webrender::api::DeviceUintSize::new(size.width as u32, size.height as u32)
}


struct Notifier (glutin::EventsLoopProxy);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone()));
    }

    fn wake_up(&self) {
        let _ = self.0.wakeup();
    }

    fn new_frame_ready(&self, _doc_id: DocumentId, _scrolled: bool, _composite_needed: bool, _render_time_ns: Option<u64>) {
        self.wake_up();
    }
}

#[derive(Deserialize)]
pub struct Op {
    kind: String,
    xy: (f32, f32),
    wh: (f32, f32),
    color: (f32, f32, f32)
}

fn parse_ops(data: &str) -> Vec<Op> {
    serde_json::from_str(data).unwrap()
}
