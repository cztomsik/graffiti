use crate::generated::{SurfaceId, UpdateSceneMsg, WindowEvent};
use crate::render::Renderer;
use std::rc::Rc;
use crate::layout::Layout;
use std::cell::RefCell;

pub struct Window {
    mouse_pos: (f32, f32),

    renderer: Box<dyn Renderer>,
    layout: Rc<RefCell<dyn Layout>>
}

impl Window {
    pub fn new(renderer: Box<dyn Renderer>, layout: Rc<RefCell<dyn Layout>>) -> Self {
        Window {
            mouse_pos: (0., 0.),
            renderer,
            layout,
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

    pub fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        self.layout.borrow_mut().update_scene(msgs);
        self.renderer.update_scene(msgs);
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
