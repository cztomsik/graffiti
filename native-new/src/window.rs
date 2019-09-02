use crate::generated::{SurfaceId, UpdateSceneMsg, WindowEvent};
use crate::render::Renderer;
use crate::layout::Layout;
use crate::text::TextLayout;
use crate::SceneListener;

pub struct Window {
    mouse_pos: (f32, f32),

    renderer: Renderer,
    layout: Box<dyn Layout>,
    text_layout: Box<dyn TextLayout>
}

impl Window {
    pub fn new(renderer: Renderer, layout: Box<dyn Layout>, text_layout: Box<dyn TextLayout>) -> Self {
        Window {
            mouse_pos: (0., 0.),
            renderer,
            layout,
            text_layout
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
        self.text_layout.update_scene(msgs);
        self.layout.update_scene(msgs);
        self.renderer.update_scene(msgs);

        let text_layout = &mut self.text_layout;

        self.layout.calculate(&mut |surface, max_width| {
            text_layout.wrap(surface, max_width);

            text_layout.get_size(surface)
        });
        self.renderer.render(&*self.layout, &*self.text_layout);
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
