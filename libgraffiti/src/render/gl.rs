#![allow(non_snake_case)]

// straight-forward drawArrays-based primitive renderer
// there was some perf-issue with indexed drawing
// and this is much simpler too

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::convert::TryInto;

use crate::commons::{Pos, Bounds, Color, TextId};
use super::{RenderBackend};


pub struct GlRenderBackend {
    rects: Vec<Rect>,
    ops: Vec<RenderOp>,

    gpu: Gpu,
    rect_program: ShaderProgram,
    text_program: ShaderProgram,

    vbo: GlBuffer,
    texts: Vec<(GlBuffer, GLint, f32)>,
}

#[derive(Debug)]
enum RenderOp {
    PushTransform { transform: Mat3 },
    PopTransform,
    DrawRects { count: GLint },
    DrawText { id: TextId, pos: Pos, color: Color },
}

// we can't use instanced drawing (UBOs) on raspi
// so we need real vertices
type Rect = Quad<Color>;
type Glyph = Quad<Pos>;

#[derive(Debug, Clone, Copy)]
struct Quad<T>([Vertex<T>; 6]);

impl <T: Copy> Quad<T> {
    const VERTEX_SIZE: GLint = mem::size_of::<Vertex<T>>() as GLint;

    fn new(bounds: Bounds, data: [T; 4]) -> Self {
        Self([
                Vertex(bounds.a, data[0]),
                Vertex(Pos::new(bounds.b.x, bounds.a.y), data[1]),
                Vertex(Pos::new(bounds.a.x, bounds.b.y), data[2]),
                Vertex(Pos::new(bounds.a.x, bounds.b.y), data[2]),
                Vertex(Pos::new(bounds.b.x, bounds.a.y), data[1]),
                Vertex(bounds.b, data[3])
        ])
    }
}

#[derive(Debug, Clone, Copy)]
struct Vertex<T>(Pos, T);

static mut GL_LOADED: bool = false;

impl GlRenderBackend {
    pub(crate) fn new() -> Self {
        unsafe {
            if !GL_LOADED {
                load_platform_gl();
                GL_LOADED = true;
            }

            let mut gpu = Gpu::new();
            let vbo = gpu.create_buffer();

            // TODO
            let mut tex = 0;
            glGenTextures(1, &mut tex);
            glBindTexture(GL_TEXTURE_2D, tex);
            // TODO: mipmap could improve small sizes but I'm not sure if it wouldn't need additional texture space
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
            // because of RGB
            //glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            //glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, 64, 64, 0, GL_RGB, GL_UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, 512, 512, 0, GL_RGBA, GL_UNSIGNED_BYTE, &SDF_TEXTURE[0] as *const u8 as *const std::ffi::c_void);
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, tex);
            check("texture");

            glDisable(GL_DEPTH_TEST);
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            glBlendEquation(GL_FUNC_ADD);

            let rect_program = gpu.create_program(
                include_str!("shaders/rect.vert"),
                include_str!("shaders/rect.frag"),
                &["u_transform"],
                &["a_pos", "a_color"]
            );

            let text_program = gpu.create_program(
                include_str!("shaders/text.vert"),
                include_str!("shaders/text.frag"),
                &["u_pos", "u_color", "u_dist_factor"],
                &["a_pos", "a_uv"]
            );

            Self {
                rects: Vec::new(),
                ops: Vec::new(),

                gpu,
                rect_program,
                text_program,

                vbo,
                texts: Vec::new(),
            }
        }
    }
}

impl RenderBackend for GlRenderBackend {
    fn realloc(&mut self, texts_count: TextId) {
        let Self { gpu, texts, .. } = self;

        // for now, each text has its own buffer
        // some ideas:
        // - keep short texts together
        // - keep together all texts set during given frame
        //   (so that old ones stay untouched)
        // - combine?
        texts.resize_with(texts_count, || unsafe { (gpu.create_buffer(), 0, 0.) })
    }

