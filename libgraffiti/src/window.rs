use crate::commons::{Pos, SurfaceId, Color, BorderRadius, Border, BoxShadow, Image};
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayout, YogaLayout, DimensionProp, Dimension, AlignProp, Align, FlexDirection, FlexWrap};
use crate::text_layout::{TextLayout, Text};
use crate::render::Renderer;
use miniserde::{Deserialize, Serialize};

// - delegates to platform for window-related things (TODO)
// - holds the scene & everything needed for rendering
// - translates events (target needs to be looked up)
// - accepts batch of updates to be applied to the scene
pub struct Window {
    // TODO: platform window id

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
    fn new(kind: EventKind, target: SurfaceId, key: u16) -> Self {
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
}

impl Window {
    pub fn new(width: i32, height: i32) -> Self {
        Window {
            mouse_pos: Pos::zero(),

            box_layout: Box::new(YogaLayout::new(width, height)),
            text_layout: TextLayout::new(),
            picker: SurfacePicker::new(),

            renderer: Renderer::new(width, height),
        }
    }

    // TODO: set_title, set_size, ...
    // (should just delegate to platform with the id/handle)

    // translate events

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
    // itself or some part of it  has to be passed to somebody who owns
    // all of the systems
    //
    // other things (set_title) can be just plain old methods
    //
    // TODO: introduce some other struct responsible for this
    pub fn update_scene(&mut self, msg: &UpdateSceneMsg) {
        for c in &msg.tree_changes {
            match c {
                TreeChange { parent: Some(parent), child: Some(child), index: Some(index) } => {
                    self.box_layout.insert_at(*parent, *child, *index);
                    self.renderer.insert_at(*parent, *child, *index);
                }
                TreeChange { parent: Some(parent), child: Some(child), .. } => {
                    self.box_layout.remove_child(*parent, *child);
                    self.renderer.remove_child(*parent, *child);
                }
                _ => {
                    self.box_layout.alloc();
                    self.renderer.alloc();
                }
            }
        }

        for TextChange { surface, color, text } in &msg.text_changes {
            if let Some(color) = color {
                self.renderer.set_text_color(*surface, *color);
            } else {
                self.box_layout.set_text(*surface, text.clone());
                self.text_layout.set_text(*surface, text.clone());
                self.renderer.set_text(*surface, text.clone());
            }
        }

        for c in &msg.layout_changes {
            match c {
                LayoutChange { surface, dim_prop: Some(p), dim: Some(v), .. } => self.box_layout.set_dimension(*surface, *p, *v),
                LayoutChange { surface, align_prop: Some(p), align: Some(v), .. } => self.box_layout.set_align(*surface, *p, *v),
                LayoutChange { surface, flex_direction: Some(v), .. } => self.box_layout.set_flex_direction(*surface, *v),
                LayoutChange { surface, flex_wrap: Some(v), .. } => self.box_layout.set_flex_wrap(*surface, *v),
                _ => unreachable!("invalid layout change")
            }
        }

        for c in &msg.background_color_changes {
            self.renderer.set_background_color(c.surface, c.color);
        }

        self.render();
    }

    fn render(&mut self) {
        silly!("render");

        let text_layout = &mut self.text_layout;

        self.box_layout.calculate(&mut |surface, max_width| {
            debug!("wrap {:?} {:?}", surface, max_width);
            dbg!(text_layout.wrap(surface, max_width))
        });

        self.renderer.render(&self.box_layout.get_bounds(), &self.text_layout);
    }

    fn get_mouse_target(&self) -> SurfaceId {
        self.picker.pick_at(self.mouse_pos, &self.renderer.scene.children, &self.box_layout.get_bounds())
    }
}

// TODO: in future, replace miniserde with some custom protocol with
//     something which is fast to build but still easy to prepare
//     (maybe some stateful text-based protocol with out-of-order inserts)
//
// can't be rust enum because of miniserde
// optimized for common changes (text, colors, tree)
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSceneMsg {
    text_changes: Vec<TextChange>,
    background_color_changes: Vec<BackgroundColorChange>,
    layout_changes: Vec<LayoutChange>,
    tree_changes: Vec<TreeChange>,
}

// alloc, insert, remove
#[derive(Deserialize, Serialize, Debug)]
pub struct TreeChange {
    parent: Option<SurfaceId>,
    child: Option<SurfaceId>,
    index: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TextChange {
    surface: SurfaceId,
    // either color or text
    color: Option<Color>,
    text: Option<Text>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LayoutChange {
    surface: SurfaceId,

    dim_prop: Option<DimensionProp>,
    dim: Option<Dimension>,

    align_prop: Option<AlignProp>,
    align: Option<Align>,

    flex_wrap: Option<FlexWrap>,
    flex_direction: Option<FlexDirection>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BackgroundColorChange {
    surface: SurfaceId,
    color: Option<Color>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetRadius {
    surface: SurfaceId,
    layout: Option<BorderRadius>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBorder {
    surface: SurfaceId,
    border: Option<Border>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetBoxShadow {
    surface: SurfaceId,
    shadow: Option<BoxShadow>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetImage {
    surface: SurfaceId,
    image: Option<Image>,
}

/*
#[derive(Deserialize, Serialize, Debug)]
pub struct SetOverflow {
    surface: SurfaceId,
    overflow: Overflow,
}
*/
