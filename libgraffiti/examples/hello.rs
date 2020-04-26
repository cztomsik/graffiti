use graffiti::{
    app::App,
    render::value_types::*,
    style::StyleProp,
    text::TextAlign,
    viewport::{Event, GlViewport, NodeId},
};

fn main() {
    let mut app = unsafe { App::init() };
    let w = app.create_window("Hello", 800, 600);

    let mut hello_str = "Hello world!".to_string();

    app.update_window_scene(w, &mut |v| {
        let el = v.create_element();
        let text = v.create_text_node();

        v.set_style(el, &StyleProp::BackgroundColor(Color::BLUE));
        v.set_style(el, &StyleProp::Color(Color::WHITE));
        v.set_style(el, &StyleProp::FontFamily("sans-serif".to_string()));
        v.set_style(el, &StyleProp::FontSize(50.));
        v.set_style(el, &StyleProp::TextAlign(TextAlign::Left));
        v.set_style(el, &StyleProp::LineHeight(60.));

        v.set_text(text, hello_str.clone());

        v.insert_child(GlViewport::ROOT, 0, el);
        v.insert_child(el, 0, text);
    });

    // loop
    'outer: loop {
        for e in app.get_events(false) {
            match e.event {
                Event::Close { .. } => break 'outer,
                Event::Resize { .. } => {
                    app.update_window_scene(w, &mut |v| {
                        // resize should be already done
                        v.update();
                    });
                }
                Event::KeyPress { key, .. } => {
                    app.update_window_scene(w, &mut |v| {
                        let text = unsafe { std::mem::transmute(2 as usize) };

                        hello_str.push(std::char::from_u32(key as u32).unwrap());

                        v.set_text(text, hello_str.clone());
                        v.update();
                    });
                }
                _ => {}
            }

            println!("{:?}", &e);
        }
    }
}
