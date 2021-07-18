use crate::css::{
    matching_rules, CssAlign, CssDimension, CssDisplay, CssFlexDirection, CssFlexWrap, CssJustify, CssPosition, Style,
    StyleProp, StyleSheet,
};
use crate::gfx::{Frame, Text, TextStyle, Vec2, AABB};
use crate::layout::{Align, Dimension, Display, FlexDirection, FlexWrap, Justify, LayoutNode, LayoutStyle, Position};
use crate::renderer::Renderer;
use crate::util::SlotMap;
use crate::{Document, DocumentEvent, NodeId, NodeType};
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::rc::Rc;

pub struct Viewport {
    size: Cell<(i32, i32)>,

    document: Rc<RefCell<Document>>,
    // TODO: something like hibitset?
    dirty_nodes: Rc<RefCell<BTreeSet<NodeId>>>,
    styles: Rc<RefCell<SlotMap<NodeId, Style>>>,
    layout_nodes: Rc<RefCell<SlotMap<NodeId, LayoutNode>>>,
    texts: Rc<RefCell<SlotMap<NodeId, Text>>>,
    renderer: Renderer,
}

impl Viewport {
    pub fn new(size: (i32, i32), document: &Rc<RefCell<Document>>) -> Self {
        let styles = Rc::new(RefCell::new(SlotMap::new()));
        let dirty_nodes = Rc::new(RefCell::new(BTreeSet::new()));
        let layout_nodes = Rc::new(RefCell::new(SlotMap::new()));
        let texts = Rc::new(RefCell::new(SlotMap::new()));
        let renderer = Renderer::new(document, &layout_nodes, &styles, &texts);

        // create root layout node
        layout_nodes
            .borrow_mut()
            .put(document.borrow().root(), LayoutNode::new());
        layout_nodes.borrow_mut()[0].set_style(Style::from("display: block; width: 100%; height: 100%").props().into());

        let viewport = Self {
            size: size.into(),
            document: Rc::clone(&document),
            dirty_nodes: Rc::clone(&dirty_nodes),
            layout_nodes: Rc::clone(&layout_nodes),
            styles: Rc::clone(&styles),
            texts: Rc::clone(&texts),
            renderer,
        };

        document.borrow_mut().add_listener(move |doc, e| {
            use DocumentEvent::*;

            match *e {
                Create(node, NodeType::Element) => {
                    layout_nodes.borrow_mut().put(node, LayoutNode::new());
                    styles.borrow_mut().put(node, Style::new());
                }
                Create(node, NodeType::Text) => {
                    let texts2 = Rc::clone(&texts);
                    let measure = move |max_width| texts2.borrow()[node].measure(max_width);

                    texts
                        .borrow_mut()
                        .put(node, Text::new(doc.cdata(node), &TextStyle::DEFAULT));
                    layout_nodes.borrow_mut().put(node, LayoutNode::new_leaf(measure));
                }
                Create(node, NodeType::Comment) => {
                    layout_nodes.borrow_mut().put(node, LayoutNode::new());
                    layout_nodes.borrow_mut()[node].set_style(Style::HIDDEN.props().into());
                }

                Insert(parent, child, index) => {
                    layout_nodes.borrow()[parent].insert_child(&layout_nodes.borrow()[child], index)
                }
                Remove(parent, child) => layout_nodes.borrow()[parent].remove_child(&layout_nodes.borrow()[child]),

                Cdata(node, cdata) => {
                    texts.borrow_mut()[node].set_text(cdata);
                    layout_nodes.borrow()[node].mark_dirty();
                }

                Drop(node, node_type) => {
                    dirty_nodes.borrow_mut().remove(&node);
                    layout_nodes.borrow_mut().remove(node);

                    if node_type == NodeType::Element {
                        styles.borrow_mut().remove(node);
                    }
                }

                _ => {}
            }
        });

        viewport
    }

    pub fn document(&self) -> &Rc<RefCell<Document>> {
        &self.document
    }

    pub fn render(&self) -> Frame {
        self.update();

        self.renderer.render()
    }

