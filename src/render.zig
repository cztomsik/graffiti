const std = @import("std");
const Canvas = @import("gfx/canvas.zig").Canvas;
const Shape = @import("gfx/canvas.zig").Shape;
const Rect = @import("gfx/canvas.zig").Rect;
const Node = @import("document.zig").Node;
const Document = @import("document.zig").Document;
const NodeId = @import("document.zig").NodeId;
const TRANSPARENT = @import("style.zig").TRANSPARENT;
const LayoutNode = @import("layout/layout.zig").LayoutNode;
const calculate = @import("layout/layout.zig").calculate;

pub fn render(allocator: std.mem.Allocator, canvas: *Canvas, document: *Document) void {
    // layout-all
    var layout_nodes = allocator.alloc(LayoutNode, document.nodes.list.len) catch @panic("oom");
    defer allocator.free(layout_nodes);
    for (layout_nodes) |*l, n| {
        const node = document.nodeById(n);

        l.* = switch (node.nodeType()) {
            .element => .{
                .style = .{},
                .user_ptr = node,
            },
            .text => .{
                .style = .{ .display = .@"inline" },
                .user_fn = Self.textLayout,
                .user_ptr = node,
            },
            .document => .{
                .style = .{},
                .user_ptr = node,
            },
            .comment, .document_fragment => .{
                .style = .{},
                .user_ptr = node,
            },
        };

        l.first_child = if (node.first_child) |ch| &layout_nodes[ch.id] else null;
        l.next = if (node.next_sibling) |ne| &layout_nodes[ne.id] else null;
    }
    layout_nodes[root.id].compute(800, 600);

    var ctx = RenderContext{ .canvas = canvas };

    // TODO: maybe we don't need document here anymore and we could render layout tree
    ctx.renderNode(&layout_nodes[document.node.id]);
}

fn textLayout(n: *LayoutNode) void {
    if (n.user_ptr) |ptr| {
        if (@ptrCast(*Node, ptr).text()) |t| {
            n.width = 10 * @intToFloat(f32, t.data.len);
            n.height = 40;
        }
    }
}

const RenderContext = struct {
    canvas: *Canvas,

    const Self = @This();

    fn renderNode(self: *Self, node: *LayoutNode) void {
        const rect = Rect{ .x = node.x, .y = node.y, .w = node.width, .h = node.height };
        const dom_node = @ptrCast(*Node, node.user_ptr.?);

        if (dom_node.as(.text)) |t| {
            self.canvas.drawText(rect, t.data);
        } else {
            self.drawContainer(rect, &.{}, node.first_child);
        }
    }

    fn drawContainer(self: *Self, rect: Rect, style: *const Style, first_child: ?*LayoutNode) void {
        // split open/close so we can skip invisibles AND we can also reduce stack usage per each recursion
        // TODO: @call(.{ .modifier = .never_inline }, ...)
        if (self.openContainer(rect, style)) {
            self.canvas.vg.translate(rect.x, rect.y);

            var next = first_child;
            while (next) |ch| : (next = ch.next) {
                self.renderNode(ch);
            }

            self.closeContainer();
        }
    }

    fn openContainer(self: *Self, rect: Rect, style: *const Style) bool {
        // we don't have to save/restore() if we can skip the whole subtree
        if (style.opacity == 0) {
            return false;
        }

        // restored later
        self.canvas.vg.save();

        if (rect.w == 0 or rect.h == 0) {
            // skip but recur
            return true;
        }

        if (style.opacity != 1.0) {
            const current = self.canvas.vg.ctx.getState().alpha;
            self.canvas.vg.globalAlpha(current * style.opacity);
        }

        const shape = if (!std.meta.eql(style.border_radius, .{ 0, 0, 0, 0 }))
            Shape{ .rrect = .{ .rect = rect, .radii = style.border_radius } }
        else
            Shape{ .rect = rect };

        // if let Some(matrix) = &style.transform {
        //     self.canvas.concat(matrix);
        // }

        // if (style.shadow) |shadow| {
        //     self.drawShadow(&shape, shadow);
        // }

        // if (style.outline) |outline| {
        //     self.drawOutline(&shape, outline);
        // }

        // if style.clip {
        //     self.clipShape(&shape, ClipOp::Intersect, true /*style.transform.is_some()*/);
        // }

        if (!std.meta.eql(style.background_color, TRANSPARENT)) {
            self.drawBgColor(&shape, style.background_color);
        }

        // TODO: image(s)

        // TODO: scroll
        // self.vg.translate(dx, dy);

        return true;
    }

    fn closeContainer(self: *Self) void {
        // TODO: optional border

        self.canvas.vg.restore();
    }

    fn drawBgColor(self: *Self, shape: *const Shape, color: Color) void {
        self.canvas.fillShape(shape, color);
    }
};
