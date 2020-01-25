use std::mem;
use std::ptr;
use std::ffi::CString;
use gl::types::*;

use crate::commons::{Pos, Bounds, Color, TextId};
use super::{RenderBackend, RenderOp, RectId};

// TODO: transpile shaders (mac vs raspi vs. GLES)
//       (maybe GLSL macros?)
//       (but we are not using any extensions currently)

pub struct GlRenderBackend {
    rects: Vec<Rect>,
    indices: Vec<VertexIndex>,

    rect_program: GLuint,
    text_program: GLuint,
    resize_uniforms: [(GLuint, GLint); 2],
    text_color_uniform: GLint,
    text_factor_uniform: GLint,

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
    const VERTEX_SIZE: usize = mem::size_of::<Vertex<Color>>();

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

#[derive(Debug, Clone, Copy)]
struct Glyph([Vertex<Pos>; 4]);


#[derive(Debug, Clone, Copy)]
struct Vertex<T>(Pos, T);

// for indexed drawing
// raspi can do only 65k vertices in one batch
type VertexIndex = u16;

impl GlRenderBackend {
    pub(crate) fn new() -> Self {
        unsafe {
            // not used but webgl & opengl core profile require it
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            check("vao");

            let ibo = GlBuffer::new();
            let vbo = GlBuffer::new();

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

            let rect_program = shader_program(RECT_VS, RECT_FS);
            check("rect_program");
            let text_program = shader_program(TEXT_VS, TEXT_FS);
            check("text_program");

            // this is important otherwise indices sometimes do not reflect
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
                rects: Vec::new(),
                indices: Vec::new(),

                rect_program,
                text_program,

                resize_uniforms,
                text_color_uniform,
                text_factor_uniform,

                ibo,
                vbo,
                text_vbos: Vec::new(),
            }
        }
    }
}

impl RenderBackend for GlRenderBackend {
    fn realloc(&mut self, rects_count: RectId, texts_count: TextId) {
        self.rects.resize(rects_count, Rect::new(Bounds::ZERO, Color::TRANSPARENT));

        // for now, each text has its own buffer
        self.text_vbos.resize_with(texts_count, || unsafe { GlBuffer::new() })
    }

    fn set_rect_bounds(&mut self, rect: RectId, bounds: Bounds) {
        self.rects[rect].set_bounds(bounds);
    }

    fn set_rect_color(&mut self, rect: RectId, color: Color) {
        self.rects[rect].set_color(color);
    }

    fn render(&mut self, rects: &[RectId], ops: &[RenderOp]) {
        unsafe {
            // TODO: opaque rect in bg (last item) might be faster
            // clear needs to fill all pixels, bg rect fills only what's left
            // maybe just `documentElement.style.backgroundColor = '#fff'` and don't clear at all
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            // TODO: | DEPTH_BUFFER_BIT
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // needed because of &self.indices[0]
            if rects.is_empty() {
                return
            }

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
            self.ibo.data(gl::ELEMENT_ARRAY_BUFFER, &self.indices);
            check("ibo");

            // upload vertices
            // (have to be bound again during rendering)
            //
            // TODO: skip if up-to-date
            self.vbo.data(gl::ARRAY_BUFFER, &self.rects);
            check("vbo");

            // index buffer is shared for everything rect-based (except text)
            let mut ibo_offset = 0;

            for op in ops {
                let quad_count;

                match op {
                    RenderOp::DrawRects { count } => {
                        quad_count = count;

                        gl::UseProgram(self.rect_program);
                        gl::EnableVertexAttribArray(0);
                        gl::VertexAttribPointer(
                            0,
                            2,
                            gl::FLOAT,
                            gl::FALSE,
                            Rect::VERTEX_SIZE as GLint,
                            ptr::null(),
                        );
                        gl::EnableVertexAttribArray(1);
                        gl::VertexAttribPointer(
                            1,
                            4,
                            gl::UNSIGNED_BYTE,
                            gl::FALSE,
                            Rect::VERTEX_SIZE as GLint,
                            mem::size_of::<Pos>() as *const std::ffi::c_void,
                        );
                    }
                }

                // draw
                gl::DrawElements(gl::TRIANGLES, (quad_count * 6) as GLint, gl::UNSIGNED_SHORT, ibo_offset as *const std::ffi::c_void);
                check("draw els");

                ibo_offset += quad_count * 6 * mem::size_of::<VertexIndex>();
            }


            /*
            match b {
                RenderOp::DrawText { color, distance_factor, count } => {
                    vertex_size = mem::size_of::<Vertex<Pos>>();
                    quad_count = count;

                    gl::UseProgram(self.text_program);
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(
                        0,
                        2,
                        gl::FLOAT,
                        gl::FALSE,
                        vertex_size as GLint,
                        ptr::null(),
                    );
                    gl::EnableVertexAttribArray(1);
                    gl::VertexAttribPointer(
                        1,
                        2,
                        gl::FLOAT,
                        gl::FALSE,
                        vertex_size as GLint,
                        mem::size_of::<Pos>() as *const std::ffi::c_void,
                    );

                    // unpack it here, maybe even in builder
                    let color: [f32; 4] = [color.r as f32 / 256., color.g as f32 / 256., color.b as f32 / 256., color.a as f32 / 256.];
                    gl::Uniform4fv(self.text_color_uniform, 1, &color as *const GLfloat);
                    gl::Uniform1f(self.text_factor_uniform, distance_factor);
                }
            }
            */
        }
    }

    fn resize(&mut self, (width, height): (f32, f32)) {
        unsafe {
            for (program, uniform) in &self.resize_uniforms {
                gl::UseProgram(*program);
                gl::Uniform2f(*uniform, width, height);
            }
            check("resize");
        }
    }
}

const RECT_VS: &str = include_str!("shaders/rect.vert");
const RECT_FS: &str = include_str!("shaders/rect.frag");
const TEXT_VS: &str = include_str!("shaders/text.vert");
const TEXT_FS: &str = include_str!("shaders/text.frag");

const SDF_TEXTURE: &[u8; 512 * 512 * 4] = include_bytes!("../../resources/sheet0.raw");

struct GlBuffer { id: GLuint }

impl GlBuffer {
    unsafe fn new() -> Self {
        let mut id = 0;

        gl::GenBuffers(1, &mut id);
        check("gen buffer");

        Self { id }
    }

    unsafe fn data<T>(&mut self, target: GLuint, data: &[T]) {
        let size = (mem::size_of::<T>() * data.len()) as GLsizeiptr;

        gl::BindBuffer(target, self.id);

        // orphaning to avoid implicit sync
        //
        // TODO: not sure if this is how it should be done and it's also possible it has no effect on
        //   raspi and other iGPUs (maybe glMapBufferRange would be better)
        //   
        //   maybe pooling & round-robin buffers with SubData might have bigger effect but that would require bigger
        //   changes and it's not clear if that would actually help
        //
        //   https://www.seas.upenn.edu/~pcozzi/OpenGLInsights/OpenGLInsights-AsynchronousBufferTransfers.pdf
        gl::BufferData(target, size, std::ptr::null_mut(), gl::STREAM_DRAW);
        gl::BufferData(target, size, mem::transmute(&data[0]), gl::STREAM_DRAW);

        check("buffer data");
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

unsafe fn shader_program(vertex_shader_source: &str, fragment_shader_source: &str) -> GLuint {
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
