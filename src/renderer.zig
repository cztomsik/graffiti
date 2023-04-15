// TODO: rendering should be async (send a frame with display items to the render thread)
//       all display items should be cached (not just text blobs)
//       -> Style size is not important because it should not be in hot path

const std = @import("std");
const nvg = @import("nanovg");

const Node = @import("dom/node.zig").Node;
const Element = @import("dom/element.zig").Element;
const CharacterData = @import("dom/character_data.zig").CharacterData;
const Document = @import("dom/document.zig").Document;
const Style = @import("style.zig").Style;
const Color = @import("style.zig").Color;
const Shadow = @import("style.zig").Shadow;
const OutlineStyle = @import("style.zig").OutlineStyle;
const BackgroundImage = @import("style.zig").BackgroundImage;
const TRANSPARENT = @import("style.zig").TRANSPARENT;

const Shape = struct {
    rect: [4]f32,
    radii: [4]f32 = .{ 0, 0, 0, 0 },
};

pub const Renderer = struct {
    allocator: std.mem.Allocator,
    vg: nvg,
    font: nvg.Font,

    pub fn init(allocator: std.mem.Allocator) anyerror!Renderer {
        var vg = try nvg.gl.init(allocator, .{
            .antialias = true,
            .stencil_strokes = false,
            .debug = true,
        });

        return .{
            .allocator = allocator,
            .vg = vg,
            .font = vg.createFontMem("sans", @embedFile("../libs/nanovg-zig/examples/Roboto-Regular.ttf")),
        };
    }

    pub fn deinit(self: *Renderer) void {
        self.vg.deinit();
    }

    pub fn render(self: *Renderer, document: *Document, size: [2]f32, scale: [2]f32) void {
        self.vg.beginFrame(size[0], size[1], scale[0]);
        defer self.vg.endFrame();

        // white bg
        self.drawShape(&Shape{ .rect = .{ 0, 0, size[0], size[1] } });
        self.vg.fillColor(nvg.rgb(255, 255, 255));
        self.vg.fill();

        // render
        self.renderNode(document.node.first_child orelse return);
    }

    noinline fn renderNode(self: *Renderer, node: *Node) void {
        switch (node.node_type) {
            .element => self.renderElement(node.cast(Element)),
            .text => self.renderText(node.cast(CharacterData)),
            else => return,
        }
    }

    fn renderText(self: *Renderer, text: *CharacterData) void {
        self.vg.fontFace("sans");
        self.vg.fontSize(16);
        self.vg.fillColor(nvg.rgb(0, 0, 0));
        _ = self.vg.text(text.node.pos[0], text.node.pos[1] + 16, text.data);
    }

    fn renderElement(self: *Renderer, element: *Element) void {
        // split open/close so we can skip invisibles AND we can also reduce the stack usage per each recursion
        if (self.openElement(element)) {
            self.vg.translate(element.node.pos[0], element.node.pos[1]);

            var iter = element.node.childNodes();
            while (iter.next()) |ch| {
                self.renderNode(ch);
            }

            self.closeElement();
        }
    }

    fn openElement(self: *Renderer, element: *Element) bool {
        const style = &element.resolved_style;

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

        if (element.node.size[0] == 0 and element.node.size[1] == 0) {
            // skip but recur
            return true;
        }

        const width = element.node.size[0];
        const shape = Shape{
            .rect = .{
                element.node.pos[0],
                element.node.pos[1],
                element.node.size[0],
                element.node.size[1],
            },
            .radii = .{
                style.border_top_left_radius.resolve(width),
                style.border_top_right_radius.resolve(width),
                style.border_bottom_right_radius.resolve(width),
                style.border_bottom_left_radius.resolve(width),
            },
        };

        // if let Some(matrix) = &style.transform {
        //     self.canvas.concat(matrix);
        // }

        // TODO: for (style.box_shadow) |*s| {
        if (style.box_shadow) |*s| {
            self.drawShadow(
                &shape,
                s.x.resolve(width),
                s.y.resolve(width),
                s.blur.resolve(width),
                s.spread.resolve(width),
                s.color,
            );
        }

        if (style.outline_style != .none and !std.meta.eql(style.outline_color, TRANSPARENT)) {
            const outline_width = style.outline_width.resolve(shape.rect[2]);
            if (width > 0) {
                self.drawOutline(&shape, outline_width, style.outline_color);
            }
        }

        // if style.clip {
        //     self.clipShape(&shape, ClipOp::Intersect, true /*style.transform.is_some()*/);
        // }

        if (!std.meta.eql(style.background_color, TRANSPARENT)) {
            self.drawBgColor(&shape, style.background_color);
        }

        // for (style.background_image) |*img| {
        //     self.drawBackgroundImage(img);
        // }

        // TODO: scroll
        // self.vg.translate(dx, dy);

        return true;
    }

    fn closeElement(self: *Renderer) void {
        // TODO: optional border

        self.vg.restore();
    }

    fn drawBgColor(self: *Renderer, shape: *const Shape, color: Color) void {
        self.drawShape(shape);
        self.vg.fillColor(nvgColor(color));
        self.vg.fill();
    }

    fn drawBackgroundImage(_: *Renderer, _: *const BackgroundImage) void {
        std.debug.print("TODO: drawBackgroundImage", .{});
    }

    fn drawOutline(self: *Renderer, shape: *const Shape, width: f32, color: Color) void {
        self.drawShape(shape);

        self.vg.strokeWidth(width);
        self.vg.strokeColor(nvgColor(color));
        self.vg.stroke();
    }

    // TODO: something here is wrong... good luck, future me :-D
    fn drawShadow(self: *Renderer, shape: *const Shape, x: f32, y: f32, blur: f32, spread: f32, color: Color) void {
        _ = spread;
        _ = y;
        _ = x;

        const rect = shape.rect;
        var out_rect: [4]f32 = .{
            rect[0] - blur,
            rect[1] - blur,
            rect[2] + 2 * blur,
            rect[3] + 2 * blur,
        };
        var col = nvgColor(color);
        // col.a *= rect.w / out_rect.w;
        const paint = self.vg.boxGradient(rect[0], rect[1], rect[2], rect[3], shape.radii[0], blur, col, nvgColor(TRANSPARENT));
        // TODO(later): shape.winding?
        // self.vg.pathWinding(nvg.Winding.solidity(.hole));
        self.drawShape(&Shape{ .rect = out_rect });
        self.vg.fillPaint(paint);
        self.vg.fill();
    }

    fn drawShape(self: *Renderer, shape: *const Shape) void {
        self.vg.beginPath();

        if (std.meta.eql(shape.radii, .{ 0, 0, 0, 0 })) {
            self.vg.rect(shape.rect[0], shape.rect[1], shape.rect[2], shape.rect[3]);
        } else if (shape.radii[0] == shape.radii[1] and shape.radii[0] == shape.radii[2] and shape.radii[0] == shape.radii[3]) {
            self.vg.roundedRect(shape.rect[0], shape.rect[1], shape.rect[2], shape.rect[3], shape.radii[0]);
        } else {
            self.vg.roundedRectVarying(
                shape.rect[0],
                shape.rect[1],
                shape.rect[2],
                shape.rect[3],
                shape.radii[0],
                shape.radii[1],
                shape.radii[2],
                shape.radii[3],
            );
        }
    }
};

fn nvgColor(color: Color) nvg.Color {
    return nvg.rgba(color.r, color.g, color.b, color.a);
}
