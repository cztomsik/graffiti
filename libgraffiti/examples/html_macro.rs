use graffiti::{App};

fn main() {
    let mut app = unsafe { App::init() };
    let mut win = app.create_window("Hello", 800, 600);
    let mut viewport = win.create_viewport();

    let doc = viewport.document_mut();

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

    doc.insert_child(doc.root(), div, 0);

    println!("{:#?}", div);

    while !win.should_close() {
        viewport.update();
        viewport.render();

        win.swap_buffers();
        app.wait_events();
    }
}
