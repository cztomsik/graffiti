use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{App, Document, Viewport, Window};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let mut viewport = Viewport::new(win.size(), &Rc::new(RefCell::new(Document::new())));
    let mut backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

    let mut doc = viewport.document().borrow_mut();
    let root = doc.root();
    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    doc.insert_child(h1, hello, 0);
    doc.insert_child(root, h1, 0);
    drop(doc);

    while !win.should_close() {
        for e in win.events().try_iter() {
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
