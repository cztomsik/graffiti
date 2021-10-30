use crate::gfx::Text;

#[derive(Debug, Clone, Copy)]
pub enum Display { None, Inline, Block, Flex }

#[derive(Debug, Clone, Copy)]
pub enum Dimension { Auto, Px(f32), /*Fraction*/ Percent(f32) }

#[derive(Debug, Clone, Copy)]
pub struct Size<T: Copy> { pub width: T, pub height: T }

impl Size<Dimension> {
    pub const AUTO: Self = Self { width: Dimension::Auto, height: Dimension::Auto };
}

#[derive(Debug, Clone, Copy)]
pub struct Rect<T: Copy> { pub top: T, pub right: T, pub bottom: T, pub left: T }

impl Rect<Dimension> {
    pub const ZERO: Self = Self { top: Dimension::Px(0.), right: Dimension::Px(0.), bottom: Dimension::Px(0.), left: Dimension::Px(0.) };
}

#[derive(Debug, Clone, Copy)]
pub enum Align { Auto, FlexStart, Center, FlexEnd, Stretch, Baseline, SpaceBetween, SpaceAround }

#[derive(Debug, Clone, Copy)]
pub enum Justify { FlexStart, Center, FlexEnd, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Debug, Clone, Copy)]
pub enum FlexDirection { Row, Column }

#[derive(Debug, Clone, Copy)]
pub enum FlexWrap { NoWrap, Wrap }

#[derive(Debug, Clone, Copy)]
pub struct LayoutStyle {
    pub display: Display,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub padding: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub border: Rect<Dimension>,

    // flex & grid (not supported ATM)
    pub align_self: Align,
    pub align_content: Align,
    pub align_items: Align,
    pub justify_content: Justify,

    // flex
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            // TODO: make it Inline when cascading works
            display: Display::Block,
            size: Size::AUTO,
            min_size: Size::AUTO,
            max_size: Size::AUTO,
            padding: Rect::ZERO,
            margin: Rect::ZERO,
            border: Rect::ZERO,
            // TODO: position
            // TODO: overflow

            align_self: Align::Auto,
            align_items: Align::Stretch,
            align_content: Align::Stretch,
            justify_content: Justify::FlexStart,

            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.,
            flex_shrink: 1.,
            flex_basis: Dimension::Auto,
        }
    }
}

pub(crate) struct LayoutNode {
    style: LayoutStyle,
    text: Option<Text>,
    children: Vec<Self>
}

impl LayoutNode {
    pub(crate) fn new(style: LayoutStyle, children: Vec<Self>) -> Self {
        Self { style, text: None, children }
    }

    pub(crate) fn new_text(text: Text) -> Self {
        Self { style: LayoutStyle { display: Display::Inline, ..LayoutStyle::default() }, text: Some(text), children: vec![] }
    }

    pub(crate) fn calculate(&self, viewport_size: Size<f32>) -> LayoutBox {
        // create "boxes" first
        // TODO: this can be incremental, it also should remove hidden/empty parts, join texts together, etc.
        let mut root = create_box(self);

        let ctx = Ctx {};
        ctx.compute_box(&mut root, viewport_size);

        root
    }
}

pub struct LayoutBox {
    // TODO: &Node? NodeId?
    style: LayoutStyle,
    pub(crate) text: Option<Text>,

    x: f32,
    y: f32,
    // "inner" size, without padding & border
    size: Size<f32>,
    children: Vec<LayoutBox>,

    // helpers
    // min_size: Size<f32>,
    // max_size: Size<f32>,
    padding: Rect<f32>,
    margin: Rect<f32>,
    border: Rect<f32>,
}

impl LayoutBox {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.size.width + self.padding.left + self.padding.right + self.border.left + self.border.right
    }

    pub fn height(&self) -> f32 {
        self.size.height + self.padding.top + self.padding.bottom + self.border.top + self.border.bottom
    }

    pub fn children(&self) -> &[Self] {
        &self.children
    }
}

// TODO: vw, vh, vmin, vmax, rem
struct Ctx {}

impl Ctx {
    fn resolve(&self, dim: Dimension, base: f32) -> f32 {
        match dim {
            Dimension::Px(v) => v,
            Dimension::Percent(v) => base * v,
            _ => f32::NAN
        }
    }

