const std = @import("std");
const nvg = @import("nanovg");
const Document = @import("document.zig").Document;
const NodeId = @import("document.zig").NodeId;
const Style = @import("style.zig").Style;
const Color = @import("style.zig").Color;
const TRANSPARENT = @import("style.zig").TRANSPARENT;
const LayoutNode = @import("layout.zig").LayoutNode;
const calculate = @import("layout.zig").calculate;

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

    pub fn render(self: *Self, document: *const Document) void {
        self.vg.reset();
        self.vg.beginFrame(800, 600, 1.0);
        defer self.vg.endFrame();

        // layout-all
        var layout_nodes = self.allocator.alloc(LayoutNode, document.nodes.items.len) catch @panic("oom");
        defer self.allocator.free(layout_nodes);
        for (layout_nodes) |*l, n| l.* = switch (document.node_type(n)) {
            .element => .{ .style = document.element_style(n).*, .children = document.children(n) },
            .text => .{ .style = .{ .display = .@"inline" }, .text = document.text(n) },
            .document => .{ .style = .{}, .children = document.children(n) },
        };
        calculate(layout_nodes, Document.ROOT, .{ .width = 800, .height = 600 });

        var ctx = RenderContext{ .document = document, .layout_nodes = layout_nodes, .vg = &self.vg };

        // white bg
        ctx.fillShape(&.{ .rect = .{ .w = 800, .h = 600 } }, nvg.rgb(255, 255, 255));

        ctx.renderNode(Document.ROOT);
    }
};

const RenderContext = struct {
    document: *const Document,
    layout_nodes: []LayoutNode,
    vg: *nvg,

    const Self = @This();

    fn renderNode(self: *Self, node: NodeId) void {
        std.debug.print("renderNode {}\n", .{node});

        const l = &self.layout_nodes[node].layout;
        var rect = Rect{ .x = l.pos.x, .y = l.pos.y, .w = l.size.width, .h = l.size.height };

        switch (self.document.node_type(node)) {
            .element => self.drawContainer(rect, self.document.element_style(node), self.document.children(node)),
            .text => self.drawText(rect, self.document.text(node)),
            .document => self.drawContainer(rect, &.{}, self.document.children(node)),
        }
    }

    fn drawContainer(self: *Self, rect: Rect, style: *const Style, children: []const NodeId) void {
        // split open/close so we can skip invisibles AND we can also reduce stack usage per each recursion
        // TODO: @call(.{ .modifier = .never_inline }, ...)
        if (self.openContainer(rect, style)) {
            self.vg.translate(rect.x, rect.y);

            for (children) |ch| {
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
        std.debug.print("shape {any}", .{shape});
        self.fillShape(shape, color);
    }

    fn fillShape(self: *Self, shape: *const Shape, color: Color) void {
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