    fn set_text_glyphs(&mut self, text: TextId, size: f32, glyphs: impl Iterator<Item=(Bounds, Bounds)>) {
        let glyphs: Vec<Glyph> = glyphs.map(|(bounds, coords)| {
            Glyph::new(bounds, [
                coords.a,
                Pos::new(coords.b.x, coords.a.y),
                Pos::new(coords.a.x, coords.b.y),
                coords.b
            ])
        }).collect();

        let (vbo, glyphs_count, distance_factor) = &mut self.texts[text];

        unsafe { self.gpu.buffer_data(vbo, GL_ARRAY_BUFFER, &glyphs) }

        *glyphs_count = glyphs.len() as GLint;

        // TODO: read from font file
        let texture_font_size = 42.;
        let px_range = 3.;
        // https://github.com/Chlumsky/msdfgen/issues/22
        // https://github.com/Chlumsky/msdfgen/issues/36
        *distance_factor = (size / texture_font_size) * px_range;
    }

    fn clear(&mut self) {
        debug!("-- clear");

        unsafe {
            self.rects.set_len(0);
            self.ops.set_len(0);
        }

        self.ops.push(RenderOp::PushTransform {
            transform: Mat3([
                1., 0., 0.,
                0., 1., 0.,
                0., 0., 1.,
            ])
        });
    }

    fn push_transform(&mut self, mut transform: Mat3, origin: Pos) {
        // TODO: this will work for translate/scale() only
        // (multiply with translate(-origin), transform, translate(origin) again)

        transform.0[2] -= (transform.0[0] * origin.x) - origin.x;
        transform.0[5] -= (transform.0[4] * origin.y) - origin.y;

        self.ops.push(RenderOp::PushTransform { transform });
    }

    fn pop_transform(&mut self) {
        self.ops.push(RenderOp::PopTransform);
    }

    fn push_rect(&mut self, bounds: Bounds, color: Color) {
        self.rects.push(
            Rect::new(bounds, [color; 4])
        );

        if let Some(RenderOp::DrawRects { count }) = self.ops.last_mut() {
            *count += 1
        } else {
            self.ops.push(RenderOp::DrawRects { count: 1 })
        }
    }

    fn push_text(&mut self, id: TextId, pos: Pos, color: Color) {
        self.ops.push(RenderOp::DrawText { id, pos, color });
    }

    fn render(&mut self) {
        unsafe {
            glClearColor(1.0, 1.0, 1.0, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);

            // upload vbo
            // (have to be bound again during rendering)
            //
            // TODO: skip if up-to-date
            // (but keep that empty check because of &rects[0] ref)
            if !self.rects.is_empty() {
                self.gpu.buffer_data(&mut self.vbo, GL_ARRAY_BUFFER, &self.rects);
                check("upload vbo");
            }

            // TODO: avoid alloc
            let mut transform_stack = Vec::new();

            // all rects share one vbo
            let mut vbo_offset = 0;

            for op in &self.ops {
                match op {
                    RenderOp::PushTransform { transform } => {
                        // TODO: multiply by current
                        transform_stack.push(*transform);

                        // TODO: some prepare() which will do this (and skip if we're up-to-date)
                        // note we don't need index, we just need some (autoincreasing) id, which can be pushed to the stack with the matrix itself
                        // (maybe ops.len() could be used as identifier)
                        // but with index we could possibly premultiply it just once for many renders
                        glUseProgram(self.rect_program.id);
                        glUniformMatrix3fv(self.rect_program.uniforms[1].loc, 1, GL_FALSE, &transform.0 as *const f32);
                        check("set transform");
                    }

                    RenderOp::PopTransform => {
                        transform_stack.pop().unwrap();

                        glUseProgram(self.rect_program.id);
                        glUniformMatrix3fv(self.rect_program.uniforms[1].loc, 1, GL_FALSE, &transform_stack.last().unwrap().0 as *const f32);
                        check("restore transform");
                    }

                    RenderOp::DrawRects { count } => {
                        glUseProgram(self.rect_program.id);
                        self.gpu.bind_buffer(&self.vbo, GL_ARRAY_BUFFER);
                        glVertexAttribPointer(
                            self.rect_program.attributes[0].loc,
                            2,
                            GL_FLOAT,
                            GL_FALSE,
                            Rect::VERTEX_SIZE,
                            ptr::null(),
                        );
                        glVertexAttribPointer(
                            self.rect_program.attributes[1].loc,
                            4,
                            GL_UNSIGNED_BYTE,
                            GL_FALSE,
                            Rect::VERTEX_SIZE,
                            mem::size_of::<Pos>() as *const std::ffi::c_void,
                        );

                        // draw
                        glDrawArrays(GL_TRIANGLES, vbo_offset, count * 6);
                        check("draw els");

                        vbo_offset += count * 6;
                    }

                    RenderOp::DrawText { id, pos, color } => {
                        let (vbo, glyphs_count, distance_factor) = &self.texts[*id];

                        glUseProgram(self.text_program.id);
                        self.gpu.bind_buffer(vbo, GL_ARRAY_BUFFER);
                        glVertexAttribPointer(
                            self.text_program.attributes[0].loc,
                            2,
                            GL_FLOAT,
                            GL_FALSE,
                            Glyph::VERTEX_SIZE,
                            ptr::null(),
                        );
                        glVertexAttribPointer(
                            self.text_program.attributes[1].loc,
                            2,
                            GL_FLOAT,
                            GL_FALSE,
                            Glyph::VERTEX_SIZE,
                            mem::size_of::<Pos>() as *const std::ffi::c_void,
                        );

                        glUniform2f(self.text_program.uniforms[1].loc, pos.x, pos.y);

                        // unpack it here, maybe even in builder
                        let color: [f32; 4] = [color.r as f32 / 256., color.g as f32 / 256., color.b as f32 / 256., color.a as f32 / 256.];
                        glUniform4fv(self.text_program.uniforms[2].loc, 1, &color as *const GLfloat);

                        glUniform1f(self.text_program.uniforms[3].loc, *distance_factor);

                        // draw
                        glDrawArrays(GL_TRIANGLES, 0, *glyphs_count * 6);
                        check("draw text");
                    }
                }
            }
        }
    }

