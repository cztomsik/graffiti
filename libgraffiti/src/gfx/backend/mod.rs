use super::Frame;

pub trait RenderBackend {
  fn render_frame(&self, frame: Frame);
}

mod gl;
pub use gl::GlBackend;
