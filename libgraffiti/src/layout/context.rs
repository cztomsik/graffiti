use super::{Dimension, Display, LayoutStyle, Rect, Size};
use std::ops::{Index, IndexMut};

// TODO: em/rem
pub(super) struct LayoutContext<'a, K> {
    pub(super) viewport_size: Size<f32>,
    pub(super) children: &'a dyn Index<K, Output = [K]>,
    pub(super) styles: &'a dyn Index<K, Output = LayoutStyle>,
    pub(super) results: &'a mut dyn IndexMut<K, Output = LayoutResult>,
}

impl<K: Copy> LayoutContext<'_, K> {
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
            right: self.resolve(rect.top, base),
            bottom: self.resolve(rect.top, base),
            left: self.resolve(rect.top, base),
        }
    }

    pub fn compute_node(&mut self, node: K, parent_size: Size<f32>) {
        let style = &self.styles[node];

        self.results[node].size = self.resolve_size(style.size, parent_size);
        // self.results[node].min_size = self.resolve_size(layout_box.style.min_size, parent_size);
        // self.results[node].max_size = self.resolve_size(layout_box.style.max_size, parent_size);
        self.results[node].padding = self.resolve_rect(style.padding, parent_size.width);
        self.results[node].margin = self.resolve_rect(style.margin, parent_size.width);
        self.results[node].border = self.resolve_rect(style.border, parent_size.width);

        match style.display {
            Display::None => {}
            //Display::Inline => self.compute_inline(layout_box, parent_size),
            Display::Block => self.compute_block(node, style, parent_size),
            Display::Flex => self.compute_flex(node, style, parent_size),
            //Display::Table => self.compute_table(style, parent_size),
            _ => self.compute_block(node, style, parent_size),
        }

        // TODO: this is because of Display::None (which then breaks sum of children for block)
        if self.results[node].size.height.is_nan() {
            self.results[node].size.height = 0.;
        }

        println!("res node size {:?}", (style.display, self.results[node].size));
    }

    // fn compute_box(&self, layout_box: &mut LayoutBox, parent_size: Size<f32>) {
    //     self.init_box(layout_box, parent_size);
    // }

    // fn compute_inline(&self, inline: &mut LayoutBox, avail_size: Size<f32>) {
    //     if let Some(text) = &inline.text {
    //         let (width, height) = text.measure(avail_size.width);
    //         //println!("measure {} {:?}", text.text(), (width, height));
    //         inline.size = Size::new(width, height);
    //     }
    // }
}

#[derive(Default)]
pub struct LayoutResult {
    pub x: f32,
    pub y: f32,
    pub size: Size<f32>,
    pub border: Rect<f32>,
    pub padding: Rect<f32>,
    pub margin: Rect<f32>,
}

impl LayoutResult {
    pub fn border_rect(&self) -> Rect<f32> {
        Rect {
            left: self.x,
            top: self.y,
            right: self.x
                + self.size.width
                + self.padding.left
                + self.padding.right
                + self.border.left
                + self.border.right,
            bottom: self.y
                + self.size.height
                + self.padding.top
                + self.padding.top
                + self.border.top
                + self.border.bottom,
        }
    }
}
