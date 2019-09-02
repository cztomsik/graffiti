use std::collections::BTreeMap;
use crate::SceneListener;
use crate::generated::{SurfaceId, UpdateSceneMsg, StyleProp, Color};
use new_hope::{NotSureWhat, RectId, TextId, DisplayItem, Pos, RGBA};

pub struct Renderer {
    children: Vec<Vec<SurfaceId>>,
    background_rects: BTreeMap<SurfaceId, RectId>,
    texts: BTreeMap<SurfaceId, TextId>,

    foo: NotSureWhat
}

impl SceneListener for Renderer {
    fn update_scene(&mut self, msgs: &[UpdateSceneMsg]) {
        let mut rebuild = false;

        for m in msgs {
            match m {
                UpdateSceneMsg::Alloc => {
                    self.children.push(Vec::new());
                }
                UpdateSceneMsg::InsertAt { parent, child, index } => {
                    self.children[*parent].insert(*index, *child);

                    rebuild = true;
                }
                UpdateSceneMsg::RemoveChild { parent, child } => {
                    self.children[*parent].retain(|c| c != child);

                    rebuild = true;
                }
                UpdateSceneMsg::SetStyleProp { surface, prop } => {
                    match prop {
                        StyleProp::BackgroundColor(c) => {
                            let rect = self.background_rects.get(surface);

                            match c {
                                Some(color) => {
                                    match rect {
                                        Some(rect_id) => self.foo.set_rect_color(*rect_id, color.into()),
                                        None => {
                                            self.background_rects.insert(*surface, self.foo.create_rect(Pos(0., 0.), Pos(1., 1.), color.into()));
                                        }
                                    }
                                }
                                None => {
                                    match rect {
                                        Some(rect_id) => self.foo.remove_rect(*rect_id),
                                        None => {}
                                    }
                                }
                            }
                        },
                        StyleProp::BorderRadius(..) | StyleProp::Border(..) | StyleProp::BoxShadow(..) | StyleProp::Image(..) | StyleProp::Text(..) => println!("TODO: handle {:?}", prop),

                        // non-visual props
                        _ => {}
                    }
                }
            }
        }

        if rebuild {
            let mut items = Vec::new();

            self.build_display_list(&mut items, 0);

            self.foo.set_display_list(&items);
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            children: vec![vec![]],
            background_rects: BTreeMap::new(),
            texts: BTreeMap::new(),

            foo: NotSureWhat::new()
        }
    }

    pub fn render(&mut self, _layout: &dyn Layout, _text_layout: &dyn TextLayout) {
      self.foo.render()
    }

    pub fn hit_test(&self, _pos: (f32, f32)) -> SurfaceId {
      // TODO
      0
    }

    pub fn scroll(&mut self, _pos: (f32, f32), _delta: (f32, f32)) {
      // TODO
    }

    fn build_display_list(&self, items: &mut Vec<DisplayItem>, surface: SurfaceId) {
        if let Some(rect_id) = self.background_rects.get(&surface) {
            items.push(DisplayItem::Rect(*rect_id));
        }

        if let Some(text_id) = self.texts.get(&surface) {
            items.push(DisplayItem::Text(*text_id));
        }

        for c in &self.children[surface] {
            self.build_display_list(items, *c);
        }
    }
}

impl Into<RGBA> for &Color {
    fn into(self) -> RGBA {
        let Color(r, g, b, a) = self;
        RGBA(*r, *g, *b, *a)
    }
}

use crate::layout::Layout;
use crate::text::TextLayout;
