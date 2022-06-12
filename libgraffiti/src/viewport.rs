use crate::{
    convert::{container_style, layout_style},
    css::{MatchingContext, Style},
    document::NodeEdge,
    layout::{self, LayoutEngine, LayoutResult, LayoutStyle, LayoutTree, Size},
    renderer::{ContainerStyle, Rect, RenderEdge, RenderTree, Renderer},
    Document, NodeId, NodeType,
};
use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    FontMgr, Paint,
};
use std::{cell::RefCell, collections::HashMap};

pub struct Viewport {
    size: (i32, i32),
    document: Document,
    renderer: Renderer,
    state: ViewState,
}

// needs to be updated before using
#[derive(Debug, Default)]
struct ViewState {
    resolved_styles: HashMap<NodeId, Style>,
    paragraphs: HashMap<NodeId, RefCell<Paragraph>>,
    layout_styles: HashMap<NodeId, LayoutStyle>,
    layout_results: Vec<LayoutResult>,
}

impl Viewport {
    pub fn new(size: (i32, i32), document: Document, renderer: Renderer) -> Self {
        Self {
            size,
            document,
            renderer,
            state: ViewState::default(),
        }
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn resize(&mut self, size: (i32, i32)) {
        self.size = size;
        self.renderer.resize(size);
    }

    pub fn element_at(&mut self, _pos: (f32, f32)) -> Option<NodeId> {
        self.update();
        todo!()
    }

    pub fn node_rect(&mut self, _node: NodeId) -> Option<()> {
        self.update();
        todo!()
    }

    pub fn render(&mut self) {
        self.update();
        self.renderer.render(&(&self.document, &self.state));
    }

    // TODO: move/click/drag/selection/...
    // pub fn scroll(&mut self, _pos: (f32, f32), _delta: (f32, f32)) {
    //     todo!()
    // }

    fn update(&mut self) {
        self.state.update(&mut self.document, self.size);
    }
}

impl ViewState {
    fn update(&mut self, doc: &Document, size: (i32, i32) /*, dirty_nodes */) {
        self.layout_results = vec![LayoutResult::default(); 100];

        self.layout_styles
            .insert(Document::ROOT, layout_style(&Style::parse("display: block").unwrap()));

        for node in doc.descendants(Document::ROOT) {
            match doc.node_type(node) {
                NodeType::Element => {
                    self.resolved_styles
                        .insert(node, ViewState::resolve_style(doc.style(node), &Style::EMPTY));
                    self.layout_styles
                        .insert(node, layout_style(&self.resolved_styles[&node]));
                }
                NodeType::Text => {
                    self.paragraphs.insert(node, RefCell::new(create_para(doc.text(node))));
                    self.layout_styles.insert(node, LayoutStyle::default());
                }
                _ => {}
            }
        }

        let tree = LayoutData {
            document: doc,
            styles: &self.layout_styles,
            paragraphs: &self.paragraphs,
        };
        LayoutEngine::new().calculate(Size::new(size.0 as _, size.1 as _), &tree, &mut self.layout_results);
    }

    fn resolve_style(inline_style: Option<&Style>, _parent_style: &Style) -> Style {
        let mut res = Style::parse("display: block").unwrap();

        if let Some(style) = inline_style {
            res.apply(style);
        }

        // TODO: inherit, css-vars?

        res
    }
}

// TODO: not sure if RenderTree should be implemented for Viewport, because it
//       needs to update ViewState first and that would require &mut or RefCell<>
//       or something, or maybe we could change the trait to &mut tree but I'm not
//       yet sure about that
impl RenderTree for (&Document, &ViewState) {
    fn visit<F: FnMut(RenderEdge) -> bool>(&self, visitor: &mut F) {
        self.0.visit(&mut |edge| match edge {
            NodeEdge::Start(node) => {
                // TODO: let rect = viewport.node_rect();
                let LayoutResult { pos: (x, y), size } = self.1.layout_results[node];
                let rect = Rect::new(x, y, x + size.width, y + size.height);

                match self.0.node_type(node) {
                    NodeType::Document => visitor(RenderEdge::OpenContainer(rect, &ContainerStyle::default())),
                    NodeType::Element => visitor(RenderEdge::OpenContainer(
                        rect,
                        &container_style(&self.1.resolved_styles[&node]),
                    )),
                    NodeType::Text => visitor(RenderEdge::Text(rect, &*self.1.paragraphs[&node].borrow())),
                }
            }
            NodeEdge::End => visitor(RenderEdge::CloseContainer),
        })
    }
}

fn create_para(s: &str) -> Paragraph {
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_color(Paint::default());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(s);

    paragraph_builder.build()
}

struct LayoutData<'a> {
    document: &'a Document,
    styles: &'a HashMap<NodeId, LayoutStyle>,
    paragraphs: &'a HashMap<NodeId, RefCell<Paragraph>>,
}

impl LayoutTree for LayoutData<'_> {
    type NodeRef = NodeId;
    type Paragraph = RefCell<Paragraph>;

    fn root(&self) -> NodeId {
        Document::ROOT
    }

    fn children(&self, parent: NodeId) -> &[NodeId] {
        self.document.children(parent)
    }

    fn style(&self, node: NodeId) -> &LayoutStyle {
        &self.styles[&node]
    }

    fn paragraph(&self, node: NodeId) -> Option<&RefCell<Paragraph>> {
        self.paragraphs.get(&node)
    }
}

impl layout::Paragraph for RefCell<Paragraph> {
    fn measure(&self, max_width: f32) -> (f32, f32) {
        let mut para = self.borrow_mut();

        para.layout(max_width);

        return (f32::min(para.max_intrinsic_width(), para.max_width()), para.height());
    }
}
