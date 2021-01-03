use graffiti::{
    render::backend::{DrawCall, Frame, RenderBackend},
    Viewport,
};
use tiny_skia::{Canvas, Paint, Pixmap, Rect};

fn main() {
    let mut viewport = Viewport::new(PixmapBackend(Pixmap::new(400, 300).unwrap()));

    let d = viewport.document_mut();
    let h1 = d.create_element("h1");
    let hello = d.create_text_node("Hello tiny-skia!");
    d.insert_child(h1, hello, 0);
    d.insert_child(d.root(), h1, 0);

    viewport.update();
    viewport.render();
}

struct PixmapBackend(Pixmap);

impl RenderBackend for PixmapBackend {
    fn render_frame(&mut self, frame: Frame) {
        let mut canvas = Canvas::from(self.0.as_mut());
        let mut i = 0;

        for DrawCall { len } in frame.draw_calls {
            for q in &frame.quads[i..len] {
                let top_left = q.vertices[0];
                let bottom_right = q.vertices[3];

                let mut paint = Paint::default();
                let [r, g, b, a] = top_left.color;
                paint.set_color_rgba8(r, g, b, a);

                canvas.fill_rect(
                    Rect::from_ltrb(
                        top_left.xyz[0],
                        top_left.xyz[1],
                        bottom_right.xyz[0],
                        bottom_right.xyz[1],
                    )
                    .unwrap(),
                    &paint,
                );
            }

            i += len
        }

        self.0.save_png("image.png").unwrap();
    }
}
