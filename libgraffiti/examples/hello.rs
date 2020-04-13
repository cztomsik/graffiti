use graffiti::{
    app::App,
    render::value_types::*,
    text_layout::{Text, TextAlign},
    viewport::{Event, GlViewport, StyleProp},
};

fn main() {
    let mut app = unsafe { App::init() };
    let w = app.create_window("Hello", 800, 600);

    let mut wrapper = 0;
    let mut text = 0;

    app.update_window_scene(w, &mut |v| {
        wrapper = v.create_element();
        text = v.create_text_node();

        v.set_style(wrapper, &StyleProp::BackgroundColor(Color::GREEN));
        v.set_text(
            text,
            &Text {
                font_size: 16.,
                line_height: 30.,
                align: TextAlign::Left,
                text: "Hello world!".to_string(),
            },
        );

        v.insert_child(GlViewport::ROOT, 0, wrapper);
        v.insert_child(wrapper, 0, text);
    });

    // loop
    'outer: loop {
        for e in app.get_events(false) {
            if let Event::Close { .. } = e.event {
                break 'outer;
            }

            println!("{:?}", &e);
        }
    }
}
