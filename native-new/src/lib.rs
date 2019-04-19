#[macro_use]
extern crate log;

mod api;
mod app;
mod window;
mod scene;
mod layout;
mod text;
mod render;
mod storage;
mod ffi;
mod generated;

// temporarily here until we make respective parts generic
pub type Id = generated::SurfaceId;
