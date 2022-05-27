#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
mod util;

mod document;
mod layout;
mod renderer;
mod windowing;

pub mod css;

pub use self::{
    document::{Document, NodeId, NodeKind},
    // TODO: feature-flag
    windowing::*,
};
pub mod gfx;

// TODO: feature-flag
mod ffi;
