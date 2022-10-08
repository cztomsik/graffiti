const std = @import("std");
const nvg = @import("nanovg");
const Node = @import("document.zig").Node;
const Document = @import("document.zig").Document;
const Style = @import("style.zig").Style;
const LayoutNode = @import("layout.zig").LayoutNode;

const Color = nvg.Color;
const TRANSPARENT = nvg.rgba(0, 0, 0, 0);

const Shape = union(enum) {
    rect: Rect,
    rrect: RRect,
};

const Rect = struct {
    x: f32 = 0,
    y: f32 = 0,
    w: f32 = 0,
    h: f32 = 0,
};

const RRect = struct {
    rect: Rect,
    radii: [4]f32,
};

pub const Renderer = struct {
    vg: nvg,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) anyerror!Self {
        const vg = try nvg.gl.init(allocator, .{
            .antialias = true,
            .stencil_strokes = false,
            .debug = true,
        });

        const font = @embedFile("../libs/nanovg-zig/examples/Roboto-Regular.ttf");
        _ = vg.createFontMem("sans", font);

        return Self{
            .vg = vg,
        };
    }

    pub fn deinit(self: *Self) void {
        self.vg.deinit();
    }

    pub fn render(self: *Self, document: *Document) void {
        const w = 800;
        const h = 600;

        self.vg.reset();
        self.vg.beginFrame(w, h, 1.0);

        // white bg
        // TODO: clear()
        // TODO: https://github.com/ziglang/zig/issues/12142#issuecomment-1204416869
        self.fillShape(&Shape{ .rect = .{ .w = w, .h = h } }, nvg.rgb(255, 255, 255));

        // TODO: 0 == document.body for now
        const root = document.nodes.at(0);

        root.layout_node.compute(w, h);

        self.renderNode(root);
        self.vg.endFrame();
    }
    fn renderNode(self: *Self, node: *Node) void {
        // std.log.debug("renderNode({any} {*})\n", .{ node.nodeType(), node });

        const ln = &node.layout_node;
        const rect = Rect{ .x = ln.x, .y = ln.y, .w = ln.width, .h = ln.height };

        switch (node.data) {
            .element => |el| self.renderElement(rect, el.style, node.first_child),
            .text => |t| self.renderText(rect, t),
            .document => @panic("TODO"), // |d| self.renderNode(d.root),
        }
    }

    fn renderText(self: *Self, rect: Rect, text: []const u8) void {
        self.vg.fontFace("sans");
        self.vg.fontSize(16);
        self.vg.fillColor(nvg.rgb(0, 0, 0));
        _ = self.vg.text(rect.x, rect.y + 16, text);
    }

    fn renderElement(self: *Self, rect: Rect, style: *const Style, first_child: ?*Node) void {
        // split open/close so we can skip invisibles AND we can also reduce stack usage per each recursion
        // TODO: @call(.{ .modifier = .never_inline }, ...)
        if (self.openElement(rect, style)) {
            self.vg.translate(rect.x, rect.y);

            var next = first_child;
            while (next) |ch| : (next = ch.next_sibling) {
                self.renderNode(ch);
            }

            self.closeElement();
        }
    }

    fn openElement(self: *Self, rect: Rect, style: *const Style) bool {
        // we don't have to save/restore() if we can skip the whole subtree
        if (style.display == .none or style.opacity == 0) {
            return false;
        }

        // restored later
        self.vg.save();

        if (style.opacity != 1.0) {
            const current = self.vg.ctx.getState().alpha;
            self.vg.globalAlpha(current * style.opacity);
        }

        if (rect.w == 0 or rect.h == 0) {
            // skip but recur
            return true;
        }

        // const shape = if (!std.meta.eql(style.border_radius, .{ 0, 0, 0, 0 }))
        //     Shape{ .rrect = .{ .rect = rect, .radii = style.border_radius } }
        // else
        //     Shape{ .rect = rect };
        const shape = Shape{ .rect = rect };

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

    fn closeElement(self: *Self) void {
        // TODO: optional border

        self.vg.restore();
    }

    fn drawBgColor(self: *Self, shape: *const Shape, color: Color) void {
        self.fillShape(shape, color);
    }

    fn fillShape(self: *Self, shape: *const Shape, color: Color) void {
        // std.log.debug("shape {any}\n", .{shape});

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
};

fn textLayout(n: *LayoutNode) void {
    if (n.user_ptr) |ptr| {
        if (@ptrCast(*Node, @alignCast(@alignOf(Node), ptr)).as(.text)) |t| {
            n.width = 10 * @intToFloat(f32, t.len);
            n.height = 40;
        }
    }
}
