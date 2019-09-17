use std::mem;
use std::ptr;
use std::ffi::CString;
use gl::types::*;

use crate::generated::Color;
use crate::commons::{Pos};
use crate::render::{Frame, Batch, Vertex, VertexIndex};

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitive batches, prepared by higher-level `Renderer`
///
/// TODO: extract trait, provide other implementations
pub struct RenderBackend {
    rect_program: u32,

    ibo: u32,
    vbo: u32,

    // TODO: shared buffers (text)
}

impl RenderBackend {
    pub(crate) fn new() -> Self {
        unsafe {
            // not used but webgl & opengl core profile require it
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            check();

            let mut ibo = 0;
            let mut vbo = 0;

            gl::GenBuffers(1, &mut ibo);
            gl::GenBuffers(1, &mut vbo);

            Self {
                rect_program: shader_program(RECT_VS, RECT_FS),

                ibo,
                vbo,
            }
        }
    }

    pub(crate) fn render_frame(&mut self, frame: Frame) {
        unsafe {
            // TODO: opaque rect in bg (last item) might be faster
            // clear needs to fill all pixels, bg rect fills only what's left
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            // TODO: | DEPTH_BUFFER_BIT
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // upload indices
            // (can stay bound for the whole time)
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<VertexIndex>() * frame.indices.len()) as isize, mem::transmute(&frame.indices[0]), gl::STATIC_DRAW);
            check();

            // upload opaque & alpha vertices
            // TODO: opaque
            // (have to be bound again during rendering)
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, frame.alpha_data.len() as isize, mem::transmute(&frame.alpha_data[0]), gl::STATIC_DRAW);
            check();

            // TODO: upload textures (and shared data if needed)

            for b in frame.batches {
                self.draw_batch(b);
            }
        }
    }

    unsafe fn draw_batch(&mut self, batch: Batch) {
        let vertices_count;
        let offset = 0;

        match batch {
            Batch::AlphaRects { num } => {
                vertices_count = num * 6;

                gl::UseProgram(self.rect_program);
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (mem::size_of::<Vertex<Color>>()) as GLint,
                    0 as *const GLvoid,
                );
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(
                    1,
                    4,
                    gl::UNSIGNED_BYTE,
                    gl::FALSE,
                    (mem::size_of::<Vertex<Color>>()) as GLint,
                    (mem::size_of::<Pos>()) as *const std::ffi::c_void,
                );

                /*
            }
            _ => {
                // everything else is alpha, with shared indices buffer
                gl::Enable(gl::ALPHA);
                */
            }
        }

        gl::DrawElements(gl::TRIANGLES, vertices_count as i32, gl::UNSIGNED_SHORT, (offset * std::mem::size_of::<VertexIndex>()) as *const std::ffi::c_void);
        check();
    }
}




const RECT_VS: &str = r#"
  #version 100

  attribute vec2 a_pos;
  attribute vec4 a_color;

  varying vec4 v_color;

  void main() {
    // TODO: uniforms
    vec2 size = vec2(1024., 768.);
    vec2 xy = (a_pos / (size / 2.)) - 1.;
    xy.y *= -1.;

    gl_Position = vec4(xy, 0.0, 1.0);
    v_color = a_color;
  }
"#;

const RECT_FS: &str = r#"
  #version 100

  precision mediump float;

  varying vec4 v_color;

  void main() {
    gl_FragColor = v_color / 256.;
  }
"#;

unsafe fn shader_program(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    let vertex_shader = shader(gl::VERTEX_SHADER, vertex_shader_source);
    let fragment_shader = shader(gl::FRAGMENT_SHADER, fragment_shader_source);

    let program = gl::CreateProgram();
    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    let mut success = gl::FALSE as GLint;

    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

    if success != gl::TRUE as GLint {
        panic!(get_program_info_log(program));
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    program
}

unsafe fn shader(shader_type: u32, source: &str) -> u32 {
    let shader = gl::CreateShader(shader_type);

    gl::ShaderSource(
        shader,
        1,
        &(CString::new(source.as_bytes()).expect("get CString")).as_ptr(),
        ptr::null(),
    );
    gl::CompileShader(shader);

    let mut success = gl::FALSE as GLint;

    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != gl::TRUE as GLint {
        panic!(get_shader_info_log(shader));
    }

    shader
}

unsafe fn check() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        panic!("gl err {}", err);
    }
}

unsafe fn get_shader_info_log(shader: GLuint) -> String {
    let mut len = 0;
    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0i8; len as usize];
    gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr());
    buf.set_len(len as usize);
    String::from_utf8_unchecked(mem::transmute(buf))
}

unsafe fn get_program_info_log(program: GLuint) -> String {
    let mut len = 0;
    gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0i8; len as usize];
    gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr());
    String::from_utf8_unchecked(mem::transmute(buf))
}
