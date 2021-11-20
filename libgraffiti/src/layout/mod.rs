#![allow(unused)]

use crate::util::{IdTree, SlotMap};
use std::num::NonZeroU32;

// LISPish macro
//
// let (tree, root) = layout_tree! (node(style_prop = val, ...)
//   (node(...) ...)
//   (text "hello")
//   ...
// )
macro_rules! layout_tree {
    ($root:tt) => ({
        let mut tree = crate::layout::LayoutTree::new();
        let root = layout_tree!(tree, $root);
        (tree, root)
    });
    // TODO: text
    ($tree:expr, ( text $text:literal )) => ($tree.create_node());
    ($tree:expr, ( $tag:ident ($($prop:ident = $val:expr),*) $($inner:tt)* )) => ({
        let node = $tree.create_node();
        let mut style = crate::layout::types::LayoutStyle::default();
        $(style.$prop = $val;)*
        $tree.set_style(node, style);

        for child in [ $(layout_tree!($tree, $inner)),* ] {
            $tree.append_child(node, child);
        }

        node
    });
}

mod block;
mod flex;
mod table;
mod types;

pub use types::*;

pub type LayoutNodeId = NonZeroU32;
type NodeId = LayoutNodeId;

#[derive(Default)]
pub struct LayoutTree {
    // TODO: I am not 100% sure about this yet, IdTree<> might get fragmented
    //       unless we rebuild it but children: Vec<NodeId> might need more
    //       allocations for inserts & appends
    tree: IdTree<LayoutNode>,

    results: SlotMap<NodeId, LayoutResult>,
}

#[derive(Default)]
pub struct LayoutResult {
    x: f32,
    y: f32,
    size: Size<f32>,
    padding: Rect<f32>,
    // margin: Rect<f32>,
    // border: Rect<f32>,
}

impl LayoutResult {
    pub fn outer_rect(&self) -> Rect<f32> {
        Rect {
            left: self.x,
            top: self.y,
            right: self.x + self.size.width + self.padding.left + self.padding.right,
            bottom: self.y + self.size.height + self.padding.top + self.padding.top
        }
    }
}

impl LayoutTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_node(&mut self) -> NodeId {
        let id = self.tree.create_node(LayoutNode {
            style: LayoutStyle::default(),
        });
        self.results.put(
            id,
            LayoutResult::default(),
        );

        id
    }

    pub fn drop_node(&mut self, node: NodeId) {
        self.tree.drop_node(node);
    }

    pub fn style(&self, node: NodeId) -> &LayoutStyle {
        &self.tree.data(node).style
    }

    pub fn set_style(&mut self, node: NodeId, style: LayoutStyle) {
        self.tree.data_mut(node).style = style;
    }

    pub fn children(&self, parent: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.tree.children(parent)
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        self.tree.append_child(parent, child);
    }

    pub fn insert_before(&mut self, parent: NodeId, child: NodeId, before: NodeId) {
        self.tree.insert_before(parent, child, before);
    }

    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) {
        self.tree.remove_child(parent, child);
    }

    pub fn layout_result(&self, node: NodeId) -> &LayoutResult {
        &self.results[node]
    }

    pub fn calculate(&mut self, node: NodeId, avail_width: f32, avail_height: f32) {
        println!("-- calculate");

        let mut ctx = Ctx { tree: &mut self.tree };

        ctx.compute_node(
            &mut self.results,
            node,
            Size {
                width: avail_width,
                height: avail_height,
            },
        );
    }

    fn debug(&self, node: NodeId) -> String {
        struct DebugRef<'a>(&'a LayoutTree, NodeId);
        impl std::fmt::Debug for DebugRef<'_> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                let node = self.0.tree.data(self.1);
                let size = self.0.results[self.1].size;

                write!(fmt, "{:?}({:?}, {:?}) ", node.style.display, size.width, size.height)?;
                fmt.debug_list()
                    .entries(self.0.tree.children(self.1).map(|id| DebugRef(self.0, id)))
                    .finish()
            }
        }

        format!("{:?}", DebugRef(self, node))
    }
}

