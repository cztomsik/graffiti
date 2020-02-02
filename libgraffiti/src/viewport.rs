use crate::box_layout::{Display, Dimension, Align, FlexDirection, FlexWrap, Overflow};
use crate::commons::{ElementId, TextId, ElementChild, Pos, Bounds, Color};
use crate::picker::SurfacePicker;
use crate::box_layout::{BoxLayoutTree, BoxLayoutImpl};
use crate::text_layout::{TextLayout, Text};
use crate::render::{Renderer, RendererImpl, BoxShadow, Transform};

/// Holds the state & systems needed for one UI "viewport"
/// basically, this is the window's "content" area but
/// nothing here is actually coupled to the window
/// 
/// It also:
/// - translates events (target needs to be looked up)
/// - accepts batch of updates to be applied to the scene
pub struct Viewport {
    size: (f32, f32),

    children: Vec<Vec<ElementChild>>,

    box_layout: BoxLayoutImpl,
    text_layout: TextLayout,
    renderer: RendererImpl,

    // TODO: fullscreen + track the element id
    // (and use it in render/calculate)

    mouse_pos: Pos,
    picker: SurfacePicker,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    MouseMove { target: ElementId },
    MouseDown { target: ElementId },
    MouseUp { target: ElementId },
    Scroll { target: ElementId },
    KeyDown { target: ElementId, key: u16 },
    KeyPress { target: ElementId, key: u16 },
    KeyUp { target: ElementId, key: u16 },
    Resize { target: ElementId },
    Close { target: ElementId },
}

#[derive(Debug, Clone)]
pub enum SceneChange {
    Realloc { elements_count: ElementId, texts_count: TextId },

    InsertAt { parent: ElementId, child: ElementChild, index: usize },
    RemoveChild { parent: ElementId, child: ElementChild },
    SetText { id: TextId, text: Text },

    Display { element: ElementId, value: Display },
    Overflow { element: ElementId, value: Overflow },

    Width { element: ElementId, value: Dimension },
    Height { element: ElementId, value: Dimension },
    MinWidth { element: ElementId, value: Dimension },
    MinHeight { element: ElementId, value: Dimension },
    MaxWidth { element: ElementId, value: Dimension },
    MaxHeight { element: ElementId, value: Dimension },

    Top { element: ElementId, value: Dimension },
    Right { element: ElementId, value: Dimension },
    Bottom { element: ElementId, value: Dimension },
    Left { element: ElementId, value: Dimension },

    MarginTop { element: ElementId, value: Dimension },
    MarginRight { element: ElementId, value: Dimension },
    MarginBottom { element: ElementId, value: Dimension },
    MarginLeft { element: ElementId, value: Dimension },

    PaddingTop { element: ElementId, value: Dimension },
    PaddingRight { element: ElementId, value: Dimension },
    PaddingBottom { element: ElementId, value: Dimension },
    PaddingLeft { element: ElementId, value: Dimension },

    FlexGrow { element: ElementId, value: f32 },
    FlexShrink { element: ElementId, value: f32 },
    FlexBasis { element: ElementId, value: Dimension },
    FlexDirection { element: ElementId, value: FlexDirection },
    FlexWrap { element: ElementId, value: FlexWrap },

    AlignSelf { element: ElementId, value: Align },
    AlignContent { element: ElementId, value: Align },
    AlignItems { element: ElementId, value: Align },
    JustifyContent { element: ElementId, value: Align },

    Color { element: ElementId, value: Color },
    BackgroundColor { element: ElementId, value: Option<Color> },

    // TODO: border
    /*
    BorderTopWidth { element: ElementId, value: f32 },
    BorderRightWidth { element: ElementId, value: f32 },
    BorderBottomWidth { element: ElementId, value: f32 },
    BorderLeftWidth { element: ElementId, value: f32 },

    BorderTopStyle { element: ElementId, value: BorderStyle },
    BorderRightStyle { element: ElementId, value: BorderStyle },
    BorderBottomStyle { element: ElementId, value: BorderStyle },
    BorderLeftStyle { element: ElementId, value: BorderStyle },
    */

