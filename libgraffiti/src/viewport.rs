use crate::style::{StyleChange};
use crate::commons::{Pos, SurfaceId, Bounds};
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, BoxLayoutImpl};
use crate::text_layout::{TextLayout};
use crate::render::Renderer;
use crate::style::StyleUpdater;

// Holds the state & systems needed for one UI "viewport"
// basically, this is the window "content" area but
// nothing here is coupled to the window
// 
// - holds the scene & everything needed for rendering
// - translates events (target needs to be looked up)
// - accepts batch of updates to be applied to the scene
pub struct Viewport {
    box_layout: BoxLayoutImpl,
    text_layout: Box<TextLayout>,
    style_updater: StyleUpdater,
    renderer: Renderer,

    mouse_pos: Pos,
    picker: SurfacePicker,
}

// TODO: tagged enum, update interop & js event handling
#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub kind: EventKind,
    pub target: SurfaceId,
    pub key: u16,
}

impl Event {
    fn new(kind: EventKind, target: SurfaceId, key: u16) -> Self {
        Self { kind, target, key }
    }
}

#[derive(Debug, Clone, Copy)]
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
}

impl Viewport {
    // Box is important because of move vs. pointers
    pub fn new(width: i32, height: i32) -> Viewport {
        let mut text_layout = Box::new(TextLayout::new());

        Viewport {
            mouse_pos: Pos::zero(),

            // TODO: this is temporary until we find a way to pass measure safely
            box_layout: BoxLayoutImpl::new(width, height, &mut *text_layout),
            text_layout,
            style_updater: StyleUpdater::new(),
            picker: SurfacePicker::new(),

            renderer: Renderer::new(width, height),
        }
    }

    pub fn get_bounds(&self, surface: SurfaceId) -> Bounds {
        self.box_layout.get_bounds()[surface]
    }

    // translate events (break coupling)
    // app delegates to platform, which gets native events & calls
    // these methods to get events specific to this window/viewport
    // apart from the method signature, there's no need for other abstractions

    pub fn mouse_move(&mut self, pos: Pos) -> Event {
        self.mouse_pos = pos;

        Event::new(EventKind::MouseMove, self.get_mouse_target(), 0)
    }

    pub fn scroll(&mut self, _delta: (f32, f32)) -> Event {
        let _target = self.get_mouse_target();

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

    pub fn key_down(&mut self, scancode: u16) -> Event {
        Event::new(EventKind::KeyDown, 0, scancode)
    }

    pub fn key_press(&mut self, char: u16) -> Event {
        Event::new(EventKind::KeyPress, 0, char)
    }

    pub fn key_up(&mut self, scancode: u16) -> Event {
        Event::new(EventKind::KeyUp, 0, scancode)
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Event {
        self.renderer.resize(width, height);
        self.box_layout.resize(width, height);
        self.box_layout.calculate();

        self.render();

        Event::new(EventKind::Resize, 0, 0)
    }

    pub fn close(&mut self) -> Event {
        Event::new(EventKind::Close, 0, 0)
    }

    pub fn update_styles(&mut self, changes: &[StyleChange]) {
        let res = self.style_updater.update_styles(
            &mut self.box_layout,
            &mut self.text_layout,
            &mut self.renderer,
            changes
        );

        if res.needs_layout {
            self.box_layout.calculate();
        }

        self.render();
    }

    // apply batch of changes
    // some of this could be done in parallel which means the batch
    // itself or some part of it has to be passed to somebody who owns
    // all of the systems
    //
    // TODO: introduce some other struct responsible for this
    pub fn update_scene(&mut self, changes: &[SceneChange]) {
        use SceneChange::*;

        for c in changes {
            match c {
                InsertAt { parent, child, index } => {
                    self.box_layout.insert_at(*parent, *child, *index);
                    self.renderer.insert_at(*parent, *child, *index);
                }
                RemoveChild { parent, child } => {
                    self.box_layout.remove_child(*parent, *child);
                    self.renderer.remove_child(*parent, *child);
                }
                Alloc => {
                    self.box_layout.alloc();
                    self.renderer.alloc();
                }
            }
        }

        self.box_layout.calculate();
        self.render();
    }

    fn render(&mut self) {
        silly!("render");

        self.renderer.render(&self.box_layout.get_bounds(), &self.text_layout);
    }

    fn get_mouse_target(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.renderer.children, &self.box_layout.get_bounds())
    }
}

#[derive(Debug, Clone)]
pub enum SceneChange {
    // tree changes
    Alloc,
    InsertAt { parent: SurfaceId, child: SurfaceId, index: usize },
    RemoveChild { parent: SurfaceId, child: SurfaceId }
}
