use glfw::{Context, Glfw, Window, WindowEvent};
use graffiti::renderer::{ContainerStyle, Outline, RenderEdge, Renderer, Shadow, StrokeStyle};
use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color, FontMgr, Paint, Rect,
};
use std::sync::mpsc::Receiver;

fn main() {
    let (mut glfw, mut win, events) = create_window();
    let mut renderer = Renderer::new(win.get_size());

    let para = create_para("Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet. 😀");

    let tick = |win: &mut Window, renderer: &mut Renderer| {
        renderer.render(&build_render_tree(&para));
        win.swap_buffers();
    };

    while !win.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => renderer.resize((width, height)),
                _ => println!("{:?}", &event),
            }
        }

        tick(&mut win, &mut renderer);
    }
}

fn create_window() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let glfw = init_glfw();

    let (mut win, events) = glfw
        .create_window(400, 300, "Hello", glfw::WindowMode::Windowed)
        .expect("GLFW create_window()");

    win.make_current();
    win.set_all_polling(true);

    (glfw, win, events)
}

fn init_glfw() -> Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}

fn create_para(s: &str) -> Paragraph {
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_color(Paint::default());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(s);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(256.0);

    paragraph
}

fn build_render_tree<'a>(para: &'a Paragraph) -> Vec<RenderEdge<&'a Paragraph>> {
    vec![
        RenderEdge::OpenContainer(
            Rect::new(10., 10., 390., 290.),
            ContainerStyle {
                bg_color: Some(Color::new(0xFFDDDDDD)),
                ..Default::default()
            },
        ),
        RenderEdge::OpenContainer(
            Rect::new(10., 10., 370., 270.),
            ContainerStyle {
                bg_color: Some(Color::new(0xFFAAAAFF)),
                outline: Some(Outline(1., Some(StrokeStyle::Solid), Color::BLUE)),
                ..Default::default()
            },
        ),
        RenderEdge::Text(Rect::new(10., 10., 300., 200.), &para),
        RenderEdge::OpenContainer(
            Rect::new(10., 150., 100., 200.),
            ContainerStyle {
                border_radii: Some([2., 2., 2., 2.]),
                shadow: Some(Shadow((0., 1.), 10., 0., Color::new(0x66000000))),
                clip: true,
                bg_color: Some(Color::new(0xFFFFFFFF)),
                ..Default::default()
            },
        ),
        RenderEdge::OpenContainer(
            Rect::new(10., 10., 300., 300.),
            ContainerStyle {
                bg_color: Some(Color::new(0xFFDDDDDD)),
                ..Default::default()
            },
        ),
        RenderEdge::CloseContainer,
        RenderEdge::CloseContainer,
        RenderEdge::CloseContainer,
        RenderEdge::CloseContainer,
    ]
}
