use graffiti::{App, DocumentRef, Renderer, Window};

fn main() {
    let app = unsafe { App::init() };
    let win = Window::new("Hello", 1024, 768);
    let doc = DocumentRef::new();
    let renderer = Renderer::new(&doc, &win);

    // super-simple prefix macro
    macro_rules! html {
        ($text:literal) => (doc.create_text_node($text).as_node());
        ([ $tag:ident $(. $cls:ident)*: $($inner:tt)* ]) => ({
            let el = doc.create_element(stringify!($tag));
            el.set_attribute("class", stringify!($($cls)*));

            for child in [ $(html!($inner)),* ].iter() {
                el.append_child(&child)
            }

            el.as_node()
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

    doc.append_child(&div);

    while !win.should_close() {
        renderer.render();
        app.tick();
    }
}
