// super-simple GPU-first 2D graphics
// x outputs (textured) vertices + draw "ops"
// x easy to integrate, inspired by imgui backend

pub type RGBA8 = [u8; 4];

pub struct Canvas {
    states: Vec<State>,
    frame: Frame,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            states: vec![State::DEFAULT],
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
            self.states[0] = State::DEFAULT
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

    pub fn fill_text(&mut self, text: &str, x: f32, y: f32) {
        todo!()
    }

    // TODO: stroke_text()
}

#[derive(Debug, Clone)]
struct State {
    fill_color: RGBA8,
    opacity: f32,
}

impl State {
    const DEFAULT: Self = Self {
        fill_color: [0, 0, 0, 255],
        opacity: 1.,
    };
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
