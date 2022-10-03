const std = @import("std");
const nvg = @import("nanovg");

pub const Color = nvg.Color;

pub const Shape = union(enum) {
    rect: Rect,
    rrect: RRect,
};

pub const Rect = struct {
    x: f32 = 0,
    y: f32 = 0,
    w: f32 = 0,
    h: f32 = 0,
};

pub const RRect = struct {
    rect: Rect,
    radii: [4]f32,
};

pub const Canvas = struct {
    vg: nvg,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) anyerror!Self {
        const vg = try nvg.gl.init(allocator, .{
            .antialias = true,
            .stencil_strokes = false,
            .debug = true,
        });

        const font = @embedFile("../../libs/nanovg-zig/examples/Roboto-Regular.ttf");
        _ = vg.createFontMem("sans", font);

        return Self{
            .vg = vg,
        };
    }

    pub fn deinit(self: *Self) void {
        self.vg.deinit();
    }

    pub fn begin(self: *Self, w: f32, h: f32) void {
        self.vg.reset();
        self.vg.beginFrame(w, h, 1.0);

        // white bg
        // TODO: clear()
        // TODO: https://github.com/ziglang/zig/issues/12142#issuecomment-1204416869
        self.fillShape(&Shape{ .rect = .{ .w = w, .h = h } }, nvg.rgb(255, 255, 255));
    }

    pub fn end(self: *Self) void {
        self.vg.endFrame();
    }

    pub fn fillShape(self: *Self, shape: *const Shape, color: Color) void {
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

    pub fn drawText(self: *Self, rect: Rect, text: []const u8) void {
        self.vg.fontFace("sans");
        self.vg.fontSize(16);
        self.vg.fillColor(nvg.rgb(0, 0, 0));
        _ = self.vg.text(rect.x, rect.y + 16, text);
    }
};
