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
  clippy::enum_glob_use
)]

#[macro_use]
mod util;

mod app;
mod css;
mod document;
mod layout;
mod renderer;
mod webview;
mod window;

pub use self::{
  app::App,
  css::CssStyleDeclaration,
  document::{Document, NodeId, NodeType},
  renderer::Renderer,
  webview::WebView,
  window::{Event, Window, WindowId},
};
pub mod gfx;

// TODO: feature-flag
// mod ffi;
