pub struct Viewport {
    size: (i32, i32),
    document: Document,
    renderer: Renderer,
    state: ViewState,
}

struct ViewState {
    resolved_styles: HashMap<NodeId, Style>,
    paragraphs: HashMap<NodeId, Paragraph>,
    layout_styles: Vec<LayoutStyle>,
    layout_results: Vec<LayoutResult>,
    render_tree: Vec<RenderEdge<Paragraph>>,
    // TODO: scroll, selection, focus + tab_next()
}

impl Viewport {
    pub fn new(size: (i32, i32), document: Document, renderer: Renderer) -> Self {
        Self {
            size,
            document,
            renderer,
            state: ViewState::new(),
        }
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn resize(&mut self, size: (i32, i32)) {
        self.size = size;
        self.renderer.resize(size);
    }

    pub fn render(&mut self) {
        self.update();
        self.renderer.render(&self.state.render_tree);
    }

    fn update(&mut self) {
        self.state.update(&self.document);
    }
}

