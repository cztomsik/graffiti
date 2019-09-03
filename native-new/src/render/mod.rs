use std::collections::BTreeMap;
use crate::SceneListener;
use crate::generated::{SurfaceId, UpdateSceneMsg, StyleProp, BoxShadow, Color, Image, Text, Border, Rect, BorderRadius};
use crate::helpers::Storage;
use crate::layout::Layout;
use crate::text::{TextLayout, Glyph};

pub struct Renderer {
    rect_program: u32,

    scene: Scene
}

struct Scene {
    border_radii: BTreeMap<SurfaceId, BorderRadius>,
    box_shadows: BTreeMap<SurfaceId, BoxShadow>,
    background_colors: BTreeMap<SurfaceId, Color>,
    images: BTreeMap<SurfaceId, Image>,
    texts: BTreeMap<SurfaceId, Text>,
    borders: BTreeMap<SurfaceId, Border>,
    children: Vec<Vec<SurfaceId>>
}

impl SceneListener for Renderer {
    fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        for m in msgs.iter().cloned() {
            match m {
                UpdateSceneMsg::Alloc => self.scene.children.push(Vec::new()),
                UpdateSceneMsg::InsertAt { parent, child, index } => self.scene.children[parent].insert(index, child),
                UpdateSceneMsg::RemoveChild { parent, child } => self.scene.children[parent].retain(|ch| *ch != child),
                UpdateSceneMsg::SetStyleProp { surface, prop } => match prop {
                    StyleProp::BorderRadius(r) => self.scene.border_radii.set(surface, r),
                    StyleProp::BoxShadow(s) => self.scene.box_shadows.set(surface, s),
                    StyleProp::BackgroundColor(c) => self.scene.background_colors.set(surface, c),
                    StyleProp::Image(i) => self.scene.images.set(surface, i),
                    StyleProp::Text(t) => self.scene.texts.set(surface, t),
                    StyleProp::Border(b) => self.scene.borders.set(surface, b),
                    _ => {}
                }
            }
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        unsafe {
            // not used but webgl & opengl core profile require it
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            check();

            Self {
                rect_program: shader_program(RECT_VS, RECT_FS),

                scene: Scene {
                    border_radii: BTreeMap::new(),
                    box_shadows: BTreeMap::new(),
                    background_colors: BTreeMap::new(),
                    images: BTreeMap::new(),
                    texts: BTreeMap::new(),
                    borders: BTreeMap::new(),
                    children: vec![vec![]]
                }
            }            
        }
    }

    pub fn render(&mut self, layout: &dyn Layout, text_layout: &dyn TextLayout) {
        debug!("render");

        let mut frame = Frame::new();

        let surface = 0;

        let mut context = RenderContext {
            layout,
            text_layout,

            scene: &self.scene,
            bounds: layout.get_rect(surface).into(),

            frame: &mut frame
        };

        context.draw_surface(surface);

        self.render_frame(&mut frame);
    }

    pub fn hit_test(&self, _pos: (f32, f32)) -> SurfaceId {
      // TODO
      0
    }

    pub fn scroll(&mut self, _pos: (f32, f32), _delta: (f32, f32)) {
      // TODO
    }

    // fn create_text() -> TextId {}

    fn render_frame(&mut self, frame: &mut Frame) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            frame.upload();

            // setup for opaque (depth, buffers)
            //gl::Disable(gl::BLEND);
            //gl::Enable(gl::DEPTH_TEST);
            frame.opaque_quads.bind_to(gl::ARRAY_BUFFER);
            frame.opaque_indices.bind_to(gl::ELEMENT_ARRAY_BUFFER);
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
            gl::DrawElements(gl::TRIANGLES, frame.opaque_indices.data.len() as i32, gl::UNSIGNED_SHORT, std::ptr::null());
            check();

            /*
            // setup for alpha (depth, alpha, buffers)
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD);
            self.mixed_quads.bind_to(gl::ARRAY_BUFFER);
            self.alpha_indices.bind_to(gl::ELEMENT_ARRAY_BUFFER);

            // TODO: setup attrib pointers for each interleaving batch & draw

            // gl::DrawElements(gl::TRIANGLES, vertices_count as i32, gl::UNSIGNED_SHORT, (offset * std::mem::size_of::<VertexIndex>()) as *const std::ffi::c_void);
            */
        }
    }
}

