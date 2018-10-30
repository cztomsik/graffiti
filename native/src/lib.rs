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

mod window;
mod display_item;

use neon::prelude::*;
use window::{Window};

declare_types! {
    pub class JsWindow for Window {
        init(mut ctx) {
            let _name = ctx.argument::<JsString>(0)?;

            let w = Window::new();

            Ok(w)
        }

        method sendFrame(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let mut this = ctx.this();

            ctx.borrow_mut(&mut this, |mut w| w.send_frame(&data));

            Ok(ctx.undefined().upcast())
        }

        method getGlyphIndices(mut ctx) {
            let str = ctx.argument::<JsString>(0)?.value();
            let mut this = ctx.this();

            let indices = ctx.borrow(&mut this, |w| w.get_glyph_indices(&str));

            let js_array = JsArray::new(&mut ctx, indices.len() as u32);

            for (i, glyph_i) in indices.iter().enumerate() {
                let js_num = ctx.number(*glyph_i as f64);
                let _ = js_array.set(&mut ctx, i as u32, js_num);
            }

            Ok(js_array.upcast())
        }
    }
}

register_module!(mut ctx, {
    ctx.export_class::<JsWindow>("Window")
});
