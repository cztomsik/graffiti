#[macro_use]
extern crate log;

mod api;
mod scene;
mod window;
mod layout;
mod render;
mod storage;
mod ffi;
mod generated;
mod temp;

// temporarily here until we make respective parts generic
pub type Id = generated::SurfaceId;
