use crate::css::{Style, StyleProp, Value};
use crate::gfx::Frame;
use crate::layout::{LayoutEngine, LayoutNode, LayoutStyle};
use crate::renderer::Renderer;
use crate::util::SlotMap;
use crate::Rect;
use crate::{Document, DocumentEvent, NodeId, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Viewport {
    size: (i32, i32),

    document: Rc<RefCell<Document>>,
    layout_state: Rc<RefCell<LayoutState>>,
    renderer: Renderer,
}

struct LayoutState {
    engine: LayoutEngine,
    nodes: SlotMap<NodeId, LayoutNode>,
}

impl Viewport {
    pub fn new(size: (i32, i32), document: &Rc<RefCell<Document>>) -> Self {
        let mut layout_engine = LayoutEngine::new();
        let mut layout_nodes = SlotMap::new();

        // create root layout node
        layout_nodes.put(
            document.borrow().root(),
            layout_engine.create_node(&LayoutStyle::DEFAULT),
        );

        let viewport = Self {
            size,
            document: Rc::clone(document),
            layout_state: Rc::new(RefCell::new(LayoutState {
                engine: layout_engine,
                nodes: layout_nodes,
            })),
            renderer: Renderer::new(),
        };

        viewport.document.borrow_mut().add_listener(viewport.layout_updater());

        viewport
    }

    pub fn document(&self) -> &Rc<RefCell<Document>> {
        &self.document
    }

    pub fn render(&mut self) -> Frame {
        self.update();

        let Self { document, renderer, .. } = self;
        let layout_state = &mut *self.layout_state.borrow_mut();

        renderer.render(&document.borrow(), &|n| {
            let n = layout_state.nodes[n];

            Rect {
                pos: layout_state.engine.node_offset(n),
                size: layout_state.engine.node_size(n),
            }
        })
    }

    // TODO: node_at_pos, node_offset, node_size, computed_style?

    // TODO: scrollTo(), scrollTop, ...

    pub fn resize(&mut self, size: (i32, i32)) {
        self.size = size;
        self.update();
    }

    fn update(&self) {
        self.update_layout()
    }

    fn update_layout(&self) {
        // TODO: skip if not needed

        let size = (self.size.0 as _, self.size.1 as _);
        let LayoutState { engine, nodes } = &mut *self.layout_state.borrow_mut();

        engine.calculate(nodes[self.document.borrow().root()], size);
    }

    fn layout_updater(&self) -> impl Fn(&Document, &DocumentEvent) {
        let layout_state = Rc::clone(&self.layout_state);

        move |_doc, e| {
            let LayoutState { engine, nodes } = &mut *layout_state.borrow_mut();

            use DocumentEvent::*;
            match *e {
                Create(node, NodeType::Element) => nodes.put(node, engine.create_node(&LayoutStyle::DEFAULT)),
                Create(node, NodeType::Text) => nodes.put(node, engine.create_leaf(|_| (100., 20.))),
                Create(node, NodeType::Comment) => nodes.put(node, engine.create_node(&LayoutStyle::HIDDEN)),

                Insert(parent, child, index) => engine.insert_child(nodes[parent], nodes[child], index),
                Remove(parent, child) => engine.remove_child(nodes[parent], nodes[child]),
                Drop(node) => engine.drop_node(nodes[node]),

                // StyleChange(el) => engine.set_style(nodes[n], &doc.style(n).into())
                _ => {}
            }
        }
    }
}
