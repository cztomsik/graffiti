#![allow(unused)]

use fontdue::Font;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::css::{CssEngine, Style};
use crate::document::DocumentEvent;
use crate::layout::{LayoutEngine, LayoutNode};
use crate::render::{backend::RenderBackend, Renderer};
use crate::util::{SlotMap, TreeAdapter};
use crate::{Document, NodeId, Rect, ResolvedStyle};

pub struct Viewport {
    size: (f32, f32),

    // model
    document: Document,
    changes: Receiver<DocumentEvent>,

    // intermediate
    resolved_styles: SlotMap<NodeId, ResolvedStyle>,
    layout_nodes: SlotMap<NodeId, LayoutNode>,
    fonts: Vec<Font>,

    css_engine: CssEngine,
    layout_engine: LayoutEngine,
    renderer: Renderer,
}

impl Viewport {
    pub fn new(size: (f32, f32), render_backend: impl RenderBackend + 'static + Send) -> Self {
        const ROBOTO: &'static [u8] = include_bytes!("../resources/Roboto/Roboto-Regular.ttf");

        let (changes_tx, changes) = channel();

        Self {
            size,

            document: Document::new(move |c| changes_tx.send(c).unwrap()),
            changes,

            resolved_styles: SlotMap::new(),
            layout_nodes: SlotMap::new(),
            fonts: vec![Font::from_bytes(ROBOTO, Default::default()).unwrap()],

            css_engine: CssEngine::new(),
            layout_engine: LayoutEngine::new(),
            renderer: Renderer::new(render_backend),
        }
    }

    pub fn size(&self) -> (f32, f32) {
        self.size
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    pub fn update(&mut self) {
        for e in self.changes.try_iter() {
            match e {
                DocumentEvent::ElementCreated(el) => {
                    // TODO: update on change
                    self.resolved_styles.put(el, ResolvedStyle::INITIAL);

                    self.layout_nodes.put(el, self.layout_engine.create_node());
                }

                DocumentEvent::TextNodeCreated(tn) => {
                    self.layout_nodes.put(tn, self.layout_engine.create_node());
                }

                DocumentEvent::NodeInserted(parent, child, index) => {
                    self.layout_engine
                        .insert_child(self.layout_nodes[parent], self.layout_nodes[child], index);
                }

                DocumentEvent::NodeRemoved(parent, child) => {
                    self.layout_engine
                        .remove_child(self.layout_nodes[parent], self.layout_nodes[child]);
                }

                DocumentEvent::NodeDestroyed(node) => {
                    self.layout_engine.free_node(self.layout_nodes[node]);
                    self.layout_nodes.remove(node);
                }

                _ => println!("TODO: {:?}", e),
            }
        }

        self.layout_engine
            .calculate(self.layout_nodes[self.document.root()], self.size);
    }

    pub fn render(&mut self) {
        let Self {
            renderer,
            document,
            resolved_styles,
            layout_engine,
            layout_nodes,
            ..
        } = self;

        renderer.render(
            document,
            |node| &resolved_styles[node],
            |node| Rect {
                pos: layout_engine.node_offset(layout_nodes[node]),
                size: layout_engine.node_size(layout_nodes[node]),
            },
        );
    }

    pub fn resolved_style(&self, node: NodeId) -> &ResolvedStyle {
        &self.resolved_styles[node]
    }

    pub fn node_at_pos(&self, pos: (f32, f32)) -> NodeId {
        println!("TODO: viewport.node_at_pos({:?})", &pos);
        self.document.root()
        // self.picker.pick_at(self.mouse_pos, &self.document, &self.layout_engine)
    }

    pub fn resize(&mut self, size: (f32, f32)) {
        println!("TODO: viewport.resize({:?})", &size);
    }
}