// TODO: text
struct LayoutNode {
    style: LayoutStyle,
}

// read-only scope available to all layout impls
// TODO: vw, vh, vmin, vmax, rem
struct Ctx<'a> {
    tree: &'a mut IdTree<LayoutNode>,
}

impl Ctx<'_> {
    fn resolve(&self, dim: Dimension, base: f32) -> f32 {
        match dim {
            Dimension::Px(v) => v,
            Dimension::Percent(v) => base * v,
            _ => f32::NAN,
        }
    }

    fn resolve_size(&self, size: Size<Dimension>, parent_size: Size<f32>) -> Size<f32> {
        Size {
            width: self.resolve(size.width, parent_size.width),
            height: self.resolve(size.height, parent_size.height),
        }
    }

    fn resolve_rect(&self, rect: Rect<Dimension>, base: f32) -> Rect<f32> {
        Rect {
            top: self.resolve(rect.top, base),
            right: self.resolve(rect.top, base),
            bottom: self.resolve(rect.top, base),
            left: self.resolve(rect.top, base),
        }
    }

    fn compute_node(&self, results: &mut SlotMap<NodeId, LayoutResult>, node: NodeId, parent_size: Size<f32>) {
        println!(
            "compute_node {:?}",
            (node, self.tree.data(node).style.display, parent_size,)
        );

        let style = &self.tree.data(node).style;

        results[node].size = self.resolve_size(style.size(), parent_size);
        // results[node].min_size = self.resolve_size(layout_box.style.min_size, parent_size);
        // results[node].max_size = self.resolve_size(layout_box.style.max_size, parent_size);
        results[node].padding = self.resolve_rect(style.padding(), parent_size.width);
        // results[node].margin = self.resolve_rect(style.margin, parent_size.width);
        // results[node].border = self.resolve_rect(style.border, parent_size.width);

        match self.tree.data(node).style.display {
            // TODO: maybe do not create box? is it worth?
            Display::None => {}
            //Display::Inline => self.compute_inline(layout_box, parent_size),
            Display::Block => self.compute_block(results, node, parent_size),
            Display::Flex => self.compute_flex(results, node, parent_size),
            //Display::Table => self.compute_table(results, node, parent_size),
            _ => self.compute_block(results, node, parent_size),
        }

        // TODO: this is because of Display::None (which then breaks sum of children for block)
        if results[node].size.height.is_nan() {
            results[node].size.height = 0.;
        }

        println!("res node size {:?}", (self.tree.data(node).style.display, results[node].size));
    }

    // fn compute_box(&self, layout_box: &mut LayoutBox, parent_size: Size<f32>) {
    //     self.init_box(layout_box, parent_size);

    //     println!("compute_box {:?}", layout_box.style.display);
    //     match layout_box.style.display {
    //         // TODO: maybe do not create box? is it worth?
    //         Display::None => {}
    //         //Display::Inline => self.compute_inline(layout_box, parent_size),
    //         Display::Block => self.compute_block(layout_box, parent_size),
    //         Display::Flex => self.compute_flex(layout_box, parent_size),
    //         Display::Table => self.compute_table(layout_box, parent_size),
    //         _ => self.compute_block(layout_box, parent_size),
    //     }

    //     // TODO: this is because of Display::None
    //     if layout_box.size.height.is_nan() {
    //         layout_box.size.height = 0.;
    //     }
    // }

    // fn compute_inline(&self, inline: &mut LayoutBox, avail_size: Size<f32>) {
    //     if let Some(text) = &inline.text {
    //         let (width, height) = text.measure(avail_size.width);
    //         //println!("measure {} {:?}", text.text(), (width, height));
    //         inline.size = Size { width, height };
    //     }
    // }
}
