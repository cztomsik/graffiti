#[macro_use]
mod util;

mod app;
mod css;
mod document;
mod layout;
mod renderer;
mod viewport;
mod webview;
mod window;

pub use self::{app::*, css::*, document::*, viewport::*, webview::*, window::*};
pub mod gfx;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

mod bindings;
