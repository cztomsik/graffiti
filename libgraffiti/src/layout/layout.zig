// use crate::layout::Paragraph;

// use super::{Dimension, Display, LayoutTree, Rect, Size};
// use std::ops::IndexMut;

// pub(super) struct LayoutContext<'a, T: LayoutTree> {
//     pub(super) viewport_size: Size<f32>,
//     pub(super) tree: &'a T,
//     pub(super) results: &'a mut dyn IndexMut<T::NodeRef, Output = LayoutResult>,
// }

// impl<T: LayoutTree> LayoutContext<'_, T> {
//     pub fn compute_node(&mut self, node: T::NodeRef, parent_size: Size<f32>) {
//         let style = self.tree.style(node);
//         let mut result = LayoutResult::default();

//         // TODO: maybe we only need padding + border for positioning and
//         //       (padding + border).inner_size() for subtracting the avail_size
//         let padding = self.resolve_rect(style.padding, parent_size.width);
//         //let border = self.resolve_rect(style.border, parent_size.width);

//         result.size = self.resolve_size(style.size, parent_size);

//         match style.display {
//             Display::None => result.size = Size::new(0., 0.),
//             Display::Block => self.compute_block(&mut result, &padding, style, self.tree.children(node), parent_size),
//             Display::Flex => self.compute_flex(&mut result, style, self.tree.children(node), parent_size),
//             // TODO: <button>, <input>
//             Display::InlineBlock => {
//                 self.compute_block(&mut result, &padding, style, self.tree.children(node), parent_size)
//             }
//             Display::Inline => {
//                 if let Some(para) = self.tree.paragraph(node) {
//                     let (width, height) = para.measure(parent_size.width);
//                     result.size = Size::new(width, height);
//                 } else {
//                     result.size = Size::default();
//                 }
//             }
//             _ => todo!(),
//         }

//         println!("res node size {:?}", (style.display, result.pos, result.size));

//         self.results[node] = result;
//     }

//     //     pub fn compute_node(&mut self, node: T::NodeRef, parent_size: Size<f32>) {
//     //         let style = &self.tree.style(node);
//     //         let mut result = LayoutResult::default();

//     //         result.size = self.resolve_size(style.size, parent_size);
//     //         result.padding = self.resolve_rect(style.padding, parent_size.width);
//     //         result.margin = self.resolve_rect(style.margin, parent_size.width);
//     //         result.border = self.resolve_rect(style.border, parent_size.width);

//     //         println!("res node size {:?}", (style.display, result.size));

//     //         self.results[node] = result;
//     //     }
// }

const std = @import("std");
const Style = @import("../style.zig").Style;
const Dimension = @import("../style.zig").Dimension;

pub const Pos = struct { x: f32 = 0, y: f32 = 0 };
pub const Size = struct { width: f32 = 0, height: f32 = 0 };
pub const Layout = struct { pos: Pos = .{}, size: Size = .{} };

// TODO: *const Style + *Layout pointing to a vec of results?
pub const LayoutNode = struct { first_child: ?*LayoutNode = null, next: ?*LayoutNode = null, style: Style, text: ?[]const u8 = null, layout: Layout = .{} };

pub fn calculate(node: *LayoutNode, avail_size: Size) void {
    _ = (LayoutContext{ .avail_size = avail_size }).compute_node(node, avail_size);
}