    fn resize(&mut self, (width, height): (f32, f32)) {
        let mat = Mat3([
            2. / width, 0., -1.,
            0., -2. / height, 1.,
            0., 0., 1.
        ]);

        unsafe {
            glUseProgram(self.rect_program.id);
            glUniformMatrix3fv(self.rect_program.uniforms[0].loc, 1, GL_FALSE, &mat.0 as *const f32);

            glUseProgram(self.text_program.id);
            glUniformMatrix3fv(self.text_program.uniforms[0].loc, 1, GL_FALSE, &mat.0 as *const f32);
            check("resize");
        }
    }
}

const SDF_TEXTURE: &[u8; 512 * 512 * 4] = include_bytes!("../../resources/sheet0.raw");

#[derive(Debug, Clone, Copy)]
pub struct Mat3(pub [f32;9]);

struct Gpu;

impl Gpu {
    unsafe fn new() -> Self {
        // not used but webgl & opengl core profile require it
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);
        check("vao");

        Self
    }

    unsafe fn create_program(&mut self, vs: &str, fs: &str, extra_uniforms: &[&str], attributes: &[&str]) -> ShaderProgram {
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
            panic!(get_program_info_log(program));
        }

        glDeleteShader(vs);
        glDeleteShader(fs);

        glUseProgram(program);

        let uniforms = ["u_projection"].iter().chain(extra_uniforms.iter()).map(|name| {
            let loc = glGetUniformLocation(program, c_str!(name.clone()));
            check("uniform location");

            ProgramUniform { loc }
        }).collect();

        let attributes = attributes.iter().map(|name| {
            let loc = glGetAttribLocation(program, c_str!(name.clone())).try_into().expect("invalid attr loc");
            check("uniform location");

            glEnableVertexAttribArray(loc);
            check("enable attrib arr");

            ProgramAttribute { loc }
        }).collect();

        ShaderProgram {
            id: program,
            uniforms,
            attributes,
        }
    }

    #[inline(always)]
    unsafe fn create_buffer(&mut self) -> GlBuffer {
        let mut id = 0;

        glGenBuffers(1, &mut id);
        check("gen buffer");

        GlBuffer { id }
    }

    #[inline(always)]
    unsafe fn bind_buffer(&mut self, buffer: &GlBuffer, target: GLuint) {
        glBindBuffer(target, buffer.id);
        check("bind buffer");
    }

    #[inline(always)]
    unsafe fn buffer_data<T>(&mut self, buffer: &mut GlBuffer, target: GLuint, data: &[T]) {
        let size = (mem::size_of::<T>() * data.len()) as GLsizeiptr;

        self.bind_buffer(buffer, target);

        // orphaning
        // https://www.seas.upenn.edu/~pcozzi/OpenGLInsights/OpenGLInsights-AsynchronousBufferTransfers.pdf
        glBufferData(target, size, std::ptr::null_mut(), GL_STREAM_DRAW);

        if !data.is_empty() {
            glBufferData(target, size, &data[0] as *const T as *const std::ffi::c_void, GL_STREAM_DRAW);
        }

        check("buffer data");
    }
}