    // TODO: intermediate; clip in renderer
    BorderTopLeftRadius { element: ElementId, value: Option<f32> },
    BorderTopRightRadius { element: ElementId, value: Option<f32> },
    BorderBottomLeftRadius { element: ElementId, value: Option<f32> },
    BorderBottomRightRadius { element: ElementId, value: Option<f32> },

    // BackgroundImageUrl { element: ElementId, value: String },

    // TODO: many
    BoxShadow { element: ElementId, value: Option<BoxShadow> },

    // TODO: many
    Transform { element: ElementId, value: Option<Transform> },
}

impl Viewport {
    const ROOT: ElementId = 0;

    pub fn new(size: (f32, f32)) -> Viewport {
        let mut viewport = Viewport {
            size,

            children: Vec::new(),

            box_layout: BoxLayoutImpl::new(),
            text_layout: TextLayout::new(),

            mouse_pos: Pos::ZERO,
            picker: SurfacePicker::new(),

            renderer: RendererImpl::new(size),
        };

        // create root
        viewport.update_scene(&[SceneChange::Realloc { elements_count: 1, texts_count: 0 }]);

        // set min-height
        viewport.resize(size);

        viewport
    }

    pub fn update_scene(&mut self, changes: &[SceneChange]) {
        let mut needs_layout = false;

        use SceneChange::*;

        for c in changes {
            // order by likelihood (perf)

            match c {
                // start with layout-independent things
                Transform { element, value } => self.renderer.set_transform(*element, *value),
                Color { element, value } => self.renderer.set_color(*element, *value),
                BackgroundColor { element, value } => self.renderer.set_background_color(*element, (*value).unwrap_or(crate::commons::Color::TRANSPARENT)),
                //BoxShadow { element, value } => self.renderer.set_box_shadow(*element, *value),

                // TODO: intermediate (top-left, top-right, ...) & set Option<BorderRadius>
                // BorderRadius { element, value } => self.renderer.set_border_radius(*element, *value),

                // TODO: Image

                // TODO: Border
                // might need relayout!

                // layout will be needed
                c => {
                    needs_layout = true;

                    match c {
                        Display { element, value } => self.box_layout.set_display(*element, *value),

                        Width { element, value } => self.box_layout.set_width(*element, *value),
                        Height { element, value } => self.box_layout.set_height(*element, *value),
                        MinWidth { element, value } => self.box_layout.set_min_width(*element, *value),
                        MinHeight { element, value } => self.box_layout.set_min_height(*element, *value),
                        MaxWidth { element, value } => self.box_layout.set_max_width(*element, *value),
                        MaxHeight { element, value } => self.box_layout.set_max_height(*element, *value),

                        Top { element, value } => self.box_layout.set_top(*element, *value),
                        Right { element, value } => self.box_layout.set_right(*element, *value),
                        Bottom { element, value } => self.box_layout.set_bottom(*element, *value),
                        Left { element, value } => self.box_layout.set_left(*element, *value),

                        MarginTop { element, value } => self.box_layout.set_margin_top(*element, *value),
                        MarginRight { element, value } => self.box_layout.set_margin_right(*element, *value),
                        MarginBottom { element, value } => self.box_layout.set_margin_bottom(*element, *value),
                        MarginLeft { element, value } => self.box_layout.set_margin_left(*element, *value),

                        PaddingTop { element, value } => self.box_layout.set_padding_top(*element, *value),
                        PaddingRight { element, value } => self.box_layout.set_padding_right(*element, *value),
                        PaddingBottom { element, value } => self.box_layout.set_padding_bottom(*element, *value),
                        PaddingLeft { element, value } => self.box_layout.set_padding_left(*element, *value),

                        FlexGrow { element, value } => self.box_layout.set_flex_grow(*element, *value),
                        FlexShrink { element, value } => self.box_layout.set_flex_shrink(*element, *value),
                        FlexBasis { element, value } => self.box_layout.set_flex_basis(*element, *value),
                        FlexDirection { element, value } => self.box_layout.set_flex_direction(*element, *value),
                        FlexWrap { element, value } => self.box_layout.set_flex_wrap(*element, *value),

                        AlignSelf { element, value } => self.box_layout.set_align_self(*element, *value),
                        AlignContent { element, value } => self.box_layout.set_align_content(*element, *value),
                        AlignItems { element, value } => self.box_layout.set_align_items(*element, *value),
                        JustifyContent { element, value } => self.box_layout.set_justify_content(*element, *value),

                        InsertAt { parent, child, index } => {
                            self.children[*parent].insert(*index, *child);
                            self.box_layout.insert_at(*parent, *child, *index);
                        }

                        RemoveChild { parent, child } => {
                            self.children[*parent].retain(|ch| *ch != *child);
                            self.box_layout.remove_child(*parent, *child);
                        }

                        Realloc { elements_count, texts_count } => {
                            self.realloc(*elements_count, *texts_count);
                        }

                        SetText { id, text } => {
                            self.box_layout.mark_text_dirty(*id);
                            self.text_layout.set_text(*id, text);
                        },

                        _ => { error!("TODO: set {:?}", &c); }
                    }
                }
            }
        }

        if needs_layout {
            self.calculate();
        }

        self.render();
    }

