use crate::generated::{SurfaceId, WindowEvent};
use crate::render::Renderer;

pub struct Window {
    mouse_pos: (f32, f32),

    renderer: Box<dyn Renderer>,
    // layout
}

impl Window {
    pub fn new(renderer: Box<dyn Renderer>) -> Self {
        Window {
            mouse_pos: (0., 0.),
            renderer,
        }
    }

    pub fn mouse_move(&mut self, pos: (f32, f32)) -> WindowEvent {
        self.mouse_pos = pos;

        WindowEvent::MouseMove {
            target: self.hit_test(),
        }
    }

    pub fn scroll(&mut self, delta: (f32, f32)) -> WindowEvent {
        let target = self.hit_test();

        self.renderer.scroll(self.mouse_pos, delta);

        WindowEvent::Scroll { target }
    }

    pub fn mouse_down(&mut self) -> WindowEvent {
        WindowEvent::MouseDown {
            target: self.hit_test(),
        }
    }

    pub fn mouse_up(&mut self) -> WindowEvent {
        WindowEvent::MouseUp {
            target: self.hit_test(),
        }
    }

    fn hit_test(&self) -> SurfaceId {
        self.renderer.hit_test(self.mouse_pos)
    }
}

/*

fn update_sizes(&mut self) {
    let w_size = self.glfw_window.get_size();
    let fb_size = self.glfw_window.get_framebuffer_size();
    let dpi = (w_size.0 as f32) / (fb_size.0 as f32);

    self.renderer.resize(fb_size, dpi);
    self.scene.set_layout_size(((w_size.0 as f32) * dpi, (w_size.1 as f32) * dpi));

    self.render();
}

fn render(&mut self) {
    self.scene.calculate_layout();

    self.glfw_window.make_current();
    self.renderer.render(&self.scene);
    self.glfw_window.swap_buffers();
}

*/
