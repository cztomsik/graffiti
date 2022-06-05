use context::LayoutContext;
use std::ops::IndexMut;

pub use {context::LayoutResult, style::*};

pub trait LayoutTree {
    type NodeRef: Copy;
    type Paragraph: Paragraph;

    fn root(&self) -> Self::NodeRef;
    fn children(&self, parent: Self::NodeRef) -> &[Self::NodeRef];
    fn style(&self, node: Self::NodeRef) -> &LayoutStyle;
    fn paragraph(&self, node: Self::NodeRef) -> Option<&Self::Paragraph>;

    // TODO: is_dirty/flags(node)
}

pub trait Paragraph {
    fn measure(&self, max_width: f32) -> (f32, f32);
}

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate<'a, T: LayoutTree>(
        &self,
        viewport_size: Size<f32>,
        tree: &T,
        results: &mut dyn IndexMut<T::NodeRef, Output = LayoutResult>,
    ) {
        let mut ctx = LayoutContext {
            viewport_size,
            tree,
            results,
        };

        ctx.compute_node(tree.root(), viewport_size);
    }
}

#[cfg(test)]
#[macro_use]
mod test_util;

mod block;
mod context;
mod flex;
mod inline;
mod style;
mod table;
