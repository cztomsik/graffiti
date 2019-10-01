use std::mem;
use std::ptr;
use std::ffi::CString;
use gl::types::*;

use crate::commons::{Pos, Color};
use crate::render::{Frame, Batch, Vertex, VertexIndex};

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitive batches, prepared by higher-level `Renderer`
///
/// TODO: transpile shaders for different devices (raspi)
///       (maybe macros?)
///
/// TODO: extract trait, provide other implementations
pub struct RenderBackend {
    rect_program: u32,
    text_program: u32,
    text_uniform: i32,

    ibo: u32,
    vbo: u32,

    // TODO: frame-shared buffers (text)
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

            // TODO
            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
            // because of RGB
            //gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 64, 64, 0, gl::RGB, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 512, 512, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            check();

            // TODO: opaque
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD);

            let rect_program = shader_program(RECT_VS, RECT_FS);
            let text_program = shader_program(TEXT_VS, TEXT_FS);

            // this is important otherwise indices sometimes does not reflect
            // the order in the shader!!!
            // TODO: works but it should be done before linking
            gl::BindAttribLocation(rect_program, 0, CString::new("a_pos").unwrap().as_ptr());
            gl::BindAttribLocation(rect_program, 1, CString::new("a_color").unwrap().as_ptr());

            gl::BindAttribLocation(text_program, 0, CString::new("a_pos").unwrap().as_ptr());
            gl::BindAttribLocation(text_program, 1, CString::new("a_uv").unwrap().as_ptr());

            Self {
                rect_program,
                text_program,

                text_uniform: gl::GetUniformLocation(text_program, CString::new("u_color").unwrap().as_ptr()),

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

            // buffer sharing
            let mut vbo_offset = 0;
            let mut ibo_offset = 0;

            for b in frame.batches {
                let vertex_size;
                let quad_count;

                // TODO: every quad vertex has pos -> VertexAttribPointer for the first attr can be set once
                match b {
                    Batch::AlphaRects { num } => {
                        vertex_size = mem::size_of::<Vertex<Color>>();
                        quad_count = num;

                        gl::UseProgram(self.rect_program);
                        gl::EnableVertexAttribArray(0);
                        gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            vertex_size as GLint,
                            vbo_offset as *const std::ffi::c_void,
                        );
                        gl::EnableVertexAttribArray(1);
                        gl::VertexAttribPointer(
                            1,
                            4,
                            gl::UNSIGNED_BYTE,
                            gl::FALSE,
                            vertex_size as GLint,
                            (vbo_offset + mem::size_of::<Pos>()) as *const std::ffi::c_void,
                        );
                    }
                    Batch::Text { color, num } => {
                        vertex_size = mem::size_of::<Vertex<Pos>>();
                        quad_count = num;

                        gl::UseProgram(self.text_program);
                        gl::EnableVertexAttribArray(0);
                        gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            vertex_size as GLint,
                            vbo_offset as *const std::ffi::c_void,
                        );
                        gl::EnableVertexAttribArray(1);
                        gl::VertexAttribPointer(
                            1,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            vertex_size as GLint,
                            (vbo_offset + mem::size_of::<Pos>()) as *const std::ffi::c_void,
                        );

                        // unpack it here, maybe even in builder
                        let color: [f32; 4] = [color.r as f32 / 256., color.g as f32 / 256., color.b as f32 / 256., color.a as f32 / 256.];
                        gl::Uniform4fv(self.text_uniform, 1, &color as *const GLfloat);
                    }
                    /*
                    _ => {
                        // everything else is alpha, with shared indices buffer
                        gl::Enable(gl::ALPHA);
                    }
                    */
                }

                gl::DrawElements(gl::TRIANGLES, (quad_count * 6) as i32, gl::UNSIGNED_SHORT, ibo_offset as *const std::ffi::c_void);
                check();

                vbo_offset += quad_count * 4 * vertex_size;
                ibo_offset += quad_count * 6 * mem::size_of::<VertexIndex>();
            }
        }
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
            // TODO: move division to VS
            gl_FragColor = v_color / 256.;
      }
"#;

const TEXT_VS: &str = r#"
      #version 100

      attribute vec2 a_pos;
      attribute vec2 a_uvv;

      varying vec2 v_uv;

      void main() {
            // TODO: uniforms
            vec2 size = vec2(1024., 768.);
            vec2 xy = (a_pos / (size / 2.)) - 1.;
            xy.y *= -1.;

            gl_Position = vec4(xy, 0.0, 1.0);
            v_uv = a_uvv;
      }
"#;

const TEXT_FS: &str = r#"
      #version 100

      precision mediump float;

      uniform vec4 u_color;
      uniform sampler2D u_texture;

      varying vec2 v_uv;

      float median(vec3 col) {
            return max(min(col.r, col.g), min(max(col.r, col.g), col.b));
      }

      void main() {
            // TODO: seems like it's BGRA instead of RGBA
            float distance = median(texture2D(u_texture, v_uv).rgb);
            // find out in what ranges (-3-3 ?) the distances actually are

            // 0.4 - 0.6 looks good for big sizes
            float alpha = smoothstep(0.3, 0.7, 1. - distance);

            gl_FragColor = vec4(u_color.rgb, alpha * u_color.a);
      }
"#;

const SDF_TEXTURE: &[u8; 512 * 512 * 4] = include_bytes!("../../../../sheet0.raw");

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
    gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
    buf.set_len(len as usize);
    String::from_utf8_unchecked(mem::transmute(buf))
}

unsafe fn get_program_info_log(program: GLuint) -> String {
    let mut len = 0;
    gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0i8; len as usize];
    gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
    String::from_utf8_unchecked(mem::transmute(buf))
}
