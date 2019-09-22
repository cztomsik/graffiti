use crate::generated::{SurfaceId, UpdateSceneMsg, WindowEvent};
use crate::commons::Pos;
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, YogaLayout};
use crate::text_layout::TextLayout;
use crate::render::Renderer;

pub struct Window {
    box_layout: Box<dyn BoxLayout>,
    text_layout: TextLayout,
    renderer: Renderer,

    mouse_pos: Pos,
    picker: SurfacePicker,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Window {
            mouse_pos: Pos::default(),

            box_layout: Box::new(YogaLayout::new((width as f32, height as f32))),
            text_layout: TextLayout::new(),
            picker: SurfacePicker::new(),

            renderer: Renderer::new(),
        }
    }

    pub fn mouse_move(&mut self, pos: Pos) -> WindowEvent {
        self.mouse_pos = pos;

        WindowEvent::MouseMove {
            target: self.get_mouse_target(),
        }
    }

    pub fn scroll(&mut self, delta: (f32, f32)) -> WindowEvent {
        let target = self.get_mouse_target();

        // TODO: just like ScrollBy/ScrollAt update message (& render() after that)
        //self.renderer.scroll(self.mouse_pos, delta);

        WindowEvent::Scroll { target }
    }

    pub fn mouse_down(&mut self) -> WindowEvent {
        WindowEvent::MouseDown {
            target: self.get_mouse_target(),
        }
    }

    pub fn mouse_up(&mut self) -> WindowEvent {
        WindowEvent::MouseUp {
            target: self.get_mouse_target(),
        }
    }

    pub fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        self.text_layout.update_scene(msgs);
        self.box_layout.update_scene(msgs);
        self.renderer.update_scene(msgs);

        let text_layout = &mut self.text_layout;

        self.box_layout.calculate(&mut |surface, max_width| {
            text_layout.wrap(surface, max_width);

            text_layout.get_size(surface)
        });

        self.renderer.render(&self.box_layout.get_bounds(), &self.text_layout);
    }

    fn get_mouse_target(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.renderer.scene.children, &self.box_layout.get_bounds())
    }
}
