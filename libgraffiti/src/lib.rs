#![warn(clippy::all, clippy::pedantic)]

mod convert;
mod document;
mod layout;
mod renderer;
mod util;
mod viewport;

pub mod css;

pub use self::{
    document::{Document, NodeId, NodeType},
    renderer::Renderer,
    viewport::Viewport,
};

#[cfg(feature = "windowing")]
mod windowing;
#[cfg(feature = "windowing")]
pub use windowing::{App, Event, Window};

#[cfg(feature = "ffi")]
mod ffi;
