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
extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use glutin::{GlWindow, EventsLoop, GlContext};
use webrender::api::{RenderApi, RenderNotifier, Transaction, DisplayListBuilder, ColorF, DocumentId, PipelineId, Epoch, FontKey, FontInstanceKey, GlyphIndex, GlyphDimensions, GlyphInstance, LayoutRect, LayoutPoint, LayoutSize, LayoutPrimitiveInfo, RectangleDisplayItem, BorderDisplayItem, TextDisplayItem, PushStackingContextDisplayItem, StackingContext};
use webrender::{Renderer};
use std::os::raw::c_int;

pub struct Window {
    items: Vec<DisplayItem>,

    gl_window: GlWindow,

    api: RenderApi,
    renderer: Renderer,

    document_id: DocumentId,
    pipeline_id: PipelineId,
    epoch: Epoch,

    font: Vec<u8>,
    font_index: c_int,
    font_key: FontKey,
    font_instance_key: FontInstanceKey,

    rx: Receiver<Msg>
}

impl Window {
    pub fn new(title: String, width: f64, height: f64) -> Self {
        let (gl_window, events_loop) = Window::create_gl_window(title, width, height);
        let (tx, rx) = channel();
        let (mut api, renderer) = Window::create_api(&gl_window, events_loop, tx);
        let (document_id, pipeline_id, epoch) = Window::create_document(&mut api, &gl_window);
        let (font, font_index, font_key, font_instance_key) = Window::load_font(&mut api);

        let mut w = Window {
            items: Vec::new(),

            gl_window,
            api, renderer,
            document_id, pipeline_id, epoch,

            font, font_index, font_key, font_instance_key,

            rx
        };

        w.send_initial_frame();

        w
    }

    pub fn create_bucket(&mut self, item: DisplayItem) -> BucketId {
        let index = self.items.len();

        self.items.push(item);

        index
    }

    pub fn update_bucket(&mut self, bucket_id: BucketId, item: DisplayItem) {
        match self.items.get_mut(bucket_id) {
            None => panic!("bucket not found"),
            Some(bucket) => *bucket = item
        }
    }

    pub fn render(&mut self, request: RenderRequest) {
        let RenderRequest { bucket_ids, layouts } = request;

        if layouts.len() != bucket_ids.len() {
            panic!("missing/extra layouts")
        }

        let framebuffer_size = self.get_size();
        let content_size = framebuffer_size.to_f32() / euclid::TypedScale::new(1.0);

        let mut b = DisplayListBuilder::new(self.pipeline_id, content_size);

        for (layout, i) in layouts.iter().zip(bucket_ids.iter()) {
            let info = layout.to_info();

            match self.items.get(*i) {
                None => panic!("item not found"),
                Some(item) => {
                    match item {
                        DisplayItem::Text(TextDisplayItem { font_key, color, glyph_options }, glyphs) => b.push_text(&info, glyphs, *font_key, *color, *glyph_options),
                        DisplayItem::Rectangle(RectangleDisplayItem { color }) => b.push_rect(&info, *color),
                        DisplayItem::Border(BorderDisplayItem { widths, details }) => b.push_border(&info, *widths, *details),
                        DisplayItem::PopStackingContext => b.pop_stacking_context(),

                        // TODO: filters
                        DisplayItem::PushStackingContext(PushStackingContextDisplayItem { stacking_context }) => {
                            let StackingContext { transform_style, mix_blend_mode, clip_node_id, raster_space } = stacking_context;

                            b.push_stacking_context(&info, *clip_node_id, *transform_style, *mix_blend_mode, &Vec::new(), *raster_space)
                        }
                    }
                }
            }
        }

        let mut tx = Transaction::new();
        tx.set_display_list(self.epoch, None, content_size, b.finalize(), true);
        tx.set_root_pipeline(self.pipeline_id);
        tx.generate_frame();

        self.api.send_transaction(self.document_id, tx);

        self.epoch.0 += 1;

        // TODO: async
        debug!("waiting until frame ready");

        let _msg = self.rx.recv();

        self.redraw();
    }

    pub fn get_glyph_indices(&self, str: &str) -> Vec<u32> {
        self.api.get_glyph_indices(self.font_key, str).iter().filter_map(|i| *i).collect()
    }

