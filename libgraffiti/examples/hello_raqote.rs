use graffiti::{
    box_layout::*,
    render::backend::raqote::RaqoteBackend,
    render::value_types::*,
    text_layout::{Text, TextAlign},
    viewport::{StyleProp, Viewport},
};

fn main() {
    let mut v = Viewport::new(RaqoteBackend::new("out.png".to_string()), (400., 300.));
    v.set_style(<Viewport<RaqoteBackend>>::ROOT, &StyleProp::BackgroundColor(Color::RED));

    let panel = v.create_element();
    v.insert_child(<Viewport<RaqoteBackend>>::ROOT, 0, panel);
    v.set_style(panel, &StyleProp::BackgroundColor(Color::GREEN));
    v.set_style(panel, &StyleProp::FlexGrow(1.));
    v.set_style(panel, &StyleProp::MarginRight(Dimension::Px(20.0)));

    let button = v.create_element();
    v.insert_child(panel, 0, button);
    v.set_style(button, &StyleProp::MarginLeft(Dimension::Px(20.0)));
    v.set_style(button, &StyleProp::BackgroundColor(Color::BLUE));
    let button_text = v.create_text_node();
    v.insert_child(button, 0, button_text);
    v.set_text(
        button_text,
        &Text {
            font_size: 16.,
            line_height: 30.,
            align: TextAlign::Left,
            text: "Hello world!".to_string(),
        },
    );

    // just to check it works fine
    v.resize((800., 600.));

    v.update();
}
