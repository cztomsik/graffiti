const std = @import("std");
const NaN = std.math.nan_f32;
const isNan = std.math.isNan;
const LayoutNode = @import("layout.zig").LayoutNode;

pub fn computeBlock(node: *LayoutNode, width: f32, height: f32) void {
    const s = &node.style;

    var y: f32 = s.padding_top.resolve(height);
    var content_height: f32 = 0;

    const inner_w = @maximum(0, width - s.padding_left.resolve(width) - s.padding_right.resolve(width));
    const inner_h = @maximum(0, height - s.padding_top.resolve(height) - s.padding_bottom.resolve(height));

    var next = node.first_child;
    while (next) |ch| : (next = ch.next) {
        ch.compute(inner_w, inner_h);

        // ch.align()?
        ch.x = s.padding_left.resolve(width);
        ch.y = y;

        content_height += ch.height;
        y += ch.height;
    }

    if (std.math.isNan(node.width)) {
        node.width = width;
    }

    if (std.math.isNan(node.height)) {
        node.height = content_height + s.padding_top.resolve(height) + s.padding_bottom.resolve(height);
    }
}
