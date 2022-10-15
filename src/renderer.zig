const std = @import("std");
const nvg = @import("nanovg");
const Node = @import("document.zig").Node;
const Document = @import("document.zig").Document;
const Style = @import("style.zig").Style;
const layout = @import("layout.zig").layout;

const Color = nvg.Color;
const TRANSPARENT = nvg.rgba(0, 0, 0, 0);

const Shape = struct {
    rect: Rect,
    radius: f32 = 0,
};

const Rect = packed struct {
    x: f32 = 0,
    y: f32 = 0,
    w: f32 = 0,
    h: f32 = 0,
};

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

        const font = @embedFile("../libs/nanovg-zig/examples/Roboto-Regular.ttf");
        _ = vg.createFontMem("sans", font);

        return Self{
            .allocator = allocator,
            .vg = vg,
        };
    }

    pub fn deinit(self: *Self) void {
        self.vg.deinit();
    }

    pub fn render(self: *Self, document: *Document, size: [2]f32, scale: [2]f32) void {
        self.vg.reset();
        self.vg.beginFrame(size[0], size[1], scale[0]);

        // white bg
        // TODO: clear()
        // TODO: https://github.com/ziglang/zig/issues/12142#issuecomment-1204416869
        self.fillShape(&Shape{ .rect = .{ .w = size[0], .h = size[1] } }, .{ .color = nvg.rgb(255, 255, 255) });

        // TODO: 0 == document.body for now
        const root = document.nodes.at(0);

        // TODO: 0 == document.body for now
        layout(document.nodes.at(0), size);

        self.renderNode(root);
        self.vg.endFrame();
    }

    fn renderNode(self: *Self, node: *Node) void {
        // std.log.debug("renderNode({any} {*})\n", .{ node.nodeType(), node });

        const rect = @bitCast(Rect, [2][2]f32{ node.pos, node.size });

        switch (node.data) {
            .element => |el| self.renderElement(rect, &el.style.data, node.first_child),
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

        const shape = Shape{ .rect = rect, .radius = style.border_radius };

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
        self.fillShape(shape, .{ .color = color });
    }

    fn fillShape(self: *Self, shape: *const Shape, fill: union(enum) { color: nvg.Color, paint: nvg.Paint }) void {
        // std.log.debug("shape {any}\n", .{shape});

        self.vg.beginPath();

        if (shape.radius != 0) {
            self.vg.roundedRect(shape.rect.x, shape.rect.y, shape.rect.w, shape.rect.h, shape.radius);
        } else {
            self.vg.rect(shape.rect.x, shape.rect.y, shape.rect.w, shape.rect.h);
        }

        switch (fill) {
            .color => |c| self.vg.fillColor(c),
            .paint => |p| self.vg.fillPaint(p),
        }

        self.vg.fill();
    }
};
