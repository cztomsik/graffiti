use crate::layout::Paragraph;

use super::{Dimension, Display, LayoutTree, Rect, Size};
use std::ops::IndexMut;

#[derive(Debug, Clone, Copy, Default)]
pub struct LayoutResult {
    pub pos: (f32, f32),
    pub size: Size<f32>,
}

pub(super) struct LayoutContext<'a, T: LayoutTree> {
    pub(super) viewport_size: Size<f32>,
    pub(super) tree: &'a T,
    pub(super) results: &'a mut dyn IndexMut<T::NodeRef, Output = LayoutResult>,
}

impl<T: LayoutTree> LayoutContext<'_, T> {
    pub fn resolve(&self, dim: Dimension, base: f32) -> f32 {
        match dim {
            Dimension::Px(v) => v,
            Dimension::Percent(v) => base * 0.01 * v,
            Dimension::Vw(v) => self.viewport_size.width * v,
            Dimension::Vh(v) => self.viewport_size.height * v,
            Dimension::Vmin(v) => self.viewport_size.min() * v,
            Dimension::Vmax(v) => self.viewport_size.max() * v,
            // TODO: em/rem
            _ => f32::NAN,
        }
    }

    pub fn resolve_size(&self, size: Size<Dimension>, parent_size: Size<f32>) -> Size<f32> {
        Size::new(
            self.resolve(size.width, parent_size.width),
            self.resolve(size.height, parent_size.height),
        )
    }

    pub fn resolve_rect(&self, rect: Rect<Dimension>, base: f32) -> Rect<f32> {
        Rect {
            top: self.resolve(rect.top, base),
            right: self.resolve(rect.right, base),
            bottom: self.resolve(rect.bottom, base),
            left: self.resolve(rect.left, base),
        }
    }

    pub fn compute_node(&mut self, node: T::NodeRef, parent_size: Size<f32>) {
        let style = self.tree.style(node);
        let mut result = LayoutResult::default();

        // TODO: maybe we only need padding + border for positioning and
        //       (padding + border).inner_size() for subtracting the avail_size
        let padding = self.resolve_rect(style.padding, parent_size.width);
        //let border = self.resolve_rect(style.border, parent_size.width);

        result.size = self.resolve_size(style.size, parent_size);

        match style.display {
            Display::Block => self.compute_block(&mut result, &padding, style, self.tree.children(node), parent_size),
            Display::Flex => self.compute_flex(&mut result, style, self.tree.children(node), parent_size),
            Display::Inline => {
                if let Some(para) = self.tree.paragraph(node) {
                    let (width, height) = para.measure(parent_size.width);
                    result.size = Size::new(width, height);
                } else {
                    result.size = Size::default();
                }
            }
            _ => todo!(),
        }

        println!("res node size {:?}", (style.display, result.pos, result.size));

        self.results[node] = result;
    }

    //     pub fn compute_node(&mut self, node: T::NodeRef, parent_size: Size<f32>) {
    //         let style = &self.tree.style(node);
    //         let mut result = LayoutResult::default();

    //         result.size = self.resolve_size(style.size, parent_size);
    //         result.padding = self.resolve_rect(style.padding, parent_size.width);
    //         result.margin = self.resolve_rect(style.margin, parent_size.width);
    //         result.border = self.resolve_rect(style.border, parent_size.width);

    //         match style.display {
    //             Display::None => {}
    //             //Display::Inline => self.compute_inline(layout_box, parent_size),
    //             Display::Block => self.compute_block(&mut result, node, style, parent_size),
    //             Display::Flex => self.compute_flex(node, style, parent_size),
    //             //Display::Table => self.compute_table(style, parent_size),
    //             _ => self.compute_block(&mut result, node, style, parent_size),
    //         }

    //         // TODO: this is because of Display::None (which then breaks sum of children for block)
    //         if result.size.height.is_nan() {
    //             result.size.height = 0.;
    //         }

    //         println!("res node size {:?}", (style.display, result.size));

    //         self.results[node] = result;
    //     }
}
