const std = @import("std");
const nvg = @import("nanovg");
const Document = @import("dom/dom.zig").Document;
const NodeId = @import("dom/dom.zig").NodeId;
const Style = @import("style.zig").Style;
const Color = @import("style.zig").Color;
const TRANSPARENT = @import("style.zig").TRANSPARENT;
const LayoutNode = @import("layout/layout.zig").LayoutNode;
const calculate = @import("layout/layout.zig").calculate;

const Shape = union(enum) { rect: Rect, rrect: RRect };

const Rect = struct { x: f32 = 0, y: f32 = 0, w: f32 = 0, h: f32 = 0 };

const RRect = struct { rect: Rect, radii: [4]f32 };

pub const Renderer = struct {
    allocator: std.mem.Allocator,
    vg: nvg,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) anyerror!Self {
        const vg = try nvg.gl.init(allocator, .{
            .antialias = true,
            .stencil_strokes = false,
            .debug = true,
        });

        const font = @embedFile("../nanovg-zig/examples/Roboto-Regular.ttf");
        _ = vg.createFontMem("sans", font);

        return Self{ .allocator = allocator, .vg = vg };
    }

    pub fn deinit(self: *Self) void {
        self.vg.deinit();
    }

    pub fn render(self: *Self, document: *Document, w: f32, h: f32) void {
        std.debug.print("TODO: style is empty for now\n", .{});

        self.vg.reset();
        self.vg.beginFrame(w, h, 1.0);
        defer self.vg.endFrame();

        // layout-all
        var layout_nodes = self.allocator.alloc(LayoutNode, document.nodes.list.len) catch @panic("oom");
        defer self.allocator.free(layout_nodes);
        for (layout_nodes) |*l, n| {
            const node = document.nodeById(n);

            l.* = switch (node.nodeType()) {
                .element => .{ .style = .{} },
                .text => .{ .style = .{ .display = .@"inline" }, .text = node.text().?.data },
                .document => .{ .style = .{} },
                .comment, .document_fragment => .{ .style = .{} },
            };

            l.first_child = if (node.first_child) |ch| &layout_nodes[ch.id] else null;
            l.next = if (node.next_sibling) |ne| &layout_nodes[ne.id] else null;
        }
        calculate(&layout_nodes[document.node.id], .{ .width = w, .height = h });

        var ctx = RenderContext{ .vg = &self.vg };

        // white bg
        // TODO: https://github.com/ziglang/zig/issues/12142#issuecomment-1204416869
        ctx.fillShape(&Shape{ .rect = .{ .w = w, .h = h } }, nvg.rgb(255, 255, 255));

        // TODO: maybe we don't need document here anymore and we could render layout tree
        ctx.renderNode(&layout_nodes[document.node.id]);
    }
};

const RenderContext = struct {
    vg: *nvg,

    const Self = @This();

    fn renderNode(self: *Self, node: *LayoutNode) void {
        const l = &node.layout;
        var rect = Rect{ .x = l.pos.x, .y = l.pos.y, .w = l.size.width, .h = l.size.height };

        if (node.text) |t| self.drawText(rect, t) else self.drawContainer(rect, &node.style, node.first_child);
    }

    fn drawContainer(self: *Self, rect: Rect, style: *const Style, first_child: ?*LayoutNode) void {
        // split open/close so we can skip invisibles AND we can also reduce stack usage per each recursion
        // TODO: @call(.{ .modifier = .never_inline }, ...)
        if (self.openContainer(rect, style)) {
            self.vg.translate(rect.x, rect.y);

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
        self.vg.save();

        if (rect.w == 0 or rect.h == 0) {
            // skip but recur
            return true;
        }

        if (style.opacity != 1.0) {
            const current = self.vg.ctx.getState().alpha;
            self.vg.globalAlpha(current * style.opacity);
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

        self.vg.restore();
    }

    fn drawBgColor(self: *Self, shape: *const Shape, color: Color) void {
        self.fillShape(shape, color);
    }

    fn fillShape(self: *Self, shape: *const Shape, color: Color) void {
        // std.debug.print("shape {any}\n", .{shape});

        self.vg.beginPath();

        switch (shape.*) {
            .rect => |rect| self.vg.rect(rect.x, rect.y, rect.w, rect.h),
            .rrect => |rrect| {
                const rect = &rrect.rect;
                // TODO: if (radii[0] == radii[1] ...) else ...
                self.vg.roundedRect(rect.x, rect.y, rect.w, rect.h, rrect.radii[0]);
            },
        }

        self.vg.fillColor(color);
        self.vg.fill();
    }

    fn drawText(self: *Self, rect: Rect, text: []const u8) void {
        self.vg.fontFace("sans");
        self.vg.fontSize(16);
        self.vg.fillColor(nvg.rgb(0, 0, 0));
        _ = self.vg.text(rect.x, rect.y + 16, text);
    }
};