    // making systems "resizable" simplifies a lot of things
    // notably there's no need to return anything, ids are shared
    // across the whole viewport and freelists are also needed just here
    fn realloc(&mut self, elements_count: ElementId, texts_count: TextId) {
        assert!(elements_count > 0);

        self.children.resize_with(elements_count, || Vec::new());

        self.text_layout.realloc(texts_count);
        self.box_layout.realloc(elements_count, texts_count);
        self.renderer.realloc(elements_count, texts_count);
    }

    pub fn get_offset_bounds(&self, element: ElementId) -> Bounds {
        self.box_layout.get_element_bounds(element)
    }

    // translate events (break coupling)
    // app delegates to platform, which gets native events & calls
    // these methods to get events specific to this window/viewport
    // apart from the method signature, there's no need for other abstractions

    pub fn mouse_move(&mut self, pos: Pos) -> Event {
        self.mouse_pos = pos;

        Event::MouseMove { target: self.get_mouse_target() }
    }

    /*
    pub fn scroll(&mut self, _delta: (f32, f32)) -> Event {
        let _target = self.get_mouse_target();

        // TODO: just like ScrollBy/ScrollAt update message (& render() after that)
        //self.renderer.scroll(self.mouse_pos, delta);

        Event::Scroll { target: self.get_mouse_target() }
    }
    */

    pub fn mouse_down(&mut self) -> Event {
        Event::MouseDown { target: self.get_mouse_target() }
    }

    pub fn mouse_up(&mut self) -> Event {
        Event::MouseUp { target: self.get_mouse_target() }
    }

    pub fn key_down(&mut self, scancode: u16) -> Event {
        Event::KeyDown { target: 0, key: scancode }
    }

    pub fn key_press(&mut self, char: u16) -> Event {
        Event::KeyPress { target: 0, key: char }
    }

    pub fn key_up(&mut self, scancode: u16) -> Event {
        Event::KeyUp { target: 0, key: scancode }
    }

    pub fn resize(&mut self, size: (f32, f32)) -> Event {
        self.size = size;

        self.renderer.resize(size);

        // let make screen overflow (for now)
        // TODO: overflow: scroll could fix it
        // @see calculate() in `yoga.rs`
        self.update_scene(&[SceneChange::MinHeight { element: Self::ROOT, value: Dimension::Px { value: size.1 } }]);
        //self.calculate();
        //self.render();

        Event::Resize { target: Self::ROOT }
    }

    pub fn close(&mut self) -> Event {
        Event::Close { target: Self::ROOT }
    }

    fn calculate(&mut self) {
        let text_layout = &mut self.text_layout;
        let mut wraps = Vec::new();

        self.box_layout.calculate(Self::ROOT, self.size, &mut |text_id, max_width| {
            // TODO: it seems it's called even when not needed in bounds example
            //println!("wrap called");
            wraps.push(text_id);
            text_layout.wrap(text_id, max_width)
        });

        for id in wraps {
            self.renderer.set_text_glyphs(id, 16., self.text_layout.get_glyphs(id))
        }
    }

    fn render(&mut self) {
        silly!("render");

        self.renderer.render(
            Self::ROOT,
            &self.children,
            &self.box_layout
        );
    }

    fn get_mouse_target(&self) -> ElementId {
        self.picker.pick_at(self.mouse_pos, &self.children, &self.box_layout)
    }
}