struct RenderContext<'a> {
    layout: &'a dyn Layout,
    text_layout: &'a dyn TextLayout,
    scene: &'a Scene,

    // TODO: clip
    bounds: (Pos, Pos),

    frame: &'a mut Frame
}

impl <'a> RenderContext<'a> {
    // TODO: bitflags
    fn draw_surface(&mut self, id: SurfaceId) {
        let parent_bounds = self.bounds;
        let mut rect = self.layout.get_rect(id);

        rect.0 += (parent_bounds.0).0;
        rect.1 += (parent_bounds.0).1;
        self.bounds = rect.into();

        if let Some(box_shadow) = self.scene.box_shadows.get(&id) {
            self.draw_box_shadow(box_shadow);
        }

        if let Some(color) = self.scene.background_colors.get(&id) {
            self.draw_background_color(color);
        }

        if let Some(image) = self.scene.images.get(&id) {
            self.draw_image(image);
        }

        if let Some(text) = self.scene.texts.get(&id) {
            self.draw_text(text, self.text_layout.get_glyphs(id));
        }

        // TODO: scroll
        if let Some((width, height)) = self.layout.get_scroll_frame(id) {
            debug!("scroll_frame {:?}", (&width, &height, &self.bounds));
        }

        // TODO: try to avoid recursion?
        for c in &self.scene.children[id] {
            self.draw_surface(*c);
        }

        if let Some(border) = self.scene.borders.get(&id) {
            self.draw_border(border);
        }

        // restore because of recursion
        self.bounds = parent_bounds;
    }

    fn draw_box_shadow(&mut self, shadow: &BoxShadow) {
        // TODO: spread
        // TODO: blur
        self.frame.push_rect(self.bounds, &shadow.color);
    }

    fn draw_background_color(&mut self, color: &Color) {
        self.frame.push_rect(self.bounds, color);
    }

    fn draw_image(&mut self, _image: &Image) {
        // TODO
        self.frame.push_rect(self.bounds, &Color(100, 200, 255, 255));
    }

    // TODO: create_text() -> TextId & Batch::Text(text_id)
    fn draw_text(&mut self, text: &Text, glyphs: &[Glyph]) {
        // TODO: should be uniform
        let origin = self.bounds.0;

        debug!("text {:?} {:?}", &origin, &text.text);

        let Pos(start_x, start_y) = origin;

        for Glyph { x, y, glyph_id: _ } in glyphs {
            self.frame.push_rect((Pos(start_x + x, start_y + y), Pos(start_x + x + 8., start_y + y + 10.)), &text.color);
        }
    }

    fn draw_border(&mut self, _border: &Border) {
        // TODO
        //self.frame.push_rect(self.bounds, &Color(0, 0, 0, 255));
    }
}

impl Copy for Rect {}
impl Copy for Color {}

impl Into<(Pos, Pos)> for Rect {
    fn into(self) -> (Pos, Pos) {
        let Rect(x, y, w, h) = self;

        (Pos(x, y), Pos(x + w, y + h))
    }
}




// low-level stuff, merged (and improved) from PoC in cztomsiK/new-hope
use std::mem;
use std::ptr;
use std::ffi::CString;
use gl::types::*;

/// Application unit (or something similar, unit of measure)
/// TODO(later): Integer type could save some CPU & memory
type Au = f32;

/// 2D Point
#[derive(Clone, Copy, Debug)]
pub struct Pos(pub Au, pub Au);

/// Everything what's rendered, is quad-based, it's easier to imagine then
#[derive(Debug)]
struct Quad<T>([Vertex<T>; 4]);

impl <T: Copy> Quad<T> {
    fn new(a: Pos, b: Pos, data: T) -> Self {
        Self([
            Vertex(a, data),
            Vertex(Pos(b.0, a.1), data),
            Vertex(Pos(a.0, b.1), data),
            Vertex(b, data),
        ])
    }
}

/// Vertex including some primitive-specific attributes
#[derive(Debug)]
struct Vertex<T>(Pos, T);

// for indexed drawing
// raspi can do only 65k vertices in one batch
// could be configurable but it's probably better to play it safe
type VertexIndex = u16;


struct Frame {
    // separate opaque pass
    // - less overdraw
    // - less batches (there's less left to interleave then)
    opaque_quads: Buffer<Quad<Color>>,
    opaque_indices: Buffer<VertexIndex>,

