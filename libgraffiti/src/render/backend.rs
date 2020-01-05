use std::mem;
use std::ptr;
use std::ffi::CString;
use gl::types::*;

use crate::commons::{Pos, Color};
use crate::render::{Frame, Batch, Vertex, VertexIndex};

/// Low-level renderer, specific to the given graphics api (OpenGL/Vulkan/SW)
/// Knows how to draw primitive batches, prepared by higher-level `Renderer`
///
/// Backend does the real drawing but it's also very simple and can't do any
/// optimizations and has absolutely no idea about scene, surfaces or anything else.
/// You don't want to use it directly and so it's useless just by itself.
///
/// TODO: transpile shaders for different devices (raspi)
///       (maybe GLSL macros?)
///
/// TODO: extract trait, provide other implementations
pub struct RenderBackend {
    rect_program: u32,
    text_program: u32,
    resize_uniforms: [(u32, i32); 2],
    text_color_uniform: i32,
    text_factor_uniform: i32,

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
            check("vao");

            let mut ibo = 0;
            let mut vbo = 0;

            gl::GenBuffers(1, &mut ibo);
            gl::GenBuffers(1, &mut vbo);

            // TODO
            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            // TODO: mipmap could improve small sizes but I'm not sure if it wouldn't need additional texture space
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            // because of RGB
            //gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 64, 64, 0, gl::RGB, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, 512, 512, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            check("texture");

            // TODO: opaque
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD);

            let rect_program = shader_program(RECT_VS, RECT_FS);
            check("rect_program");
            let text_program = shader_program(TEXT_VS, TEXT_FS);
            check("text_program");

            // this is important otherwise indices sometimes does not reflect
            // the order in the shader!!!
            // TODO: works but it should be done before linking
            gl::BindAttribLocation(rect_program, 0, c_str!("a_pos"));
            gl::BindAttribLocation(rect_program, 1, c_str!("a_color"));
            check("rect attrs");

            gl::BindAttribLocation(text_program, 0, c_str!("a_pos"));
            gl::BindAttribLocation(text_program, 1, c_str!("a_uv"));
            check("text attrs");

            let resize_uniforms = [
                (rect_program, gl::GetUniformLocation(rect_program, c_str!("u_win_size"))),
                (text_program, gl::GetUniformLocation(text_program, c_str!("u_win_size")))
            ];
            let text_color_uniform = gl::GetUniformLocation(text_program, c_str!("u_color"));
            let text_factor_uniform = gl::GetUniformLocation(text_program, c_str!("u_dist_factor"));
            check("uniforms");

            Self {
                rect_program,
                text_program,

                resize_uniforms,
                text_color_uniform,
                text_factor_uniform,

                ibo,
                vbo,
            }
        }
    }

    pub(crate) fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            for (program, uniform) in &self.resize_uniforms {
                gl::UseProgram(*program);
                gl::Uniform2f(*uniform, width as f32, height as f32);
            }
            check("resize");
        }
    }

    pub(crate) fn render_frame(&mut self, frame: Frame) {
        silly!("frame {:?}", &frame);

        unsafe {
            // TODO: opaque rect in bg (last item) might be faster
            // clear needs to fill all pixels, bg rect fills only what's left
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            // TODO: | DEPTH_BUFFER_BIT
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if frame.indices.is_empty() {
                return;
            }

            // upload indices
            // (can stay bound for the whole time)
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mem::size_of::<VertexIndex>() * frame.indices.len()) as isize, mem::transmute(&frame.indices[0]), gl::STATIC_DRAW);
            check("ibo");

            // upload opaque & alpha vertices
            // TODO: opaque
            // (have to be bound again during rendering)
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, frame.alpha_data.len() as isize, mem::transmute(&frame.alpha_data[0]), gl::STATIC_DRAW);
            check("vbo");

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
                    Batch::Text { color, distance_factor, num } => {
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
                        gl::Uniform4fv(self.text_color_uniform, 1, &color as *const GLfloat);
                        gl::Uniform1f(self.text_factor_uniform, distance_factor);
                    }
                    /*
                    _ => {
                        // everything else is alpha, with shared indices buffer
                        gl::Enable(gl::ALPHA);
                    }
                    */
                }

                gl::DrawElements(gl::TRIANGLES, (quad_count * 6) as i32, gl::UNSIGNED_SHORT, ibo_offset as *const std::ffi::c_void);
                check("draw els");

                vbo_offset += quad_count * 4 * vertex_size;
                ibo_offset += quad_count * 6 * mem::size_of::<VertexIndex>();
            }
        }
    }
}

const RECT_VS: &str = include_str!("shaders/rect.vert");
const RECT_FS: &str = include_str!("shaders/rect.frag");
const TEXT_VS: &str = include_str!("shaders/text.vert");
const TEXT_FS: &str = include_str!("shaders/text.frag");

const SDF_TEXTURE: &[u8; 512 * 512 * 4] = include_bytes!("../../resources/sheet0.raw");

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

unsafe fn check(hint: &str) {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        panic!("gl err {} near {}", err, hint);
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
