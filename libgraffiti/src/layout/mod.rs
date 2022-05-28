use std::ops::{Index, IndexMut};

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate<'a, K: Copy>(
        &self,
        viewport_size: Size<f32>,
        node: K,
        children: &'a dyn Index<K, Output = [K]>,
        styles: &'a dyn Index<K, Output = LayoutStyle>,
        results: &'a mut dyn IndexMut<K, Output = LayoutResult>,
    ) {
        let mut ctx = LayoutContext {
            viewport_size,
            children,
            styles,
            results,
        };
        ctx.compute_node(node, viewport_size);
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

pub use {context::LayoutResult, style::*};

use context::LayoutContext;