const LayoutContext = struct {
    // TODO: vw, vh, ...
    avail_size: Size,

    const Self = @This();

    fn resolve(
        _: *Self,
        val: Dimension,
        // base: f32
    ) f32 {
        return switch (val) {
            .auto => std.math.nan_f32,
            .px => |px| px,
            // Dimension::Percent(v) => base * 0.01 * v,
            // Dimension::Vw(v) => self.viewport_size.width * v,
            // Dimension::Vh(v) => self.viewport_size.height * v,
            // Dimension::Vmin(v) => self.viewport_size.min() * v,
            // Dimension::Vmax(v) => self.viewport_size.max() * v,
            // // TODO: em/rem
        };
    }

    // fn resolve_size(&self, size: Size<Dimension>, parent_size: Size<f32>) -> Size<f32> {
    //     Size::new(
    //         self.resolve(size.width, parent_size.width),
    //         self.resolve(size.height, parent_size.height),
    //     )
    // }

    // fn resolve_rect(&self, rect: Rect<Dimension>, base: f32) -> Rect<f32> {
    //     Rect {
    //         top: self.resolve(rect.top, base),
    //         right: self.resolve(rect.right, base),
    //         bottom: self.resolve(rect.bottom, base),
    //         left: self.resolve(rect.left, base),
    //     }
    // }

    fn compute_node(self: *Self, node: *LayoutNode, parent_size: Size) Size {
        node.layout.size = .{ .width = self.resolve(node.style.width), .height = self.resolve(node.style.height) };

        switch (node.style.display) {
            .none => node.layout.size = .{},
            .block => self.compute_block(node, parent_size),
            .@"flex" => self.compute_flex(node, parent_size),
            .@"inline" => self.compute_inline(node),
            // else => {},
        }

        // std.debug.print("{*} {} {d:.2} -> {d:.2}@{d:.2}\n", .{ node, node.style.display, parent_size.width, node.layout.size.width, node.layout.size.height });

        return node.layout.size;
    }

    fn compute_inline(_: *Self, node: *LayoutNode) void {
        node.layout.size = if (node.text) |t| .{ .width = 10 * @intToFloat(f32, t.len), .height = 40 } else .{};
    }

    fn compute_block(self: *Self, node: *LayoutNode, parent_size: Size) void {
        var y: f32 = self.resolve(node.style.padding_top);
        var content_height: f32 = 0;

        const avail_inner = .{
            .width = @maximum(0, parent_size.width - self.resolve(node.style.padding_left) - self.resolve(node.style.padding_right)),
            .height = @maximum(0, parent_size.height - self.resolve(node.style.padding_top) - self.resolve(node.style.padding_bottom)),
        };

        var next = node.first_child;
        while (next) |ch| : (next = ch.next) {
            _ = self.compute_node(ch, avail_inner);

            ch.layout.pos = .{ .x = self.resolve(node.style.padding_left), .y = y };

            content_height += ch.layout.size.height;
            y += ch.layout.size.height;
        }

        if (std.math.isNan(node.layout.size.width)) {
            node.layout.size.width = parent_size.width;
        }

        if (std.math.isNan(node.layout.size.height)) {
            node.layout.size.height = content_height + self.resolve(node.style.padding_top) + self.resolve(node.style.padding_bottom);
        }
    }

    fn compute_flex(_: *Self, _: *LayoutNode, _: Size) void {
        @panic("TODO");
        //     let dir = style.flex_direction;

        //     // TODO: if not defined
        //     let available_space = self.resolve_size(style.size, parent_size);

        //     // TODO: node.total_flex_basis() or something like that
        //     let total_flex_basis: f32 = children
        //         .iter()
        //         .map(|&ch| {
        //             let mut res = self.resolve(self.tree.style(ch).flex_basis, parent_size.main(dir));
        //             if res.is_nan() {
        //                 // compute max-content size?
        //                 todo!()
        //             }

        //             res
        //         })
        //         .sum();
        //     let remaining_space = available_space.main(dir) - total_flex_basis;
        //     let total_grow: f32 = children.iter().map(|&ch| self.tree.style(ch).flex_grow).sum();

        //     //println!("{:?}", (available_space, total_flex_basis, remaining_space, total_grow));
        //     for &child in children {
        //         let child_style = &self.tree.style(child);
        //         let child_res = &mut self.results[child];

        //         if child_style.flex_grow > 0. {
        //             child_res
        //                 .size
        //                 .set_main(dir, (child_style.flex_grow / total_grow) * remaining_space);
        //             child_res.size.set_cross(dir, available_space.cross(dir));
        //             println!("{:?}", (child_style.flex_grow, child_res.size));
        //         } else {
        //             println!("TODO: nonflexible items should be already resolved here");
        //         }
        //     }
        // }

        // // flexbox extensions
        // impl<T: Copy> Size<T> {
        //     fn main(&self, dir: FlexDirection) -> T {
        //         match dir {
        //             FlexDirection::Row => self.width,
        //             FlexDirection::Column => self.height,
        //         }
        //     }

        //     fn set_main(&mut self, dir: FlexDirection, val: T) {
        //         match dir {
        //             FlexDirection::Row => self.width = val,
        //             FlexDirection::Column => self.height = val,
        //         }
        //     }

        //     fn cross(&self, dir: FlexDirection) -> T {
        //         match dir {
        //             FlexDirection::Row => self.height,
        //             FlexDirection::Column => self.width,
        //         }
        //     }

        //     fn set_cross(&mut self, dir: FlexDirection, val: T) {
        //         match dir {
        //             FlexDirection::Row => self.height = val,
        //             FlexDirection::Column => self.width = val,
        //         }
        //     }
        // }

    }
};

test "block" {
    // #[test]
    // fn fixed_width_height() {
    //     let calculate = layout_tree! {
    //         (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
    //     };

    //     let results = calculate(Size::new(0., 0.));
    //     assert_eq!(results[0].size, Size::new(10., 10.));
    // }

    // #[test]
    // fn fixed_height() {
    //     let calculate = layout_tree! {
    //         (node(display = Block, size.height = Px(10.)))
    //     };

    //     let results = calculate(Size::new(0., 10.));
    //     assert_eq!(results[0].size, Size::new(0., 10.));

    //     let results = calculate(Size::new(10., 0.));
    //     assert_eq!(results[0].size, Size::new(10., 10.));
    // }

    // #[test]
    // fn content_height() {
    //     let calculate = layout_tree! {
    //         (node(display = Block)
    //             (node(display = Block, size.width = Px(10.), size.height = Px(10.)))
    //             (node(display = Block, size.height = Px(10.)))
    //         )
    //     };

    //     let results = calculate(Size::new(100., 0.));
    //     assert_eq!(results[0].size, Size::new(100., 20.));
    //     assert_eq!(results[1].size, Size::new(10., 10.));
    //     assert_eq!(results[2].size, Size::new(100., 10.));
    // }

    // #[test]
    // fn padding() {
    //     let calculate = layout_tree! {
    //         (node(display = Block, padding.top = Px(10.), padding.left = Px(10.))
    //             (node(display = Block, size.height = Px(10.)))
    //         )
    //     };

    //     let results = calculate(Size::new(100., 0.));
    //     assert_eq!(results[0].size, Size::new(100., 20.));
    //     assert_eq!(results[1].size, Size::new(90., 10.));
    // }

    // #[test]
    // #[ignore]
    // fn margin() {
    //     todo!()
    // }
}

test "flex" {
    // #[test]
    // fn flex_row_grow() {
    //     let calculate = layout_tree! {
    //         (node(display = Flex, size.width = Px(300.), size.height = Px(10.))
    //             (node(flex_grow = 1., flex_basis = Px(0.)))
    //             (node(flex_grow = 2., flex_basis = Px(0.)))
    //         )
    //     };

    //     let results = calculate(Size::new(0., 0.));

    //     assert_eq!(results[0].size, Size::new(300., 10.));
    //     assert_eq!(results[1].size, Size::new(100., 10.));
    //     assert_eq!(results[2].size, Size::new(200., 10.));
    // }
}
