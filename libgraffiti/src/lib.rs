#[macro_use]
mod util;

mod css;
mod document;
mod layout;
mod render;
mod viewport;

//#[cfg(feature = "window")]
mod window;
pub use window::*;

pub use self::{
    css::ResolvedStyle,
    document::{Document, NodeId},
    render::backend,
    viewport::Viewport,
};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

mod bindings;
