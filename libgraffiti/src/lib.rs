#![warn(clippy::all, clippy::pedantic)]
#![allow(
  unused,
  clippy::module_name_repetitions,
  clippy::must_use_candidate,
  clippy::missing_panics_doc,
  clippy::wildcard_imports,
  clippy::missing_safety_doc,
  clippy::cast_possible_truncation,
  clippy::cast_precision_loss,
  clippy::cast_lossless,
  clippy::cast_sign_loss,
  clippy::enum_glob_use,
  clippy::missing_errors_doc,
  clippy::type_complexity
)]

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
