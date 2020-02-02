// straight-forward drawArrays-based primitive renderer
// there was some perf-issue with indexed drawing
// and this is much simpler too

use std::mem;
use std::ptr;
use std::ffi::CString;
use std::convert::TryInto;
use gl::types::*;

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

impl GlRenderBackend {
    pub(crate) fn new() -> Self {
        unsafe {
            let mut gpu = Gpu::new();
            let vbo = gpu.create_buffer();

            // TODO
            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            // TODO: mipmap could improve small sizes but I'm not sure if it wouldn't need additional texture space
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            // because of RGB
            //gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, 64, 64, 0, gl::RGB, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, 512, 512, 0, gl::RGBA, gl::UNSIGNED_BYTE, &SDF_TEXTURE[0] as *const u8 as *const std::ffi::c_void);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            check("texture");

            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD);

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

        unsafe { self.gpu.buffer_data(vbo, gl::ARRAY_BUFFER, &glyphs) }

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
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // upload vbo
            // (have to be bound again during rendering)
            //
            // TODO: skip if up-to-date
            // (but keep that empty check because of &rects[0] ref)
            if !self.rects.is_empty() {
                self.gpu.buffer_data(&mut self.vbo, gl::ARRAY_BUFFER, &self.rects);
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
                        gl::UseProgram(self.rect_program.id);
                        gl::UniformMatrix3fv(self.rect_program.uniforms[1].loc, 1, gl::FALSE, &transform.0 as *const f32);
                        check("set transform");
                    }

                    RenderOp::PopTransform => {
                        transform_stack.pop().unwrap();

                        gl::UseProgram(self.rect_program.id);
                        gl::UniformMatrix3fv(self.rect_program.uniforms[1].loc, 1, gl::FALSE, &transform_stack.last().unwrap().0 as *const f32);
                        check("restore transform");
                    }

                    RenderOp::DrawRects { count } => {
                        gl::UseProgram(self.rect_program.id);
                        self.gpu.bind_buffer(&self.vbo, gl::ARRAY_BUFFER);
                        gl::VertexAttribPointer(
                            self.rect_program.attributes[0].loc,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            Rect::VERTEX_SIZE,
                            ptr::null(),
                        );
                        gl::VertexAttribPointer(
                            self.rect_program.attributes[1].loc,
                            4,
                            gl::UNSIGNED_BYTE,
                            gl::FALSE,
                            Rect::VERTEX_SIZE,
                            mem::size_of::<Pos>() as *const std::ffi::c_void,
                        );

                        // draw
                        gl::DrawArrays(gl::TRIANGLES, vbo_offset, count * 6);
                        check("draw els");

                        vbo_offset += count * 6;
                    }

                    RenderOp::DrawText { id, pos, color } => {
                        let (vbo, glyphs_count, distance_factor) = &self.texts[*id];

                        gl::UseProgram(self.text_program.id);
                        self.gpu.bind_buffer(vbo, gl::ARRAY_BUFFER);
                        gl::VertexAttribPointer(
                            self.text_program.attributes[0].loc,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            Glyph::VERTEX_SIZE,
                            ptr::null(),
                        );
                        gl::VertexAttribPointer(
                            self.text_program.attributes[1].loc,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            Glyph::VERTEX_SIZE,
                            mem::size_of::<Pos>() as *const std::ffi::c_void,
                        );

                        gl::Uniform2f(self.text_program.uniforms[1].loc, pos.x, pos.y);

                        // unpack it here, maybe even in builder
                        let color: [f32; 4] = [color.r as f32 / 256., color.g as f32 / 256., color.b as f32 / 256., color.a as f32 / 256.];
                        gl::Uniform4fv(self.text_program.uniforms[2].loc, 1, &color as *const GLfloat);

                        gl::Uniform1f(self.text_program.uniforms[3].loc, *distance_factor);

                        // draw
                        gl::DrawArrays(gl::TRIANGLES, 0, *glyphs_count * 6);
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
            gl::UseProgram(self.rect_program.id);
            gl::UniformMatrix3fv(self.rect_program.uniforms[0].loc, 1, gl::FALSE, &mat.0 as *const f32);

            gl::UseProgram(self.text_program.id);
            gl::UniformMatrix3fv(self.text_program.uniforms[0].loc, 1, gl::FALSE, &mat.0 as *const f32);
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
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        check("vao");

        Self
    }

    unsafe fn create_program(&mut self, vs: &str, fs: &str, extra_uniforms: &[&str], attributes: &[&str]) -> ShaderProgram {
        let vs = shader(gl::VERTEX_SHADER, vs);
        check("vertex shader");

        let fs = shader(gl::FRAGMENT_SHADER, fs);
        check("fragment shader");

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            panic!(get_program_info_log(program));
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);

        gl::UseProgram(program);

        let uniforms = ["u_projection"].iter().chain(extra_uniforms.iter()).map(|name| {
            let loc = gl::GetUniformLocation(program, c_str!(name.clone()));
            check("uniform location");

            ProgramUniform { loc }
        }).collect();

        let attributes = attributes.iter().map(|name| {
            let loc = gl::GetAttribLocation(program, c_str!(name.clone())).try_into().expect("invalid attr loc");
            check("uniform location");

            gl::EnableVertexAttribArray(loc);
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

        gl::GenBuffers(1, &mut id);
        check("gen buffer");

        GlBuffer { id }
    }

    #[inline(always)]
    unsafe fn bind_buffer(&mut self, buffer: &GlBuffer, target: GLuint) {
        gl::BindBuffer(target, buffer.id);
        check("bind buffer");
    }

    #[inline(always)]
    unsafe fn buffer_data<T>(&mut self, buffer: &mut GlBuffer, target: GLuint, data: &[T]) {
        let size = (mem::size_of::<T>() * data.len()) as GLsizeiptr;

        self.bind_buffer(buffer, target);

        // orphaning
        // https://www.seas.upenn.edu/~pcozzi/OpenGLInsights/OpenGLInsights-AsynchronousBufferTransfers.pdf
        gl::BufferData(target, size, std::ptr::null_mut(), gl::STREAM_DRAW);

        if !data.is_empty() {
            gl::BufferData(target, size, &data[0] as *const T as *const std::ffi::c_void, gl::STREAM_DRAW);
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
            gl::DeleteBuffers(1, &self.id);
            check("drop buf");
        }
    }
}

unsafe fn shader(shader_type: GLuint, source: &str) -> GLuint {
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
