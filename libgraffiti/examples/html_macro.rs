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

    // super-simple prefix macro
    macro_rules! html {
        ($text:literal) => (doc.create_text_node($text));
        ([ $tag:ident $(. $cls:ident)*: $($inner:tt)* ]) => ({
            let el = doc.create_element(stringify!($tag));
            doc.set_attribute(el, "class", stringify!($($cls)*));

            for (i, &child) in [ $(html!($inner)),* ].iter().enumerate() {
                doc.insert_child(el, child, i)
            }

            el
        });
    }

    let root = doc.root();
    let div = html! (
        [div.bar:
            [h1: "Hello macro!"]
            [p:
                "xxx"
                "yyy"
                [strong: "zzz"]
            ]
        ]
    );

    doc.insert_child(root, div, 0);
    drop(doc);

    while !win.should_close() {
        backend.render_frame(viewport.render());

        win.swap_buffers();
        app.wait_events();
    }
}
