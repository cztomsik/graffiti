const std = @import("std");
const Style = @import("style.zig").Style;
const Dimension = @import("style.zig").Dimension;

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

    fn resolve(_: *Self, val: Dimension) f32 {
        return switch (val) {
            .auto => std.math.nan_f32,
            .px => |px| px,
        };
    }

    fn compute_node(self: *Self, node: *LayoutNode, parent_size: Size) Size {
        node.layout.size = .{ .width = self.resolve(node.style.width), .height = self.resolve(node.style.height) };

        switch (node.style.display) {
            .block => self.compute_block(node, parent_size),
            .@"inline" => self.compute_inline(node),
            else => {},
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
};
