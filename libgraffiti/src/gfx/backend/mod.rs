use super::Frame;

pub trait RenderBackend {
  fn render_frame(&mut self, frame: Frame);
}

mod gl;
pub use gl::GlBackend;
