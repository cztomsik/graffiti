/*
    ? tx priorities (scroll)
*/

extern crate app_units;
extern crate euclid;
extern crate font_loader;
extern crate gleam;
extern crate glutin;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate webrender;

use glutin::dpi::LogicalPosition;
use glutin::{EventsLoop, GlContext, GlWindow};
use std::os::raw::c_int;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use webrender::api::{
    BorderDisplayItem, ColorF, DisplayListBuilder, DocumentId, Epoch, FontInstanceKey, FontKey,
    GlyphDimensions, GlyphIndex, GlyphInstance, HitTestFlags, LayoutPoint, LayoutPrimitiveInfo,
    LayoutRect, LayoutSize, PipelineId, PushStackingContextDisplayItem, RectangleDisplayItem,
    RenderApi, RenderNotifier, StackingContext, TextDisplayItem, Transaction, WorldPoint,
};
use webrender::Renderer;

pub struct Window {
    items: Vec<DisplayItem>,

    gl_window: GlWindow,
    events_loop: EventsLoop,
    mouse_position: LogicalPosition,

    api: RenderApi,
    renderer: Renderer,

    // TODO subscribe to event
    dpi: f64,

    document_id: DocumentId,
    pipeline_id: PipelineId,
    epoch: Epoch,

    font: Vec<u8>,
    font_index: c_int,
    font_key: FontKey,
    font_instance_key: FontInstanceKey,

    rx: Receiver<Msg>,
}

impl Window {
    pub fn new(title: String, width: f64, height: f64) -> Self {
        let (gl_window, events_loop) = Window::create_gl_window(title, width, height);

        let dpi = gl_window.get_hidpi_factor();
        let (tx, rx) = channel();
        let (mut api, renderer) = Window::create_api(&gl_window, &events_loop, tx, dpi);
        let (document_id, pipeline_id, epoch) = Window::create_document(&mut api, &gl_window, dpi);
        let (font, font_index, font_key, font_instance_key) = Window::load_font(&mut api);

        let mut w = Window {
            items: Vec::new(),

            gl_window,
            events_loop,
            mouse_position: LogicalPosition::new(0., 0.),
            api,
            renderer,
            dpi,

            document_id,
            pipeline_id,
            epoch,

            font,
            font_index,
            font_key,
            font_instance_key,

            rx,
        };

        w.send_initial_frame();

        w
    }

    // TODO: thread
    // TODO: one thread for many windows
    // this will be big change, and it's mostly needed because EventsLoop cannot be moved to thread
    // and so the thread will need to hold everything and we will probably just send messages to it
    // for now, it's enough to call window.handleEvents() from js setInterval()
    pub fn handle_events(&mut self) -> Vec<u32> {
        let mut callback_ids = Vec::new();

        let mut should_redraw = false;

        // all of this is because of borrowing vs. mutable mouse_position
        {
            let document_id = self.document_id;
            let events_loop = &mut self.events_loop;
            let api = &self.api;
            let mouse_position = &mut self.mouse_position;

            events_loop.poll_events(|glutin_event| {
                match glutin_event {
                    glutin::Event::WindowEvent { event, .. } => {
                        debug!("Event {:?}", event);

                        match event {
                            // TODO
                            glutin::WindowEvent::Resized(_) => should_redraw = true,

                            glutin::WindowEvent::CursorMoved { position, .. } => {
                                *mouse_position = position
                            }

                            glutin::WindowEvent::MouseInput { state, .. } => match state {
                                glutin::ElementState::Released => {
                                    let LogicalPosition { x, y } = *mouse_position;
                                    let point = WorldPoint::new(x as f32, y as f32);
                                    let res = api.hit_test(
                                        document_id,
                                        None,
                                        point,
                                        HitTestFlags::FIND_ALL,
                                    );

                                    for it in res.items {
                                        callback_ids.push(it.tag.0 as u32)
                                    }
                                }

                                _ => {}
                            },

                            _ => {}
                        }
                    }
                    _ => {}
                }
            });
        }

        if should_redraw {
            self.redraw();
        }

        callback_ids
    }

    pub fn create_bucket(&mut self, item: DisplayItem) -> BucketId {
        let index = self.items.len();

        self.items.push(item);

        index
    }

    pub fn update_bucket(&mut self, bucket_id: BucketId, item: DisplayItem) {
        match self.items.get_mut(bucket_id) {
            None => panic!("bucket not found"),
            Some(bucket) => *bucket = item,
        }
    }

