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

// test "fixed width & height" {
//     var node = LayoutNode{ .display = .block, .width = 10, .height = 10 };
//     node.calculate(0, 0);
//     expectLayout(
//         \\ block 0 0 10 10
//         ,
//     );
// }

// test "fixed height" {
//     var node = LayoutNode{ .display = .block, .height = 10 };

//     node.calculate(0, 10);
//     expectLayout(
//         \\ block 0 0 0 10
//         ,
//     );

//     node.calculate(10, 0);
//     expectLayout(
//         \\ block 0 0 10 10
//         ,
//     );
// }

// test "content height" {
//     // content height
//     var root = LayoutNode{ .display = .block };
//     var ch1 = LayoutNode{ .display = .block, .width = 10, .height = 10 };
//     var ch1 = LayoutNode{ .display = .block, .height = 10 };
//     root.appendChild(ch1);
//     root.appendChild(ch2);

//     root.compute(100, 0);
//     expectRect(root, 0, 0, 100, 20);
//     expectRect(ch1, 0, 0, 10, 10);
//     expectRect(ch2, 0, 10, 100, 10);
// }

// test "padding" {
//     var root = LayoutNode{ .display = .block, .padding_top = 10, .padding_left = 10 };
//     var ch1 = LayoutNode{ .display = .block, .height = 10 };
//     root.appendChild(ch1);

//     root.compute(100, 0);
//     expectRect(root, 0, 0, 100, 20);
//     expectRect(ch1, 10, 10, 90, 10);
// }
