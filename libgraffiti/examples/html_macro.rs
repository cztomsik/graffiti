use graffiti::gfx::{GlBackend, RenderBackend};
use graffiti::{App, Document, Node, Viewport, Window};
use std::rc::Rc;

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let doc = Document::new();
    let viewport = Viewport::new(win.size(), doc.clone());
    let mut backend = unsafe { GlBackend::new(|s| win.get_proc_address(s) as _) };

    // super-simple prefix macro
    macro_rules! html {
        ($text:literal) => (doc.create_text_node($text) as Rc<dyn Node>);
        ([ $tag:ident $(. $cls:ident)*: $($inner:tt)* ]) => ({
            let el = doc.create_element(stringify!($tag));
            el.set_attribute("class", stringify!($($cls)*));

            for (_i, child) in [ $(html!($inner)),* ].iter().enumerate() {
                el.append_child(child.clone())
            }

            el as Rc<dyn Node>
        });
    }

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

    doc.append_child(div);

    while !win.should_close() {
        backend.render_frame(viewport.render());

        win.swap_buffers();
        app.wait_events();
    }
}
