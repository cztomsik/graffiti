use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{App, Document, Viewport, Window};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = unsafe { App::init() };
    let mut win = Window::new(&app, "Hello", 1024, 768);
    let mut viewport = Viewport::new(win.size(), &Rc::new(RefCell::new(Document::new())));

    unsafe { GlBackend::load_with(|s| win.get_proc_address(s) as _) }

    let mut backend = GlBackend::new();

    let mut doc = viewport.document().borrow_mut();
    let root = doc.root();
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    doc.insert_child(h1, hello, 0);
    doc.insert_child(root, h1, 0);
    drop(doc);

    while !win.should_close() {
        if let Some(e) = win.take_event() {
            viewport
                .document()
                .borrow_mut()
                .set_cdata(hello, &format!("Hello {:#?}", e));
        }

        backend.render_frame(viewport.render());

        win.swap_buffers();
        app.wait_events();
    }
}
