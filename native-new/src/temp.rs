//! temp helpers so that they don't clutter the rest of the code
//! all of this has to be refactored before we can support multiple windows
//! TODO: software renderer (RendererKind)

use std::sync::mpsc::{Sender};
use webrender::api::{
    DocumentId,
    RenderNotifier,
};

pub fn handle_events() {
/*    unsafe {
        match &mut TEMP {
            None => {}
            Some(Temp {
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

                                /*
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
                                    .expect("resize re-render failed");*/
                                //temp.gl_window.swap_buffers().ok();

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
    }*/
}

pub struct Notifier(pub Sender<()>);

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<RenderNotifier> {
        return Box::new(Notifier(self.0.clone()));
    }

    fn wake_up(&self) {
        self.0.send(()).ok();
    }

    fn new_frame_ready(
        &self,
        _document_id: DocumentId,
        _scrolled: bool,
        _composite_needed: bool,
        _render_time_ns: Option<u64>,
    ) {
        self.wake_up();
    }
}
