// node-bindings
#[macro_use]
extern crate neon;
use neon::prelude::*;

extern crate webrender;
// Renderer::new takes gleam::gl::Gl
extern crate gleam;

// window
extern crate glutin;

use webrender::api::{RenderNotifier, DocumentId};

// scope with get_proc_address()
use glutin::GlContext;

use webrender::api::*;

extern crate app_units;

extern crate font_loader;
use font_loader::system_fonts;

struct Notifier (glutin::EventsLoopProxy);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone()));
    }

    fn wake_up(&self) {
        self.0.wakeup().ok();
    }

    fn new_frame_ready(&self, _doc_id: DocumentId, _scrolled: bool, _composite_needed: bool, _render_time_ns: Option<u64>) {
        // TODO
        // self.0.wake_up();
    }
}

fn init() {
    let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
    let window_builder = glutin::WindowBuilder::new().with_dimensions(glutin::dpi::LogicalSize::new(800.0, 600.0));
    let context_builder = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true);
    let mut events_loop = glutin::EventsLoop::new();

    println!("get window");
    let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
    unsafe { window.make_current().ok() };
    window.show();

    println!("get gl");
    let gl = unsafe { gleam::gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const std::os::raw::c_void) };

    println!("get renderer");
    let opts = webrender::RendererOptions {
        device_pixel_ratio: 1.0,
        clear_color: Some(ColorF::new(0.3, 0.0, 0.0, 1.0)),
        debug_flags: webrender::DebugFlags::ECHO_DRIVER_MESSAGES,
        ..webrender::RendererOptions::default()
    };
    let (mut renderer, sender) = webrender::Renderer::new(
        gl,
        Box::new(Notifier(events_loop.create_proxy())),
        opts,
        Option::None
    ).unwrap();

    let framebuffer_size = DeviceUintSize::new(800, 600);
    let api = sender.create_api();
    let document_id = api.add_document(framebuffer_size, 0);
    let epoch = Epoch(0);
    let pipeline_id = PipelineId(0, 0);
    let layout_size = LayoutSize::new(800.0, 600.0);
    let mut txn = Transaction::new();

    // font
    let property = system_fonts::FontPropertyBuilder::new().family("Arial").build();
    let (font, index) = system_fonts::get(&property).unwrap();
    let font_key = api.generate_font_key();
    let font_instance_key = api.generate_font_instance_key();
    txn.add_raw_font(font_key, font, index as u32);
    txn.add_font_instance(font_instance_key, font_key, app_units::Au::from_px(32), None, None, Vec::new());
    txn.set_root_pipeline(pipeline_id);
    txn.generate_frame();
    api.send_transaction(document_id, txn);

    println!("frame ok");

    events_loop.run_forever(|_event| {
        let mut txn = Transaction::new();
        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

        render(font_key, font_instance_key, &api, &mut txn, &mut builder);

        txn.set_display_list(
            epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.generate_frame();
        api.send_transaction(document_id, txn);

        renderer.update();
        renderer.render(framebuffer_size).unwrap();
        let _ = renderer.flush_pipeline_info();
        window.swap_buffers().ok();

        glutin::ControlFlow::Continue
    });

    println!("loop ok");
}

fn render(font_key: FontKey, font_instance_key: FontInstanceKey, api: &RenderApi, _txn: &mut Transaction, builder: &mut DisplayListBuilder) {
    // rect
    let info = LayoutPrimitiveInfo::new(LayoutRect::new(
        LayoutPoint::new(10.0, 10.0),
        LayoutSize::new(100.0, 100.0),
    ));
    builder.push_rect(&info, ColorF::new(0.0, 0.0, 1.0, 1.0));

    // text
    let glyph_indices: Vec<GlyphIndex> = api.get_glyph_indices(font_key, "Hello world").iter().filter_map(|i| *i).collect();
    let metrics = api.get_glyph_dimensions(font_instance_key, glyph_indices.clone());

    println!("Glyph indices {}", glyph_indices.len());

    // layout glyphs
    let mut i = 0;
    let mut x = 0.0;
    let mut glyphs = Vec::new();
    for m in metrics {
        match m {
            Some(m) => {
                glyphs.push(GlyphInstance {
                    index: glyph_indices[i],
                    point: LayoutPoint::new(x, 60.0)
                });

                x += m.advance;
                i += 1;
            }
            None => {}
        }
    }

    for g in glyphs.clone() {
        println!("Glyph {} {}", g.index, g.point);
    }

    let info = LayoutPrimitiveInfo::new(LayoutRect::new(
        LayoutPoint::new(0.0, 30.0),
        LayoutSize::new(200.0, 100.0),
    ));
    builder.push_text(&info, &glyphs, font_instance_key, ColorF::new(0.0, 1.0, 0.0, 1.0), None);
}

register_module!(mut cx, {
    init();

    //cx.export_function("hello", hello)

    let hello = cx.string("hello");
    cx.export_value("hello", hello)
});
