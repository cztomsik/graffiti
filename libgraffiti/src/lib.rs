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

mod bindings {
    #[cfg(feature = "deno")]
    mod deno;

    #[cfg(feature = "nodejs")]
    mod nodejs;

    //#[cfg(feature = "quickjs")]
    //mod quickjs;
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}
