// node-bindings
#[macro_use]
extern crate neon;
extern crate glutin;
extern crate webrender;
extern crate gleam;
extern crate app_units;
extern crate euclid;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;
use webrender::api::*;
use glutin::GlContext;

// fonts
//extern crate font_loader;
//use font_loader::system_fonts;

// webrender calls back
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

pub struct Window {
    window: glutin::GlWindow,
    api: RenderApi,
    renderer: webrender::Renderer,
    document_id: DocumentId,
    epoch: Epoch,
    pipeline_id: PipelineId
}

impl Window {
    fn new() -> Self {
        let gl_version = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2));
        let window_builder = glutin::WindowBuilder::new().with_dimensions(glutin::dpi::LogicalSize::new(800.0, 600.0));
        let context_builder = glutin::ContextBuilder::new().with_gl(gl_version).with_vsync(true);
        let events_loop = glutin::EventsLoop::new();

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
            ..webrender::RendererOptions::default()
        };
        let (renderer, sender) = webrender::Renderer::new(
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

        Window {
            window, api, renderer, document_id, epoch, pipeline_id
        }
    }

    pub fn get_size(&self) -> euclid::TypedSize2D<u32, DevicePixel> {
        let size = self.window.get_inner_size().unwrap();
        DeviceUintSize::new(size.width as u32, size.height as u32)
    }

    pub fn send_frame(&mut self, ops: Vec<Op>) {
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
    }

    pub fn redraw(&mut self) {
        let framebuffer_size = self.get_size();
        let renderer = &mut self.renderer;

        renderer.update();
        renderer.render(framebuffer_size).unwrap();

        self.window.swap_buffers().ok();
    }
}

fn parse_ops(data: &str) -> Vec<Op> {
    serde_json::from_str(data).unwrap()
}


/*
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
*/



declare_types! {
    pub class JsWindow for Window {
        init(mut ctx) {
            let _name = ctx.argument::<JsString>(0)?;

            let w = Window::new();

            Ok(w)
        }

        method sendFrame(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let ops = parse_ops(&data);

            let _ = {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut w = this.borrow_mut(&guard);
                w.send_frame(ops);
            };

            Ok(ctx.undefined().upcast())
        }

        method redraw(mut ctx) {
            let _ = {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut w = this.borrow_mut(&guard);
                w.redraw();
            };

            Ok(ctx.undefined().upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsWindow>("Window")
});
