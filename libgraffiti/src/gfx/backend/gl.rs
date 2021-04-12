#![allow(unused, non_snake_case)]

use super::super::{DrawOp, Frame, TexData, Vertex};
use super::RenderBackend;
use std::marker::PhantomData;

/// Super-simple OpengGL 2.1 backend
/// - one vbo
/// - one shader
/// - one texture
/// - drawArrays
pub struct GlBackend {
    // !Send, !Sync
    _marker: PhantomData<*mut ()>,

    vao: GLuint,
    vbo: GLuint,
    tex: GLuint,
    program: GLuint,
}

macro_rules! offsetof {
    ($type:ident . $field:ident $(,)?) => {{
        // ptr::null() didnt work in --release
        let uninit = std::mem::MaybeUninit::<$type>::uninit();
        (&(*uninit.as_ptr()).$field as *const _ as *const c_void).sub(uninit.as_ptr() as _)
    }};
}

impl GlBackend {
    pub unsafe fn load_with(load_symbol: impl FnMut(&str) -> *mut c_void) {
        self::load_with(load_symbol)
    }

    pub fn new() -> Self {
        let STRIDE = mem::size_of::<Vertex>() as _;

        unsafe {
            // not used but webgl & opengl core profile require it
            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
            check("vao");

            glDisable(GL_DEPTH_TEST);
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            glBlendEquation(GL_FUNC_ADD);

            // one vbo
            let mut vbo = 0;
            glGenBuffers(1, &mut vbo);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            assert_ne!(vbo, 0);
            check("vbo");

            // one texture
            let mut tex = 0;
            glGenTextures(1, &mut tex);
            glBindTexture(GL_TEXTURE_2D, tex);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as _);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as _);
            //glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as _);
            //glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as _);
            // needed if RGB
            //glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            check("texture");

            // one shader
            let program = create_program(VS, FS);
            glUseProgram(program);

            // setup attrs
            let a_pos = glGetAttribLocation(program, c_str!("a_pos")) as _;
            glEnableVertexAttribArray(a_pos);
            glVertexAttribPointer(a_pos, 2, GL_FLOAT, GL_FALSE, STRIDE, offsetof!(Vertex.xy));
            check("a_pos");

            let a_uv = glGetAttribLocation(program, c_str!("a_uv")) as _;
            glEnableVertexAttribArray(a_uv);
            glVertexAttribPointer(a_uv, 2, GL_FLOAT, GL_FALSE, STRIDE, offsetof!(Vertex.uv));
            check("a_uv");

            let a_color = glGetAttribLocation(program, c_str!("a_color")) as _;
            glEnableVertexAttribArray(a_color);
            glVertexAttribPointer(a_color, 4, GL_UNSIGNED_BYTE, GL_TRUE, STRIDE, offsetof!(Vertex.color));
            check("a_color");

            Self {
                _marker: PhantomData,
                vao,
                vbo,
                tex,
                program,
            }
        }
    }
}

impl Drop for GlBackend {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &mut self.vbo);
            glDeleteVertexArrays(1, &mut self.vao);
            glDeleteTextures(1, &mut self.tex);
            glDeleteProgram(self.program);
        }
    }
}

