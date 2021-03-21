use graffiti::unstable::{Surface, Viewport};

fn main() {
  let mut viewport = Viewport::new(Surface::new(0 as _, (0., 0.)));
  let d = viewport.document_mut();

  let style = d.create_element("style");
  let rule = d.create_text_node("
    div { padding: 40px }
    .bg-white { background-color: #fff }
  ");

  d.insert_child(d.root(), style, 0);
  d.insert_child(style, rule, 0);

  let div = d.create_element("div");
  let hello = d.create_text_node("Hello world");

  d.insert_child(d.root(), div, 1);
  d.insert_child(div, hello, 0);

  d.set_attribute(div, "class", "bg-white");
  d.set_attribute(div, "style", "margin-top: 20px");

  viewport.update();

  println!("hello computed_style: {:?}", viewport.computed_style(div));
}
