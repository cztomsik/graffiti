use crate::css::{Style, StyleProp, Value};
use crate::gfx::{Frame, Text, TextStyle};
use crate::layout::{LayoutEngine, LayoutNode};
use crate::matching_style;
use crate::renderer::Renderer;
use crate::util::SlotMap;
use crate::Rect;
use crate::StyleSheet;
use crate::{Document, DocumentEvent, NodeId, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Viewport {
    size: (i32, i32),

    document: Rc<RefCell<Document>>,
    layout_state: Rc<RefCell<LayoutState>>,
    resolved_styles: Rc<RefCell<SlotMap<NodeId, Style>>>,
    // texts: SlotMap<NodeId, Text>
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
        layout_nodes.put(document.borrow().root(), layout_engine.create_node());

        let layout_state = Rc::new(RefCell::new(LayoutState {
            engine: layout_engine,
            nodes: layout_nodes,
        }));

        let resolved_styles = Rc::new(RefCell::new(SlotMap::new()));

        let doc = Self {
            size,
            document: Rc::clone(document),
            layout_state: Rc::clone(&layout_state),
            resolved_styles: Rc::clone(&resolved_styles),
            renderer: Renderer::new(),
        };

        document.borrow_mut().add_listener(move |doc, e| {
            let LayoutState { engine, nodes } = &mut *layout_state.borrow_mut();
            let mut styles = resolved_styles.borrow_mut();

            use crate::layout::*;
            use DocumentEvent::*;

            match *e {
                Create(node, NodeType::Element) => {
                    nodes.put(node, engine.create_node());
                    styles.put(node, Style::new());
                }
                Create(node, NodeType::Text) => {
                    let text = Text::new(doc.cdata(node), &TextStyle::DEFAULT);
                    nodes.put(node, engine.create_leaf(move |max_width| text.measure(max_width)));
                }
                Create(node, NodeType::Comment) => {
                    nodes.put(node, engine.create_node());
                    update_layout_node(engine, nodes[node], &Style::HIDDEN);
                }

                Insert(parent, child, index) => engine.insert_child(nodes[parent], nodes[child], index),
                Remove(parent, child) => engine.remove_child(nodes[parent], nodes[child]),
                Drop(node) => {
                    engine.drop_node(nodes[node]);
                    nodes.remove(node);
                    styles.remove(node);
                }

                _ => {}
            }
        });

        doc
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
        self.update_styles();
        self.update_layout();
    }

    fn update_styles(&self) {
        let doc = self.document.borrow();
        let mut styles = self.resolved_styles.borrow_mut();
        let LayoutState { engine, nodes } = &mut *self.layout_state.borrow_mut();

        let sheets: Vec<_> = doc
            .query_selector_all(doc.root(), "html > head > style")
            .iter()
            .map(|s| StyleSheet::from(&*doc.text_content(*s)))
            .collect();
        doc.with_matching_context(|ctx| {
            for (el, style) in styles.iter_mut() {
                *style = matching_style(&ctx, &sheets, el);

                // TODO: add inline style

                // TODO: resolve inherit/initial/unset

                update_layout_node(engine, nodes[el], style);
            }
        });
    }

    fn update_layout(&self) {
        let size = (self.size.0 as _, self.size.1 as _);
        let LayoutState { engine, nodes } = &mut *self.layout_state.borrow_mut();

        engine.calculate(nodes[self.document.borrow().root()], size);
    }
}

fn update_layout_node(e: &mut LayoutEngine, n: LayoutNode, style: &Style) {
    fn dim(d: &super::css::Dimension) -> super::layout::Dimension {
        match d {
            super::css::Dimension::Px(v) => super::layout::Dimension::Px(*v),
            _ => super::layout::Dimension::Undefined,
        }
    }

    for p in style.props() {
        use super::layout::*;
        use StyleProp as P;
        use Value::Specified as S;

        match p {
            P::Display(S(v)) => e.set_display(
                n,
                match v {
                    _ => Display::None,
                },
            ),

            P::PaddingTop(S(v)) => e.set_padding_top(n, dim(v)),
            P::PaddingRight(S(v)) => e.set_padding_right(n, dim(v)),
            P::PaddingBottom(S(v)) => e.set_padding_bottom(n, dim(v)),
            P::PaddingLeft(S(v)) => e.set_padding_left(n, dim(v)),

            P::MarginTop(S(v)) => e.set_margin_top(n, dim(v)),
            P::MarginRight(S(v)) => e.set_margin_right(n, dim(v)),
            P::MarginBottom(S(v)) => e.set_margin_bottom(n, dim(v)),
            P::MarginLeft(S(v)) => e.set_margin_left(n, dim(v)),

            P::Top(S(v)) => e.set_top(n, dim(v)),
            P::Right(S(v)) => e.set_right(n, dim(v)),
            P::Bottom(S(v)) => e.set_bottom(n, dim(v)),
            P::Left(S(v)) => e.set_left(n, dim(v)),

            _ => {}
        }
    }
}
