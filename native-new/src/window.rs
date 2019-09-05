use crate::generated::{SurfaceId, UpdateSceneMsg, WindowEvent};
use crate::commons::Pos;
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, StretchLayout};
use crate::text_layout::TextLayout;
use crate::renderer::Renderer;

pub struct Window {
    mouse_pos: Pos,

    box_layout: Box<dyn BoxLayout>,
    text_layout: TextLayout,
    picker: SurfacePicker,

    renderer: Renderer,
}

impl Window {
    pub fn new(width: i32, height: i32) -> Self {
        Window {
            mouse_pos: (0., 0.),

            box_layout: Box::new(StretchLayout::new((width as f32, height as f32))),
            text_layout: TextLayout::new(),
            picker: SurfacePicker::new(),

            renderer: Renderer::new(),
        }
    }

    pub fn mouse_move(&mut self, pos: (f32, f32)) -> WindowEvent {
        self.mouse_pos = pos;

        WindowEvent::MouseMove {
            target: self.hovered_surface(),
        }
    }

    pub fn scroll(&mut self, delta: (f32, f32)) -> WindowEvent {
        let target = self.hovered_surface();

        self.renderer.scroll(self.mouse_pos, delta);

        WindowEvent::Scroll { target }
    }

    pub fn mouse_down(&mut self) -> WindowEvent {
        WindowEvent::MouseDown {
            target: self.hovered_surface(),
        }
    }

    pub fn mouse_up(&mut self) -> WindowEvent {
        WindowEvent::MouseUp {
            target: self.hovered_surface(),
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

    fn hovered_surface(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.layout.get_bounds())
    }
}