impl RenderBackend for GlBackend {
    fn render_frame(&mut self, frame: Frame) {
        unsafe {
            // TODO: uniform
            // TODO: glViewport(0, 0, width, height);

            glBufferData(
                GL_ARRAY_BUFFER,
                mem::size_of_val(&frame.vertices[..]) as _,
                frame.vertices.as_ptr() as _,
                GL_DYNAMIC_DRAW,
            );
            check("upload vbo");

            glClearColor(1., 1., 1., 1.);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            let mut index: GLint = 0;

            for op in frame.draw_ops {
                match op {
                    DrawOp::TexData(TexData { width, height, pixels }) => {
                        glBindTexture(GL_TEXTURE_2D, self.tex);
                        glTexImage2D(
                            GL_TEXTURE_2D,
                            0,
                            GL_RGBA as _,
                            width,
                            height,
                            0,
                            GL_RGBA,
                            GL_UNSIGNED_BYTE,
                            pixels.as_ptr() as _,
                        );
                        glActiveTexture(GL_TEXTURE0);
                    }

                    DrawOp::DrawArrays(num) => {
                        glDrawArrays(GL_TRIANGLES, index, num as _);
                        index += num as GLint;
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
    v_uv = a_uv;
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

use std::ffi::CString;
use std::mem;
use std::ptr;

unsafe fn create_program(vs: &str, fs: &str) -> GLuint {
    let vs = shader(GL_VERTEX_SHADER, vs);
    check("vertex shader");

    let fs = shader(GL_FRAGMENT_SHADER, fs);
    check("fragment shader");

    let program = glCreateProgram();
    glAttachShader(program, vs);
    glAttachShader(program, fs);
    glLinkProgram(program);

    let mut success = GL_FALSE as GLint;
    glGetProgramiv(program, GL_LINK_STATUS, &mut success);

    if success != GL_TRUE as GLint {
        panic!("{}", get_program_info_log(program));
    }

    program
}

unsafe fn shader(shader_type: GLuint, source: &str) -> GLuint {
    let shader = glCreateShader(shader_type);
    glShaderSource(shader, 1, &*c_str!(source), ptr::null());
    glCompileShader(shader);

    let mut success = GL_FALSE as GLint;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);

    if success != GL_TRUE as GLint {
        panic!("{}", get_shader_info_log(shader));
    }

    shader
}

unsafe fn check(hint: &str) {
    let err = glGetError();
    if err != GL_NO_ERROR {
        panic!("gl err {} near {}", err, hint);
    }
}

unsafe fn get_shader_info_log(shader: GLuint) -> String {
    let mut len = 0;
    glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0i8; len as usize];
    glGetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
    buf.set_len(len as usize);

    String::from_utf8_unchecked(mem::transmute(buf))
}

unsafe fn get_program_info_log(program: GLuint) -> String {
    let mut len = 0;
    glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0i8; len as usize];
    glGetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
    buf.set_len(len as usize);

    String::from_utf8_unchecked(mem::transmute(buf))
}

// hand-written opengl bindings/loader
// - retains glXxXx() naming so it matches examples from other langs
// - no deps, faster to compile
// - explicit about what we really need
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_uint, c_void};

// gl types
type GLenum = c_uint;
type GLboolean = c_uchar;
type GLbitfield = c_uint;
type GLint = c_int;
type GLuint = c_uint;
type GLsizei = c_int;
type GLfloat = c_float;
type GLchar = c_char;
type GLsizeiptr = isize;

// consts
const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;
const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;
//const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;
const GL_FALSE: GLboolean = 0;
const GL_TRUE: GLboolean = 1;
const GL_TRIANGLES: GLenum = 0x0004;
const GL_SRC_ALPHA: GLenum = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
const GL_FUNC_ADD: GLenum = 0x8006;
const GL_ARRAY_BUFFER: GLenum = 0x8892;
//const GL_STREAM_DRAW: GLenum = 0x88E0;
//const GL_STATIC_DRAW: GLenum = 0x88E4;
const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
const GL_TEXTURE_2D: GLenum = 0x0DE1;
const GL_BLEND: GLenum = 0x0BE2;
const GL_DEPTH_TEST: GLenum = 0x0B71;
const GL_GREATER: GLenum = 0x0204;
const GL_NO_ERROR: GLenum = 0;
const GL_UNSIGNED_BYTE: GLenum = 0x1401;
const GL_FLOAT: GLenum = 0x1406;
const GL_RGBA: GLenum = 0x1908;
const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
const GL_VERTEX_SHADER: GLenum = 0x8B31;
const GL_LINK_STATUS: GLenum = 0x8B82;
const GL_LINEAR: GLenum = 0x2601;
const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
const GL_TEXTURE0: GLenum = 0x84C0;
const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
const GL_COMPILE_STATUS: GLenum = 0x8B81;
const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;

// what we need
dylib! {
    extern "C" {
        // err
        fn glGetError() -> GLenum;
        fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
        fn glGetProgramInfoLog(program: GLuint, buf_size: GLsizei, len: *mut GLsizei, log: *mut GLchar);
        fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
        fn glGetShaderInfoLog(shader: GLuint, buf_size: GLsizei, len: *mut GLsizei, log: *mut GLchar);

        // vao
        fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
        fn glBindVertexArray(vao: GLuint);
        fn glDeleteVertexArrays(n: GLsizei, arrays: *mut GLuint);

        // program, shaders
        fn glCreateProgram() -> GLuint;
        fn glUseProgram(program: GLuint);
        fn glCreateShader(kind: GLenum) -> GLuint;
        fn glShaderSource(shader: GLuint, count: GLsizei, source: *const *const GLchar, len: *const GLint);
        fn glCompileShader(shader: GLuint);
        fn glAttachShader(program: GLuint, shader: GLuint);
        fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
        fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
        fn glLinkProgram(program: GLuint);
        fn glDeleteShader(shader: GLuint);
        fn glDeleteProgram(shader: GLuint);

        // tex
        fn glGenTextures(n: GLsizei, textures: *mut GLuint);
        fn glBindTexture(target: GLenum, tex: GLuint);
        fn glTexParameteri(target: GLenum, pname: GLenum, params: GLint);
        fn glTexImage2D(target: GLenum, level: GLint, format: GLint, width: GLsizei, height: GLsizei, border: GLint, internal_format: GLenum, kind: GLenum, pixels: *const c_void);
        fn glActiveTexture(tex: GLenum);
        fn glDeleteTextures(n: GLsizei, textures: *mut GLuint);

        // rendering
        fn glViewport(x: GLint, y: GLint, w: GLsizei, h: GLsizei);
        fn glEnable(cap: GLenum);
        fn glDisable(cap: GLenum);
        fn glClear(mask: GLbitfield);
        fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
        fn glDepthMask(flag: GLboolean);
        fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
        fn glBlendEquation(mode: GLenum);
        fn glUniform1f(loc: GLint, v: GLfloat);
        fn glUniform2f(loc: GLint, v0: GLfloat, v1: GLfloat);
        fn glUniform4fv(loc: GLint, count: GLsizei, value: *const GLfloat);
        fn glUniformMatrix3fv(loc: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
        fn glGenBuffers(n: GLsizei, bufs: *mut GLuint);
        fn glBindBuffer(target: GLenum, buf: GLuint);
        fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
        fn glEnableVertexAttribArray(index: GLuint);
        fn glVertexAttribPointer(index: GLuint, size: GLint, kind: GLenum, normalized: GLboolean, stride: GLsizei, ptr: *const c_void);
        fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
        fn glDeleteBuffers(n: GLsizei, bufs: *mut GLuint);
    }
}
