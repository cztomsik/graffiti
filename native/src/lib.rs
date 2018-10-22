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
    let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
    unsafe { window.make_current().ok() };
    window.show();

    let gl = unsafe { gleam::gl::GlFns::load_with(|s| window.context().get_proc_address(s) as *const std::os::raw::c_void) };

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

    println!("renderer initialized");

    let framebuffer_size = DeviceUintSize::new(800, 600);
    let api = std::rc::Rc::new(sender.create_api());
    let document_id = api.add_document(framebuffer_size, 0);
    let epoch = Epoch(0);
    let pipeline_id = PipelineId(0, 0);
    let layout_size = LayoutSize::new(800.0, 600.0);
    let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
    let mut txn = Transaction::new();

    render(&mut builder);

    println!("render done");

    txn.set_display_list(
        epoch,
        None,
        layout_size,
        builder.finalize(),
        true,
    );
    txn.set_root_pipeline(pipeline_id);
    txn.generate_frame();
    api.send_transaction(document_id, txn);

    println!("frame ok");

    events_loop.run_forever(|_event| {
/*        let mut txn = Transaction::new();
        let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

        render(&mut builder);

        txn.set_display_list(
            epoch,
            None,
            layout_size,
            builder.finalize(),
            true,
        );
        txn.generate_frame();
        api.send_transaction(document_id, txn);*/

        renderer.update();
        renderer.render(framebuffer_size).unwrap();
        let _ = renderer.flush_pipeline_info();
        window.swap_buffers().ok();

        glutin::ControlFlow::Continue
    });

    println!("loop ok");
}

fn render(builder: &mut DisplayListBuilder) {
    let info = LayoutPrimitiveInfo::new(LayoutRect::new(
        LayoutPoint::new(10.0, 10.0),
        LayoutSize::new(100.0, 100.0)
    ));
    builder.push_rect(&info, ColorF::new(0.0, 0.0, 1.0, 1.0));
}

register_module!(mut cx, {
    init();

    //cx.export_function("hello", hello)

    let hello = cx.string("hello");
    cx.export_value("hello", hello)
});
