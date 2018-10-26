// node-bindings
#[macro_use]
extern crate neon;
extern crate glutin;
extern crate gleam;
extern crate webrender;
extern crate app_units;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

mod window;
use window::{Window};

/*
fn render(font_key: FontKey, font_instance_key: FontInstanceKey, api: &RenderApi, _txn: &mut Transaction, builder: &mut DisplayListBuilder) {
    // rect
    let info = LayoutPrimitiveInfo::new(LayoutRect::new(
        LayoutPoint::new(10.0, 10.0),
        LayoutSize::new(100.0, 100.0),
    ));
    builder.push_rect(&info, ColorF::new(0.0, 0.0, 1.0, 1.0));

    // text
    let glyph_indices: Vec<GlyphIndex> = api.get_glyph_indices(font_key, "Hello world").iter().filter_map(|i| *i).collect();
    let metrics = api.get_glyph_dimensions(font_instance_key, glyph_indices.clone());

    println!("Glyph indices {}", glyph_indices.len());

    // layout glyphs
    let mut i = 0;
    let mut x = 0.0;
    let mut glyphs = Vec::new();
    for m in metrics {
        match m {
            Some(m) => {
                glyphs.push(GlyphInstance {
                    index: glyph_indices[i],
                    point: LayoutPoint::new(x, 60.0)
                });

                x += m.advance;
                i += 1;
            }
            None => {}
        }
    }

    for g in glyphs.clone() {
        println!("Glyph {} {}", g.index, g.point);
    }

    let info = LayoutPrimitiveInfo::new(LayoutRect::new(
        LayoutPoint::new(0.0, 30.0),
        LayoutSize::new(200.0, 100.0),
    ));
    builder.push_text(&info, &glyphs, font_instance_key, ColorF::new(0.0, 1.0, 0.0, 1.0), None);
}

*/

declare_types! {
    pub class JsWindow for Window {
        init(mut ctx) {
            let _name = ctx.argument::<JsString>(0)?;

            let w = Window::new();

            Ok(w)
        }

        method sendFrame(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();

            {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut w = this.borrow_mut(&guard);
                w.send_frame(&data);
            };

            Ok(ctx.undefined().upcast())
        }

        method redraw(mut ctx) {
            {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut w = this.borrow_mut(&guard);
                w.redraw();
            }

            Ok(ctx.undefined().upcast())
        }
    }
}

register_module!(mut ctx, {
    ctx.export_class::<JsWindow>("Window")
});