struct ShaderProgram {
    id: GLuint,
    uniforms: Vec<ProgramUniform>,
    attributes: Vec<ProgramAttribute>,
}

struct ProgramUniform { loc: GLint }
struct ProgramAttribute { loc: GLuint }

struct GlBuffer { id: GLuint }

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &mut self.id);
            check("drop buf");
        }
    }
}

unsafe fn shader(shader_type: GLuint, source: &str) -> GLuint {
    let shader = glCreateShader(shader_type);

    glShaderSource(
        shader,
        1,
        &(CString::new(source.as_bytes()).expect("get CString")).as_ptr(),
        ptr::null(),
    );
    glCompileShader(shader);

    let mut success = GL_FALSE as GLint;

    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);

    if success != GL_TRUE as GLint {
        panic!(get_shader_info_log(shader));
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
    String::from_utf8_unchecked(mem::transmute(buf))
}

// TODO
pub unsafe fn set_curr_fb_size(width: i32, height: i32) {
    glViewport(0, 0, width, height);
}

use std::os::raw::{c_char, c_uchar, c_void, c_int, c_uint, c_float};

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
const GL_FALSE: GLboolean = 0;
const GL_TRUE: GLboolean = 1;
const GL_TRIANGLES: GLenum = 0x0004;
const GL_SRC_ALPHA: GLenum = 0x0302;
const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
const GL_FUNC_ADD: GLenum = 0x8006;
const GL_ARRAY_BUFFER: GLenum = 0x8892;
const GL_STREAM_DRAW: GLenum = 0x88E0;
const GL_TEXTURE_2D: GLenum = 0x0DE1;
const GL_BLEND: GLenum = 0x0BE2;
const GL_DEPTH_TEST: GLenum = 0x0B71;
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

dylib! {
    #[load_gl]
    extern "C" {
        // err
        fn glGetError() -> GLenum;
        fn glGetProgramiv(program: GLuint, pname: GLenum, params: *const GLint);
        fn glGetProgramInfoLog(program: GLuint, buf_size: GLsizei, len: *mut GLsizei, log: *mut GLchar);
        fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *const GLint);
        fn glGetShaderInfoLog(shader: GLuint, buf_size: GLsizei, len: *mut GLsizei, log: *mut GLchar);

        // vao
        fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
        fn glBindVertexArray(vao: GLuint);

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

        // tex
        fn glGenTextures(n: GLsizei, textures: *mut GLuint);
        fn glBindTexture(target: GLenum, tex: GLuint);
        fn glTexParameteri(target: GLenum, pname: GLenum, params: GLint);
        fn glTexImage2D(target: GLenum, level: GLint, format: GLint, width: GLsizei, height: GLsizei, border: GLint, internal_format: GLenum, kind: GLenum, pixels: *const c_void);
        fn glActiveTexture(tex: GLenum);

        // rendering
        fn glViewport(x: GLint, y: GLint, w: GLsizei, h: GLsizei);
        fn glEnable(cap: GLenum);
        fn glDisable(cap: GLenum);
        fn glClear(mask: GLbitfield);
        fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
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

unsafe fn load_platform_gl() {
    let file = {
        if cfg!(target_os = "windows") {
            "opengl32.dll"
        } else if cfg!(target_os = "macos") {
            "/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL"
        } else {
            "libGL.so.1"
        }
    };

    load_gl(c_str!(file));
}
