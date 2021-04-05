// super-simple GPU-first 2D graphics
// x outputs (textured) vertices + draw "ops"
// x easy to integrate, inspired by imgui backend

use super::font::SANS_SERIF_FACE;
use super::Font;
use owned_ttf_parser::AsFaceRef;
use std::sync::Arc;

pub type RGBA8 = [u8; 4];

pub struct Canvas {
    states: Vec<State>,
    frame: Frame,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            states: vec![State::default()],
            frame: Frame::new(),
        }
    }

    // TODO: channel?
    pub fn flush(&mut self) -> Frame {
        std::mem::replace(&mut self.frame, Frame::new())
    }

    pub fn save(&mut self) {
        self.states.push(self.state().clone());
    }

    pub fn restore(&mut self) {
        if self.states.len() > 1 {
            drop(self.states.pop())
        } else {
            self.states[0] = State::default()
        }
    }

    fn state(&self) -> &State {
        self.states.last().unwrap()
    }

    fn state_mut(&mut self) -> &mut State {
        self.states.last_mut().unwrap()
    }

    pub fn opacity(&self) -> f32 {
        self.state().opacity
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.state_mut().opacity = opacity;
    }

    pub fn fill_color(&self) -> RGBA8 {
        self.state().fill_color
    }

    pub fn set_fill_color(&mut self, fill_color: RGBA8) {
        self.state_mut().fill_color = fill_color;
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let color = self.state().fill_color;

        // TODO
        let z = 1.;
        let uv = [0., 0.];

        self.frame.vertices.extend_from_slice(&[
            Vertex {
                xyz: [x, y, z],
                uv,
                color,
            },
            Vertex {
                xyz: [x + width, y, z],
                uv,
                color,
            },
            Vertex {
                xyz: [x, y + height, z],
                uv,
                color,
            },
            Vertex {
                xyz: [x + width, y, z],
                uv,
                color,
            },
            Vertex {
                xyz: [x, y + height, z],
                uv,
                color,
            },
            Vertex {
                xyz: [x + width, y + height, z],
                uv,
                color,
            },
        ]);

        // TODO: join
        self.frame.draw_calls.push(DrawOp::DrawArrays(6));
    }

    pub fn set_font(&mut self, font: &str) {
        todo!()
    }

    pub fn measure_text(&self, text: &str, max_width: Option<f32>) -> TextMetrics {
        todo!()
    }

    pub fn fill_text(&mut self, text: &str, mut x: f32, y: f32) {
        let scale = SANS_SERIF_FACE.scale;
        let face = SANS_SERIF_FACE.face.as_face_ref();

        for c in text.chars() {
            if let Some(glyph_id) = face.glyph_index(c) {
                if let Some(glyph_rect) = face.glyph_bounding_box(glyph_id) {
                    self.fill_rect(
                        x,
                        y,
                        glyph_rect.width() as f32 * scale,
                        glyph_rect.height() as f32 * scale,
                    );
                    x += face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32 * scale;
                }
            }
        }
    }

    // TODO: stroke_text()
}

#[derive(Debug, Clone)]
struct State {
    fill_color: RGBA8,
    font: Arc<Font>,
    opacity: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fill_color: [0, 0, 0, 255],
            font: Arc::clone(&SANS_SERIF_FACE),
            opacity: 1.,
        }
    }
}

pub struct TextMetrics {
    width: f32,
}

#[derive(Debug)]
pub struct Frame {
    pub vertices: Vec<Vertex>,
    pub draw_calls: Vec<DrawOp>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(1024),
            draw_calls: Vec::with_capacity(32),
        }
    }
}

#[derive(Debug)]
pub enum DrawOp {
    // TODO: scissor_rect, texture, ?
    DrawArrays(usize),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub xyz: [f32; 3],
    pub uv: [f32; 2],
    pub color: RGBA8,
}