    pub fn render(&mut self, request: RenderRequest) {
        let RenderRequest {
            bucket_ids,
            layouts,
        } = request;

        if layouts.len() != bucket_ids.len() {
            panic!("missing/extra layouts")
        }

        let framebuffer_size = get_frame_buffer_size(&self.gl_window, self.dpi);
        let content_size = framebuffer_size.to_f32() / euclid::TypedScale::new(self.dpi as f32);

        let mut b = DisplayListBuilder::new(self.pipeline_id, content_size);

        for (layout, i) in layouts.iter().zip(bucket_ids.iter()) {
            let mut info = layout.to_info();

            match self.items.get(*i) {
                None => panic!("item not found"),
                Some(item) => {
                    match item {
                        DisplayItem::HitTest(tag) => {
                            info.tag = Some((*tag as u64, 0 as u16));
                            b.push_rect(&info, ColorF::TRANSPARENT);
                        }
                        DisplayItem::Text(
                            TextDisplayItem {
                                font_key,
                                color,
                                glyph_options,
                            },
                            glyphs,
                        ) => b.push_text(&info, glyphs, *font_key, *color, *glyph_options),
                        DisplayItem::Rectangle(RectangleDisplayItem { color }) => {
                            b.push_rect(&info, *color)
                        }
                        DisplayItem::Border(BorderDisplayItem { widths, details }) => {
                            b.push_border(&info, *widths, *details)
                        }
                        DisplayItem::PopStackingContext => b.pop_stacking_context(),

                        // TODO: filters
                        DisplayItem::PushStackingContext(PushStackingContextDisplayItem {
                            stacking_context,
                        }) => {
                            let StackingContext {
                                transform_style,
                                mix_blend_mode,
                                clip_node_id,
                                raster_space,
                            } = stacking_context;

                            b.push_stacking_context(
                                &info,
                                *clip_node_id,
                                *transform_style,
                                *mix_blend_mode,
                                &Vec::new(),
                                *raster_space,
                            )
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

    pub fn get_glyph_indices_and_advances(&self, text: &str) -> (Vec<GlyphIndex>, Vec<f32>) {
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
            .get_glyph_dimensions(self.font_instance_key, glyph_indices.clone())
            .iter()
            .map(|dims| dims.unwrap_or(EMPTY_DIMENSIONS).advance)
            .collect();

        (glyph_indices, advances)
    }

    fn create_gl_window(title: String, width: f64, height: f64) -> (GlWindow, EventsLoop) {
        let window_builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let context_builder =
            glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            });
        let events_loop = EventsLoop::new();
        let gl_window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();

        unsafe { gl_window.make_current().ok() };
        gl_window.show();

        (gl_window, events_loop)
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

    fn load_font(api: &mut RenderApi) -> (Vec<u8>, c_int, FontKey, FontInstanceKey) {
        let property = font_loader::system_fonts::FontPropertyBuilder::new()
            .family("Arial")
            .build();
        let (font, font_index) = font_loader::system_fonts::get(&property).unwrap();
        let font_key = api.generate_font_key();
        let font_instance_key = api.generate_font_instance_key();

        debug!(
            "(font_key, font_instance_key) = {}",
            serde_json::to_string_pretty(&(font_key, font_instance_key)).unwrap()
        );

        (font, font_index, font_key, font_instance_key)
    }

    fn send_initial_frame(&mut self) {
        let size = get_frame_buffer_size(&self.gl_window, self.dpi).to_f32()
            / euclid::TypedScale::new(self.dpi as f32);
        let background = Some(ColorF::new(0., 1.0, 0., 1.));

        // initial tx
        let mut tx = Transaction::new();
        let b = webrender::api::DisplayListBuilder::new(self.pipeline_id, size);

        tx.add_raw_font(self.font_key, self.font.clone(), self.font_index as u32);
        tx.add_font_instance(
            self.font_instance_key,
            self.font_key,
            app_units::Au::from_px(24),
            None,
            None,
            Vec::new(),
        );

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

#[derive(Deserialize)]
pub struct RenderRequest {
    bucket_ids: Vec<BucketId>,
    layouts: Vec<Layout>,
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
    HitTest(u32),
    Rectangle(RectangleDisplayItem),
    Border(BorderDisplayItem),
    Text(TextDisplayItem, Vec<GlyphInstance>),
    PopStackingContext,
    PushStackingContext(PushStackingContextDisplayItem),
}
