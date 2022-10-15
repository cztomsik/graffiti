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
        // std.debug.print("renderNode({} {s} {d} {d} {d} {d})\n", .{ node.id, @tagName(node.data), node.pos[0], node.pos[1], node.size[0], node.size[1] });

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

        if (style.box_shadow) |shadow| {
            self.drawShadow(&shape, shadow);
        }

        if (style.outline) |outline| {
            self.drawOutline(&shape, outline);
        }

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

    fn drawOutline(self: *Self, shape: *const Shape, outline: Style.Outline) void {
        self.vg.beginPath();
        self.vg.rect(shape.rect.x, shape.rect.y, shape.rect.w, shape.rect.h);
        self.vg.strokeWidth(outline.width);
        self.vg.strokeColor(outline.color);
        self.vg.stroke();
    }

    // TODO: something here is wrong... good luck, future me :-D
    fn drawShadow(self: *Self, shape: *const Shape, shadow: Style.BoxShadow) void {
        const rect = shape.rect;
        var out_rect = Rect{
            .x = rect.x - shadow.blur,
            .y = rect.y - shadow.blur,
            .w = rect.w + 2 * shadow.blur,
            .h = rect.h + 2 * shadow.blur,
        };
        var col = shadow.color;
        //col.a *= rect.w / out_rect.w;
        const paint = self.vg.boxGradient(rect.x, rect.y, rect.w, rect.h, shape.radius, shadow.blur, col, TRANSPARENT);
        // TODO(later): shape.winding?
        // self.vg.pathWinding(nvg.Winding.solidity(.hole));
        self.fillShape(&Shape{ .rect = out_rect }, .{ .paint = paint });
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