    pub fn get_glyph_dimensions(&self, glyph_indices: Vec<GlyphIndex>) -> Vec<GlyphDimensions> {
        self.api.get_glyph_dimensions(self.font_instance_key, glyph_indices).iter().filter_map(|dims| *dims).collect()
    }

    fn create_gl_window(title: String, width: f64, height: f64) -> (GlWindow, EventsLoop){
        let window_builder = glutin::WindowBuilder::new().with_title(title).with_dimensions(glutin::dpi::LogicalSize::new(width, height));
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

    fn create_api(gl_window: &GlWindow, events_loop: EventsLoop, tx: Sender<Msg>) -> (RenderApi, Renderer) {
        let gl = match gl_window.get_api() {
            glutin::Api::OpenGl => unsafe { gleam::gl::GlFns::load_with(|s| gl_window.get_proc_address(s) as *const _) },
            glutin::Api::OpenGlEs => unsafe { gleam::gl::GlesFns::load_with(|s| gl_window.get_proc_address(s) as *const _) },
            glutin::Api::WebGl => unimplemented!()
        };

        let options = webrender::RendererOptions {
            clear_color: Some(ColorF::new(1., 1., 1., 1.)),
            ..webrender::RendererOptions::default()
        };

        let notifier = Notifier(events_loop.create_proxy(), tx);

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

        debug!("(font_key, font_instance_key) = {}", serde_json::to_string_pretty(&(font_key, font_instance_key)).unwrap());

        (font, font_index, font_key, font_instance_key)
    }

    fn send_initial_frame(&mut self) {
        let size = self.get_size().to_f32() / euclid::TypedScale::new(1.);
        let background = Some(ColorF::new(0., 0., 0., 1.));

        // initial tx
        let mut tx = Transaction::new();
        let b = webrender::api::DisplayListBuilder::new(self.pipeline_id, size);

        tx.add_raw_font(self.font_key, self.font.clone(), self.font_index as u32);
        tx.add_font_instance(self.font_instance_key, self.font_key, app_units::Au::from_px(24), None, None, Vec::new());

        tx.set_display_list(self.epoch, background, size, b.finalize(), true);
        tx.set_root_pipeline(self.pipeline_id);
        self.api.send_transaction(self.document_id, tx);
    }

    fn get_size(&self) -> euclid::TypedSize2D<i32, webrender::api::DevicePixel> {
        get_gl_window_size(&self.gl_window)
    }

    fn redraw(&mut self) {
        let framebuffer_size = self.get_size();
        let renderer = &mut self.renderer;

        renderer.update();
        renderer.render(framebuffer_size).unwrap();

        self.gl_window.swap_buffers().ok();
    }
}

fn get_gl_window_size(gl_window: &GlWindow) -> euclid::TypedSize2D<i32, webrender::api::DevicePixel> {
    let size = gl_window.get_inner_size().unwrap();
    webrender::api::DeviceIntSize::new(size.width as i32, size.height as i32)
}


struct Notifier (glutin::EventsLoopProxy, Sender<Msg>);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone(), self.1.clone()));
    }

    fn wake_up(&self) {
        let _ = self.0.wakeup();
    }

    fn new_frame_ready(&self, _doc_id: DocumentId, _scrolled: bool, _composite_needed: bool, _render_time_ns: Option<u64>) {
        let _ = self.1.send(Msg {});
        self.wake_up();
    }
}

#[derive(Deserialize)]
pub struct RenderRequest {
    bucket_ids: Vec<BucketId>,
    layouts: Vec<Layout>
}

pub struct Msg {}

pub type BucketId = usize;

#[derive(Deserialize)]
struct Layout(f32, f32, f32, f32);

impl Layout {
    fn to_info(&self) -> LayoutPrimitiveInfo {
        let Layout(x, y, width, height) = *self;
        let layout_rect = LayoutRect::new(LayoutPoint::new(x, y), LayoutSize::new(width, height));

        LayoutPrimitiveInfo::new(layout_rect)
    }
}

// like SpecificDisplayItem::* but the Text actually holds glyphs
#[derive(Deserialize)]
pub enum DisplayItem {
    Rectangle(RectangleDisplayItem),
    Border(BorderDisplayItem),
    Text(TextDisplayItem, Vec<GlyphInstance>),
    PopStackingContext,
    PushStackingContext(PushStackingContextDisplayItem)
}