    fn resolve_size(&self, size: Size<Dimension>, parent_size: Size<f32>) -> Size<f32> {
        Size { width: self.resolve(size.width, parent_size.width), height: self.resolve(size.height, parent_size.height) }
    }

    fn resolve_rect(&self, rect: Rect<Dimension>, base: f32) -> Rect<f32> {
        Rect { top: self.resolve(rect.top, base), right: self.resolve(rect.top, base), bottom: self.resolve(rect.top, base), left: self.resolve(rect.top, base) }
    }

    fn compute_box(&self, layout_box: &mut LayoutBox, parent_size: Size<f32>) {
        layout_box.size = self.resolve_size(layout_box.style.size, parent_size);
        // layout_box.min_size = self.resolve_size(layout_box.style.min_size, parent_size);
        // layout_box.max_size = self.resolve_size(layout_box.style.max_size, parent_size);
        layout_box.padding = self.resolve_rect(layout_box.style.padding, parent_size.width);
        layout_box.margin = self.resolve_rect(layout_box.style.margin, parent_size.width);
        layout_box.border = self.resolve_rect(layout_box.style.border, parent_size.width);

        //println!("compute_box {:?}", layout_box.style.display);
        match layout_box.style.display {
            // TODO: maybe do not create box? is it worth?
            Display::None => {},
            Display::Inline => self.compute_inline(layout_box, parent_size),
            Display::Block => self.compute_block(layout_box, parent_size),
            Display::Flex => self.compute_flex(layout_box, parent_size),
        }

        // TODO: this is because of Display::None
        if layout_box.size.height.is_nan() {
            layout_box.size.height = 0.;
        }
    }

    fn compute_inline(&self, inline: &mut LayoutBox, avail_size: Size<f32>) {
        if let Some(text) = &inline.text {
            let (width, height) = text.measure(avail_size.width);
            println!("measure {} {:?}", text.text(), height);
            inline.size = Size { width, height };
        }
    }

    fn compute_block(&self, block: &mut LayoutBox, parent_size: Size<f32>) {
        if block.size.width.is_nan() {
            block.size.width = parent_size.width;
        }

        let mut y = block.padding.top;

        // TODO: filter position != absolute/fixed
        for child in &mut block.children {
            self.compute_box(child, parent_size);
            child.y = y;
            child.x = block.padding.left;

            y += child.size.height;
        }

        if block.size.height.is_nan() {
            block.size.height = block.children.iter().map(|ch| ch.size.height).sum();
        }

        println!("{:?}", block.size);
        // // TODO: add padding_x + border_x to defined width/height
        // block.size.width = self.resolve(block.style.size.width, parent_size.width).unwrap_or(avail_size.width);

        // let avail_size = block.size; // TODO - padding_x - border_x
        // let mut y = 0.;

        // for ch in &mut block.children {
        //     // TODO - margin_x
        //     self.compute_box(ch, avail_size, block.size);

        //     // TODO: collapsing
        //     // TODO: y += margin.top

        //     // TODO: ch.y = ...
        // }

        // // TODO: add padding_y + border_y to defined width/height
        // block.size.height = self.resolve(block.style.size.height, parent_size.height).unwrap_or(inner_size.height)
    }

    fn compute_flex(&self, flex: &mut LayoutBox, parent_size: Size<f32>) {
        // TODO: determine_available_space(), distribute_free_space(), etc.
        self.compute_block(flex, parent_size);
    }
}

fn create_box<'a>(node: &'a LayoutNode) -> LayoutBox {
    LayoutBox {
        style: node.style,
        text: node.text.clone(),
        children: node.children.iter().map(create_box).collect(),
        x: 0.,
        y: 0.,
        size: Size { width: 0., height: 0. },
        // min_size: Size { width: 0., height: 0. },
        // max_size: Size { width: 0., height: 0. },
        padding: Rect { top: 0., right: 0., bottom: 0., left: 0. },
        margin: Rect { top: 0., right: 0., bottom: 0., left: 0. },
        border: Rect { top: 0., right: 0., bottom: 0., left: 0. },
    }
}
