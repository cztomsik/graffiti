#![warn(clippy::all, clippy::pedantic)]

mod convert;
mod document;
mod layout;
mod renderer;
mod util;
mod viewport;
//mod windowing;

pub mod css;

// TODO: feature-flag
// windowing::*,
pub use self::{
    document::{Document, NodeId, NodeType},
    renderer::Renderer,
    viewport::Viewport,
};

// TODO: feature-flag
// mod ffi;