    // TODO: this is wrong anyway because children can be bigger than their parents
    //       so we will need some kind of AABB tree anyway
    // TODO: display: none
    // TODO: scroll
    // TODO: clip (hidden and/or radius)
    pub fn element_from_point(&self, point: (f32, f32)) -> Option<NodeId> {
        let pos = Vec2::from(point);

        let document = self.document.borrow();
        let layout_nodes = self.layout_nodes.borrow();

        let mut parent = document.first_child(document.root())?;
        let mut offset = Vec2::ZERO;
        let mut continue_down;

        // go down (starting from root) through each matching surface and return the last & deepest one
        loop {
            continue_down = false;
            offset = Vec2::from(layout_nodes[parent].offset()) + offset;

            for el in document.children(parent) {
                let rect = AABB::new(Vec2::ZERO, Vec2::from(layout_nodes[el].size()))
                    + Vec2::from(layout_nodes[el].offset())
                    + offset;

                if rect.contains(pos) {
                    parent = el;
                    continue_down = true;
                }
            }

            if !continue_down {
                return Some(parent);
            }
        }
    }

    // TODO: caretPositionFromPoint

    // TODO: getClientRect, offsetLeft, offsetTop, offsetWidth, offsetHeight

    // TODO: scrollTo(), scrollTop, ...

    // TODO: computed_style?

    pub fn resize(&self, size: (i32, i32)) {
        self.size.set(size);
        self.update();
    }

    fn update(&self) {
        self.update_styles();
        self.update_layout();
    }

    fn update_styles(&self) {
        let doc = self.document.borrow();
        let mut styles = self.styles.borrow_mut();
        let layout_nodes = self.layout_nodes.borrow_mut();

        let mut sheets: Vec<_> = doc
            .query_selector_all(doc.root(), "html > head > style")
            .iter()
            //.inspect(|s| println!("style: {}", doc.text_content(**s)))
            .map(|s| StyleSheet::from(&*doc.text_content(*s)))
            .collect();

        sheets.insert(0, StyleSheet::from(include_str!("../resources/ua.css")));

        doc.with_matching_context(|ctx| {
            for (el, out) in styles.iter_mut() {
                // TODO: just iterate props, no need to merge anymore
                let mut style = Style::new();

                for r in matching_rules(&ctx, &sheets, el) {
                    for p in r.style().props() {
                        style.add_prop(p.clone());
                    }
                }

                // add inline style
                for p in doc.element_style(el).props() {
                    style.add_prop(p.clone());
                }

                layout_nodes[el].set_style(style.props().into());

                // TODO: keep just renderstyle
                *out = style;
            }
        });
    }

    fn update_layout(&self) {
        let (w, h) = self.size.get();
        let size = (w as _, h as _);

        self.layout_nodes.borrow()[self.document.borrow().root()].calculate(size);
    }
}

