use std::mem;
use std::ptr;
use std::ffi::CString;
use std::convert::TryInto;
use gl::types::*;

use crate::commons::{Pos, Bounds, Color, TextId};
use super::{RenderBackend, RenderOp, RectId};
use crate::text_layout::GlyphInstance;

// TODO: transpile shaders (mac vs raspi vs. GLES)
//       (maybe GLSL macros?)
//       (but we are not using any extensions currently)

pub struct GlRenderBackend {
    rects: Vec<Rect>,
    indices: Vec<VertexIndex>,

    gpu: Gpu,
    rect_program: ShaderProgram,
    text_program: ShaderProgram,

    ibo: GlBuffer,
    vbo: GlBuffer,
    text_vbos: Vec<GlBuffer>,
}

// We can't use instanced drawing for raspi
// so we need real quad for each rectangular area
// TODO: try other mem layouts
#[derive(Debug, Clone, Copy)]
struct Rect([Vertex<Color>; 4]);

impl Rect {
    const VERTEX_SIZE: GLint = mem::size_of::<Vertex<Color>>()  as GLint;

    fn new (bounds: Bounds, color: Color) -> Self {
        let mut res = Self([Vertex(Pos::ZERO, Color::TRANSPARENT); 4]);

        res.set_bounds(bounds);
        res.set_color(color);

        res
    }

    fn set_bounds(&mut self, Bounds { a, b }: Bounds) {
        (self.0)[0].0 = a;
        (self.0)[1].0 = Pos::new(b.x, a.y);
        (self.0)[2].0 = Pos::new(a.x, b.y);
        (self.0)[3].0 = b;
    }

    fn set_color(&mut self, color: Color) {
        (self.0)[0].1 = color;
        (self.0)[1].1 = color;
        (self.0)[2].1 = color;
        (self.0)[3].1 = color;
    }
}

// we use glDrawArrays for glyphs so that we don't need ibo
// (but that could change in future)
#[derive(Debug, Clone, Copy)]
struct Glyph([Vertex<Pos>; 6]);

impl Glyph {
    const VERTEX_SIZE: GLint = mem::size_of::<Vertex<Pos>>()  as GLint;
}

#[derive(Debug, Clone, Copy)]
struct Vertex<T>(Pos, T);

// for indexed drawing
// raspi can do only 65k vertices in one batch
type VertexIndex = u16;

impl GlRenderBackend {
    pub(crate) fn new() -> Self {
        unsafe {
            let mut gpu = Gpu::new();
            let ibo = gpu.create_buffer();
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
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, 512, 512, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(SDF_TEXTURE));
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
                &["u_projection"],
                &["a_pos", "a_color"]
            );

            let text_program = gpu.create_program(
                include_str!("shaders/text.vert"),
                include_str!("shaders/text.frag"),
                &["u_projection", "u_pos", "u_color", "u_dist_factor"],
                &["a_pos", "a_uv"]
            );

            Self {
                rects: Vec::new(),
                indices: Vec::new(),

                gpu,
                rect_program,
                text_program,

                ibo,
                vbo,
                text_vbos: Vec::new(),
            }
        }
    }
}

impl RenderBackend for GlRenderBackend {
    fn realloc(&mut self, rects_count: RectId, texts_count: TextId) {
        let Self { gpu, rects, text_vbos, .. } = self;

        rects.resize(rects_count, Rect::new(Bounds::ZERO, Color::TRANSPARENT));

        // for now, each text has its own buffer
        text_vbos.resize_with(texts_count, || unsafe { gpu.create_buffer() })
    }

    fn set_rect_bounds(&mut self, rect: RectId, bounds: Bounds) {
        self.rects[rect].set_bounds(bounds);
    }

    fn set_rect_color(&mut self, rect: RectId, color: Color) {
        self.rects[rect].set_color(color);
    }

    fn set_text_glyphs<I>(&mut self, text: TextId, glyphs: I) where I: Iterator<Item=GlyphInstance> {
        let glyphs: Vec<Glyph> = glyphs.map(|GlyphInstance { bounds, coords }| {
            Glyph([
                Vertex(bounds.a, coords.a),
                Vertex(Pos::new(bounds.b.x, bounds.a.y), Pos::new(coords.b.x, coords.a.y)),
                Vertex(Pos::new(bounds.a.x, bounds.b.y), Pos::new(coords.a.x, coords.b.y)),
                Vertex(Pos::new(bounds.b.x, bounds.a.y), Pos::new(coords.b.x, coords.a.y)),
                Vertex(Pos::new(bounds.a.x, bounds.b.y), Pos::new(coords.a.x, coords.b.y)),
                Vertex(bounds.b, coords.b),
            ])
        }).collect();

        unsafe { self.gpu.buffer_data(&mut self.text_vbos[text], gl::ARRAY_BUFFER, &glyphs) }
    }

    fn render(&mut self, rects: &[RectId], ops: &[RenderOp]) {
        // needed because of &self.indices[0]
        if rects.is_empty() {
            return
        }

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // clear & generate indices
            self.indices.set_len(0);
            for i in rects {
                let base = (i * 4) as VertexIndex;

                // 6 indices for 2 triangles
                self.indices.push(base);
                self.indices.push(base + 3);
                self.indices.push(base + 2);
                // second one
                self.indices.push(base);
                self.indices.push(base + 1);
                self.indices.push(base + 3);
            }

            // upload indices
            // (can stay bound for the whole time)
            self.gpu.buffer_data(&mut self.ibo, gl::ELEMENT_ARRAY_BUFFER, &self.indices);
            check("ibo");

            // upload vertices
            // (have to be bound again during rendering)
            //
            // TODO: skip if up-to-date
            self.gpu.buffer_data(&mut self.vbo, gl::ARRAY_BUFFER, &self.rects);
            check("vbo");

            // index buffer is shared for everything rect-based (except text)
            let mut ibo_offset = 0;

            for op in ops {
                match op {
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
                        gl::DrawElements(gl::TRIANGLES, (count * 6) as GLint, gl::UNSIGNED_SHORT, ibo_offset as *const std::ffi::c_void);
                        check("draw els");

                        ibo_offset += count * 6 * mem::size_of::<VertexIndex>();
                    }

                    RenderOp::DrawText { id, pos, color, distance_factor } => {
                        let vbo = &self.text_vbos[*id];

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
                        gl::DrawArrays(gl::TRIANGLES, 0, (vbo.len() * 6) as GLint);
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

struct Mat3([f32;9]);

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

    unsafe fn create_program(&mut self, vs: &str, fs: &str, uniforms: &[&str], attributes: &[&str]) -> ShaderProgram {
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

        let uniforms = uniforms.iter().map(|name| {
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

        GlBuffer { id, len: 0 }
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
            gl::BufferData(target, size, mem::transmute(&data[0]), gl::STREAM_DRAW);
        }

        check("buffer data");

        buffer.len = data.len();
    }
}

struct ShaderProgram {
    id: GLuint,
    uniforms: Vec<ProgramUniform>,
    attributes: Vec<ProgramAttribute>,
}

struct ProgramUniform { loc: GLint }
struct ProgramAttribute { loc: GLuint }

struct GlBuffer { id: GLuint, len: usize }

impl GlBuffer {
    fn len(&self) -> usize {
        self.len
    }
}

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
