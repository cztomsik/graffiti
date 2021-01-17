// low-level rendering
// x inspired by imgui backend
// x super-simple to implement
//   (or to integrate to an existing game engine pipeline)

pub trait RenderBackend {
  fn render_frame(&mut self, frame: Frame);
}

#[derive(Debug)]
pub struct Frame {
  pub quads: Vec<Quad>,
  pub draw_calls: Vec<DrawCall>,
}

impl Frame {
  pub fn new() -> Self {
      Self {
          quads: Vec::with_capacity(256),
          draw_calls: Vec::with_capacity(32),
      }
  }
}

#[derive(Debug)]
pub struct DrawCall {
  // TODO: scissor_rect, texture
  pub len: usize
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quad {
  pub vertices: [Vertex; 4],
}

// TODO: compress? https://dev.to/keaukraine/optimization-of-opengl-es-vertex-data-15d0
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
  pub xyz: [f32; 3],
  pub uv: [f32; 2],
  pub color: [u8; 4],
}

mod gl;

pub use gl::GlBackend;
