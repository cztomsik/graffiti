use super::super::{DrawOp, Frame, TexData, Vertex};
use super::RenderBackend;
use glow::*;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

/// Super-simple OpenGL 2.1 backend
/// - one vbo
/// - one shader
/// - one texture
/// - drawArrays
pub struct GlBackend {
    // !Send, !Sync
    _marker: PhantomData<*mut ()>,

    gl: glow::Context,
    vao: Option<NativeVertexArray>,
    vbo: NativeBuffer,
    tex: NativeTexture,
    program: NativeProgram,
}

macro_rules! offsetof {
    ($type:ident . $field:ident $(,)?) => {{
        // ptr::null() didnt work in --release
        let uninit = std::mem::MaybeUninit::<$type>::uninit();
        (&(*uninit.as_ptr()).$field as *const _ as *const c_void).sub(uninit.as_ptr() as _)
    }};
}

impl GlBackend {
    // TODO: could be safe if everything happenned in App::push_task()
    pub unsafe fn new(load_symbol: impl FnMut(&str) -> *const c_void) -> Self {
        let gl = glow::Context::from_loader_function(load_symbol);

        let STRIDE = mem::size_of::<Vertex>() as _;

        // not used but webgl & opengl core profile require it
        let vao = gl.create_vertex_array().ok();
        gl.bind_vertex_array(vao);
        check(&gl, "vao");

        gl.disable(DEPTH_TEST);
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        gl.blend_equation(FUNC_ADD);

        // one vbo
        let vbo = gl.create_buffer().expect("create vbo");
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        check(&gl, "vbo");

        // one texture
        let tex = gl.create_texture().expect("create tex");
        gl.bind_texture(TEXTURE_2D, Some(tex));
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as _);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as _);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as _);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as _);
        // needed if RGB
        //gl.pixel_store_i32(UNPACK_ALIGNMENT, 1);
        check(&gl, "texture");

        // one shader
        let program = create_program(&gl, VS, FS);
        gl.use_program(Some(program));

        // setup attrs
        let a_pos = gl.get_attrib_location(program, "a_pos").unwrap();
        gl.enable_vertex_attrib_array(a_pos);
        gl.vertex_attrib_pointer_f32(a_pos, 2, FLOAT, false, STRIDE, offsetof!(Vertex.xy) as _);
        check(&gl, "a_pos");

        let a_uv = gl.get_attrib_location(program, "a_uv").unwrap();
        gl.enable_vertex_attrib_array(a_uv);
        gl.vertex_attrib_pointer_f32(a_uv, 2, UNSIGNED_SHORT, false, STRIDE, offsetof!(Vertex.uv) as _);
        check(&gl, "a_uv");

        let a_color = gl.get_attrib_location(program, "a_color").unwrap();
        gl.enable_vertex_attrib_array(a_color);
        gl.vertex_attrib_pointer_f32(a_color, 4, UNSIGNED_BYTE, true, STRIDE, offsetof!(Vertex.color) as _);
        check(&gl, "a_color");

        Self {
            _marker: PhantomData,
            gl,
            vao,
            vbo,
            tex,
            program,
        }
    }
}

impl Drop for GlBackend {
    fn drop(&mut self) {
        unsafe {
            if let Some(vao) = self.vao {
                self.gl.delete_vertex_array(vao);
            }

            self.gl.delete_buffer(self.vbo);
            self.gl.delete_texture(self.tex);
            self.gl.delete_program(self.program);
        }
    }
}

impl RenderBackend for GlBackend {
    // TODO: unsafe because it depends on (right) context being active
    fn render_frame(&self, frame: Frame) {
        unsafe {
            // TODO: uniform
            // TODO: glViewport(0, 0, width, height);

            self.gl.buffer_data_u8_slice(
                ARRAY_BUFFER,
                std::slice::from_raw_parts(frame.vertices.as_ptr() as _, mem::size_of_val(&frame.vertices[..]) as _),
                DYNAMIC_DRAW,
            );
            check(&self.gl, "upload vbo");

            self.gl.clear_color(1., 1., 1., 1.);
            self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            let mut index: i32 = 0;

            for op in frame.draw_ops {
                match op {
                    DrawOp::TexData(TexData { width, height, pixels }) => {
                        self.gl.bind_texture(TEXTURE_2D, Some(self.tex));
                        self.gl.tex_image_2d(
                            TEXTURE_2D,
                            0,
                            RGBA as _,
                            width,
                            height,
                            0,
                            RGBA,
                            UNSIGNED_BYTE,
                            Some(std::slice::from_raw_parts(
                                pixels.as_ptr() as _,
                                mem::size_of_val(&pixels[..]) as _,
                            )),
                        );
                        self.gl.active_texture(TEXTURE0);
                    }

                    DrawOp::DrawArrays(num) => {
                        self.gl.draw_arrays(TRIANGLES, index, num as _);
                        index += num as i32;
                    }
                }
            }
        }
    }
}

pub const VS: &str = r#"
#version 100

//uniform mat3 u_projection;

attribute vec2 a_pos;
attribute vec2 a_uv;
attribute vec4 a_color;

varying vec4 v_color;
varying vec2 v_uv;

void main() {
    vec2 size = vec2(1024., 768.);
    vec2 xy = (a_pos.xy / (size / 2.)) - 1.;

    // TODO: Z
    gl_Position = vec4(xy.x, xy.y * -1., 0.5, 1.0);
    // TODO: TEXTURE_SIZE uniform
    v_uv = a_uv / vec2(1024, 1024);
    v_color = a_color;
}
"#;

pub const FS: &str = r#"
#version 100

precision mediump float;

uniform sampler2D tex;

varying vec2 v_uv;
varying vec4 v_color;

void main() {
    gl_FragColor = v_color * texture2D(tex, v_uv);
}
"#;

// gl utils

unsafe fn create_program(gl: &glow::Context, vs: &str, fs: &str) -> NativeProgram {
    let vs = shader(gl, VERTEX_SHADER, vs);
    let fs = shader(gl, FRAGMENT_SHADER, fs);

    let program = gl.create_program().unwrap();
    gl.attach_shader(program, vs);
    gl.attach_shader(program, fs);
    gl.link_program(program);

    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    program
}

unsafe fn shader(gl: &glow::Context, shader_type: u32, source: &str) -> NativeShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    }

    shader
}

unsafe fn check(gl: &glow::Context, hint: &str) {
    let err = gl.get_error();
    if err != NO_ERROR {
        panic!("gl err {} near {}", err, hint);
    }
}