impl<'a, I: Iterator<Item = &'a StyleProp>> From<I> for LayoutStyle {
    fn from(props: I) -> Self {
        let mut res = LayoutStyle::default();

        for p in props {
            use StyleProp as P;

            match *p {
                // size
                P::Width(v) => res.width = v.into(),
                P::Height(v) => res.height = v.into(),
                P::MinWidth(v) => res.min_width = v.into(),
                P::MinHeight(v) => res.min_height = v.into(),
                P::MaxWidth(v) => res.max_width = v.into(),
                P::MaxHeight(v) => res.max_height = v.into(),

                // padding
                P::PaddingTop(v) => res.padding_top = v.into(),
                P::PaddingRight(v) => res.padding_right = v.into(),
                P::PaddingBottom(v) => res.padding_bottom = v.into(),
                P::PaddingLeft(v) => res.padding_left = v.into(),

                // margin
                P::MarginTop(v) => res.margin_top = v.into(),
                P::MarginRight(v) => res.margin_right = v.into(),
                P::MarginBottom(v) => res.margin_bottom = v.into(),
                P::MarginLeft(v) => res.margin_left = v.into(),

                // position
                P::Position(v) => res.position = v.into(),
                P::Top(v) => res.top = v.into(),
                P::Right(v) => res.right = v.into(),
                P::Bottom(v) => res.bottom = v.into(),
                P::Left(v) => res.left = v.into(),

                // flex
                P::FlexGrow(v) => res.flex_grow = v,
                P::FlexShrink(v) => res.flex_shrink = v,
                P::FlexBasis(v) => res.flex_basis = v.into(),
                P::FlexWrap(v) => res.flex_wrap = v.into(),
                P::FlexDirection(v) => res.flex_direction = v.into(),
                P::AlignContent(v) => res.align_content = v.into(),
                P::AlignItems(v) => res.align_items = v.into(),
                P::AlignSelf(v) => res.align_self = v.into(),
                P::JustifyContent(v) => res.justify_content = v.into(),

                // other
                P::Display(v) => {
                    res.display = match v {
                        CssDisplay::None => Display::None,
                        CssDisplay::Flex => Display::Flex,
                        // TODO
                        CssDisplay::Block => {
                            // weird but correct, what's missing is that all inlines should
                            // be wrapped in anonymous box/line, then it should work fine
                            // (but that "line" is not just flex-wrap: wrap)
                            res.flex_direction = FlexDirection::Column;
                            res.align_items = Align::Stretch;
                            Display::Flex
                        }
                        // TODO
                        CssDisplay::Inline => {
                            //res.flex_direction = FlexDirection::Row;
                            //res.flex_wrap = FlexWrap::Wrap;
                            Display::Flex
                        }
                    }
                }

                // TODO: remove
                _ => {}
            }
        }

        res
    }
}

impl From<CssDimension> for Dimension {
    fn from(v: CssDimension) -> Self {
        match v {
            CssDimension::Px(v) => Self::Px(v),
            CssDimension::Percent(v) => Self::Percent(v),
            CssDimension::Auto => Self::Auto,
            // ? => Self::Undefined,
        }
    }
}

impl From<CssAlign> for Align {
    fn from(v: CssAlign) -> Align {
        match v {
            CssAlign::Auto => Self::Auto,
            CssAlign::FlexStart => Self::FlexStart,
            CssAlign::Center => Self::Center,
            CssAlign::FlexEnd => Self::FlexEnd,
            CssAlign::Stretch => Self::Stretch,
            CssAlign::Baseline => Self::Baseline,
            CssAlign::SpaceBetween => Self::SpaceBetween,
            CssAlign::SpaceAround => Self::SpaceAround,
        }
    }
}

impl From<CssJustify> for Justify {
    fn from(v: CssJustify) -> Justify {
        match v {
            CssJustify::FlexStart => Self::FlexStart,
            CssJustify::Center => Self::Center,
            CssJustify::FlexEnd => Self::FlexEnd,
            CssJustify::SpaceBetween => Self::SpaceBetween,
            CssJustify::SpaceAround => Self::SpaceAround,
            CssJustify::SpaceEvenly => Self::SpaceEvenly,
        }
    }
}

impl From<CssFlexWrap> for FlexWrap {
    fn from(v: CssFlexWrap) -> Self {
        match v {
            CssFlexWrap::NoWrap => Self::NoWrap,
            CssFlexWrap::Wrap => Self::Wrap,
            CssFlexWrap::WrapReverse => Self::WrapReverse,
        }
    }
}

impl From<CssPosition> for Position {
    fn from(v: CssPosition) -> Self {
        match v {
            CssPosition::Absolute => Self::Absolute,
            // TODO
            _ => Self::Relative,
        }
    }
}

impl From<CssFlexDirection> for FlexDirection {
    fn from(v: CssFlexDirection) -> Self {
        match v {
            CssFlexDirection::Row => Self::Row,
            CssFlexDirection::Column => Self::Column,
            CssFlexDirection::RowReverse => Self::RowReverse,
            CssFlexDirection::ColumnReverse => Self::ColumnReverse,
        }
    }
}
