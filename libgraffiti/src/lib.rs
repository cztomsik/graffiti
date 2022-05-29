#![warn(clippy::all, clippy::pedantic)]

// TODO: all 3 should be private
#[macro_use]
pub mod util;
pub mod layout;
pub mod renderer;

mod document;
//mod windowing;

pub mod css;

// TODO: feature-flag
// windowing::*,
pub use self::document::{Document, NodeId, NodeKind};

// TODO: feature-flag
mod ffi;
