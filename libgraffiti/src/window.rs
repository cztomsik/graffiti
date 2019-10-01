use crate::commons::{Pos, SurfaceId};
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, StretchLayout};
use crate::text_layout::TextLayout;
use crate::render::Renderer;
use miniserde::{Deserialize, Serialize};

pub struct Window {
    box_layout: Box<dyn BoxLayout>,
    text_layout: TextLayout,
    renderer: Renderer,

    mouse_pos: Pos,
    picker: SurfacePicker,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Event {
    kind: EventKind,
    target: SurfaceId,
    key: u16,
}

impl Event {
    // TODO: private
    pub fn new(kind: EventKind, target: SurfaceId, key: u16) -> Self {
        Self { kind, target, key }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum EventKind {
    MouseMove,
    MouseDown,
    MouseUp,
    Scroll,
    KeyDown,
    KeyPress,
    KeyUp,
    Focus,
    Blur,
    Resize,
    Close,
    Unknown,    
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Window {
            mouse_pos: Pos::zero(),

            box_layout: Box::new(StretchLayout::new((width as f32, height as f32))),
            text_layout: TextLayout::new(),
            picker: SurfacePicker::new(),

            renderer: Renderer::new(),
        }
    }

    pub fn mouse_move(&mut self, pos: Pos) -> Event {
        self.mouse_pos = pos;

        Event::new(EventKind::MouseMove, self.get_mouse_target(), 0)
    }

    pub fn scroll(&mut self, delta: (f32, f32)) -> Event {
        let target = self.get_mouse_target();

        // TODO: just like ScrollBy/ScrollAt update message (& render() after that)
        //self.renderer.scroll(self.mouse_pos, delta);

        Event::new(EventKind::Scroll, self.get_mouse_target(), 0)
    }

    pub fn mouse_down(&mut self) -> Event {
        Event::new(EventKind::MouseDown, self.get_mouse_target(), 0)
    }

    pub fn mouse_up(&mut self) -> Event {
        Event::new(EventKind::MouseUp, self.get_mouse_target(), 0)
    }

    /*
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
    */

    fn get_mouse_target(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.renderer.scene.children, &self.box_layout.get_bounds())
    }
}
