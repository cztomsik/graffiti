use graffiti::unstable::{Surface, Viewport};

fn main() {
  let mut viewport = Viewport::new(Surface::new(0 as _, (0., 0.)));

  viewport.insert_css_rule(0, "div", "padding: 40px");
  viewport.insert_css_rule(1, ".bg-white", "background-color: #fff");

  let d = viewport.document_mut();
  let div = d.create_element("div");
  let hello = d.create_text_node("Hello world");

  d.insert_child(d.root(), div, 0);
  d.insert_child(div, hello, 0);

  d.set_attribute(div, "class", "bg-white");
  // TODO: set inline style

  viewport.update();

  println!("hello computed_style: {:?}", viewport.computed_style(div));
}