    // the rest has to be drawn in alpha and so it has to be interleaved in multiple batches
    // but at least we can put everything into one vertex buffer & index buffer 
    mixed_quads: Buffer<u8>,
    alpha_indices: Buffer<VertexIndex>,

    batches: Vec<()>,

    z: f32
}

impl Frame {
    fn new() -> Self {
        Self {
            opaque_quads: Buffer::new(),
            opaque_indices: Buffer::new(),

            mixed_quads: Buffer::new(),
            alpha_indices: Buffer::new(),

            batches: Vec::new(),

            z: 0.
        }
    }

    fn push_rect(&mut self, (a, b): (Pos, Pos), color: &Color) {
        // TODO: opaque/alpha branch

        self.opaque_quads.push(Quad::new(a, b, *color));

        let n = self.opaque_quads.data.len();
        let base = 4 * (n as VertexIndex);

        self.opaque_indices.push(base + 1);
        self.opaque_indices.push(base);
        self.opaque_indices.push(base + 3);

        self.opaque_indices.push(base);
        self.opaque_indices.push(base + 2);
        self.opaque_indices.push(base + 3);

        // TODO: alpha colors should be drawn in alpha batches
        // all indices would be relative to the current batch
        // each batch has to start at new offset (important for vertex attrib pointer)
    }

    unsafe fn upload(&mut self) {
        silly!("upload {:?}", &self.opaque_quads.data);
        silly!("upload {:?}", &self.opaque_indices.data);

        self.opaque_quads.upload_to(gl::ARRAY_BUFFER);
        self.opaque_indices.upload_to(gl::ELEMENT_ARRAY_BUFFER);

        self.mixed_quads.upload_to(gl::ARRAY_BUFFER);
        self.alpha_indices.upload_to(gl::ELEMENT_ARRAY_BUFFER);

        check();
    }
}

struct Buffer<T> {
    id: u32,
    data: Vec<T>
}

impl <T> Buffer<T> {
    fn new() -> Self {
        let mut id = 0;

        unsafe { gl::GenBuffers(1, &mut id) }

        Self {
            id,
            data: Vec::new()
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    unsafe fn bind_to(&mut self, target: u32) {
        gl::BindBuffer(target, self.id);
    }

    unsafe fn upload_to(&mut self, target: u32) {
        if self.data.is_empty() {
            return;
        }

        self.bind_to(target);

        gl::BufferData(
            target,
            (self.data.len() * mem::size_of::<T>()) as isize,
            mem::transmute(&self.data[0]),
            gl::STATIC_DRAW
        );        
    }
}

impl <T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
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

// from gleam
fn get_shader_info_log(shader: GLuint) -> String {
    let mut max_len = [0];
    unsafe {
        get_shader_iv(shader, gl::INFO_LOG_LENGTH, &mut max_len);
    }
    if max_len[0] == 0 {
        return String::new();
    }
    let mut result = vec![0u8; max_len[0] as usize];
    let mut result_len = 0 as GLsizei;
    unsafe {
        gl::GetShaderInfoLog(
            shader,
            max_len[0] as GLsizei,
            &mut result_len,
            result.as_mut_ptr() as *mut GLchar,
        );
    }
    result.truncate(if result_len > 0 {
        result_len as usize
    } else {
        0
    });
    String::from_utf8(result).unwrap()
}
unsafe fn get_shader_iv(shader: GLuint, pname: GLenum, result: &mut [GLint]) {
    assert!(!result.is_empty());
    gl::GetShaderiv(shader, pname, result.as_mut_ptr());
}
fn get_program_info_log(program: GLuint) -> String {
    let mut max_len = [0];
    unsafe {
        get_program_iv(program, gl::INFO_LOG_LENGTH, &mut max_len);
    }
    if max_len[0] == 0 {
        return String::new();
    }
    let mut result = vec![0u8; max_len[0] as usize];
    let mut result_len = 0 as GLsizei;
    unsafe {
        gl::GetProgramInfoLog(
            program,
            max_len[0] as GLsizei,
            &mut result_len,
            result.as_mut_ptr() as *mut GLchar,
        );
    }
    result.truncate(if result_len > 0 {
        result_len as usize
    } else {
        0
    });
    String::from_utf8(result).unwrap()
}
unsafe fn get_program_iv(program: GLuint, pname: GLenum, result: &mut [GLint]) {
    assert!(!result.is_empty());
    gl::GetProgramiv(program, pname, result.as_mut_ptr());
}
