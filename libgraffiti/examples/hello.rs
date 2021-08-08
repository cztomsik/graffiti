use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{App, Document, Node, Viewport, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let doc = Document::new();
    let viewport = Viewport::new(win.size(), doc.clone());
    let mut backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

    let h1 = doc.create_element("h1");
    let hello = doc.create_text_node("Hello");
    h1.append_child(hello.clone());
    doc.append_child(h1);

    while !win.should_close() {
        for e in win.events().try_iter() {
            hello.set_data(&format!("Hello {:#?}", e));
            println!("{}", hello.data());
        }

        backend.render_frame(viewport.render());

        win.swap_buffers();
        app.wait_events();
    }
}
