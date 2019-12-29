use crate::commons::{Pos, SurfaceId, Color, BorderRadius, Border, BoxShadow, Image, Bounds};
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, BoxLayoutImpl, DimensionProp, Dimension, AlignProp, Align, FlexDirection, FlexWrap};
use crate::text_layout::{TextLayout, Text};
use crate::render::Renderer;

// Holds the state & systems needed for one UI "viewport"
// basically, this is the window "content" area but
// nothing here is coupled to the window
// 
// - holds the scene & everything needed for rendering
// - translates events (target needs to be looked up)
// - accepts batch of updates to be applied to the scene
pub struct Viewport {
    box_layout: Box<dyn BoxLayout>,
    text_layout: TextLayout,
    renderer: Renderer,

    mouse_pos: Pos,
    picker: SurfacePicker,
}

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
    pub fn new(width: i32, height: i32) -> Self {
        Viewport {
            mouse_pos: Pos::zero(),

            box_layout: Box::new(BoxLayoutImpl::new(width, height)),
            text_layout: TextLayout::new(),
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

        self.render();

        Event::new(EventKind::Resize, 0, 0)
    }

    pub fn close(&mut self) -> Event {
        Event::new(EventKind::Close, 0, 0)
    }

    // apply batch of changes
    // some of this could be done in parallel which means the batch
    // itself or some part of it has to be passed to somebody who owns
    // all of the systems
    //
    // other things (set_title) can be just plain old methods
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

                Text { surface, text } => {
                    // TODO: box_layout needs only true/false flag if measure is needed
                    // TODO: renderer needs only font_size, which could be provided from the text_layout,
                    //   along with the glyph iterator
                    self.box_layout.set_text(*surface, text.clone());
                    self.text_layout.set_text(*surface, text.clone());
                    self.renderer.set_text(*surface, text.clone());
                }

                BackgroundColor { surface, value } => self.renderer.set_background_color(*surface, *value),
                Border { surface, value } => self.renderer.set_border(*surface, *value),
                BoxShadow { surface, value } => self.renderer.set_box_shadow(*surface, *value),
                TextColor { surface, value } => self.renderer.set_text_color(*surface, *value),
                BorderRadius { surface, value } => self.renderer.set_border_radius(*surface, *value),
                Image { surface, value } => self.renderer.set_image(*surface, value.clone()),

                Dimension { surface, prop, value } => self.box_layout.set_dimension(*surface, *prop, *value),
                Align { surface, prop, value } => self.box_layout.set_align(*surface, *prop, *value),
                FlexWrap { surface, value } => self.box_layout.set_flex_wrap(*surface, *value),
                FlexDirection { surface, value } => self.box_layout.set_flex_direction(*surface, *value),
            }
        }

        self.render();
    }

    fn render(&mut self) {
        silly!("render");

        let text_layout = &mut self.text_layout;

        self.box_layout.calculate(&mut |surface, max_width| {
            text_layout.wrap(surface, max_width)
        });

        self.renderer.render(&self.box_layout.get_bounds(), &self.text_layout);
    }

    fn get_mouse_target(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.renderer.scene.children, &self.box_layout.get_bounds())
    }
}

#[derive(Debug, Clone)]
pub enum SceneChange {
    // tree changes
    Alloc,
    InsertAt { parent: SurfaceId, child: SurfaceId, index: usize },
    RemoveChild { parent: SurfaceId, child: SurfaceId },

    // layout changes
    Dimension { surface: SurfaceId, prop: DimensionProp, value: Dimension },
    Align { surface: SurfaceId, prop: AlignProp, value: Align },
    FlexWrap { surface: SurfaceId, value: FlexWrap },
    FlexDirection { surface: SurfaceId, value: FlexDirection },

    // visual changes
    BackgroundColor { surface: SurfaceId, value: Option<Color> },
    Border { surface: SurfaceId, value: Option<Border> },
    BoxShadow { surface: SurfaceId, value: Option<BoxShadow> },
    TextColor { surface: SurfaceId, value: Color },
    BorderRadius { surface: SurfaceId, value: Option<BorderRadius> },
    Image { surface: SurfaceId, value: Option<Image> },

    // text changes
    Text { surface: SurfaceId, text: Option<Text> },
}
