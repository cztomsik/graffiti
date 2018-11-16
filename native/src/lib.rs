// node-bindings
#[macro_use] extern crate neon;
extern crate glutin;
extern crate gleam;
extern crate webrender;
extern crate app_units;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate env_logger;

mod window;

use neon::prelude::*;
use window::{Window};

declare_types! {
    pub class JsWindow for Window {
        init(mut ctx) {
            let title = ctx.argument::<JsString>(0)?.value();

            let w = Window::new(title);

            Ok(w)
        }

        method createBucket(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let item = serde_json::from_str(&data).unwrap();

            let index = {
                let mut this = ctx.this();
                let guard = ctx.lock();
                let mut w = this.borrow_mut(&guard);

                w.create_bucket(item)
            };

            // TODO: maybe we can restrict vector size?
            Ok(ctx.number(index as f64).upcast())
        }

        method updateBucket(mut ctx) {
            let bucket = ctx.argument::<JsNumber>(0)?.value() as usize;

            let data = ctx.argument::<JsString>(0)?.value();
            let item = serde_json::from_str(&data).unwrap();

            let mut this = ctx.this();

            ctx.borrow_mut(&mut this, |mut w| w.update_bucket(bucket, item));

            Ok(ctx.undefined().upcast())
        }

        method render(mut ctx) {
            let data = ctx.argument::<JsString>(0)?.value();
            let request = serde_json::from_str(&data).unwrap();
            let mut this = ctx.this();

            ctx.borrow_mut(&mut this, |mut w| w.render(request));

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

        method getGlyphDimensions(mut ctx) {
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
    env_logger::init();

    ctx.export_class::<JsWindow>("Window")
});
