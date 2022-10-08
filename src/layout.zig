const std = @import("std");
const Style = @import("style.zig").Style;
const NaN = std.math.nan_f32;
const isNan = std.math.isNan;
const expectFmt = std.testing.expectFmt;

// enums
pub const Display = enum { none, block, flex, @"inline" };
// TODO
// pub const FlexDirection = enum { row, column, row_reverse, column_reverse };
// pub const FlexWrap = enum { no_wrap, wrap, wrap_reverse };
// pub const AlignContent = enum { flex_start, center, flex_end, stretch, space_between, space_around, space_evenly };
// pub const AlignItems = enum { flex_start, center, flex_end, baseline, stretch };
// pub const AlignSelf = enum { auto, flex_start, center, flex_end, baseline, stretch };
// pub const JustifyContent = enum { flex_start, center, flex_end, space_between, space_around, space_evenly };

pub const Dimension = union(enum) {
    auto,
    px: f32,
    percent: f32,

    pub fn resolve(self: Dimension, base: f32) f32 {
        return switch (self) {
            .auto => std.math.nan_f32,
            .px => |v| v,
            .percent => |v| v / 100 * base,
        };
    }
};

pub const LayoutNode = struct {
    style: *const Style,

    // links
    first_child: ?*Self = null,
    next: ?*Self = null,

    // result
    x: f32 = 0,
    y: f32 = 0,
    width: f32 = 0,
    height: f32 = 0,

    // custom extensions
    user_ptr: ?*anyopaque = null,
    user_fn: ?std.meta.FnPtr(fn (*Self) void) = null,

    pub const Self = @This();

    pub fn format(self: *const Self, comptime _: []const u8, opts: std.fmt.FormatOptions, writer: anytype) !void {
        const indent = opts.width orelse 0;

        try writer.writeByteNTimes(' ', indent);
        try writer.print("{s} {d} {d} {d} {d}", .{ @tagName(self.style.display), self.x, self.y, self.width, self.height });

        var next = self.first_child;
        while (next) |ch| : (next = ch.next) {
            try writer.print("\n{:[w]}", .{ .v = ch, .w = indent + 2 });
        }
    }

    pub fn compute(self: *Self, width: f32, height: f32) void {
        self.width = self.style.width.resolve(width);
        self.height = self.style.height.resolve(height);

        switch (self.style.display) {
            .none => {
                self.width = 0;
                self.height = 0;
            },
            .block => self.computeBlock(width, height),
            .flex => self.computeFlex(width, height),
            .@"inline" => self.computeInline(),
        }

        // std.log.debug("{*} {} {d:.2} -> {d:.2}@{d:.2}\n", .{ node, node.style.display, parent_size.width, node.layout.size.width, node.layout.size.height });

        return;
    }

    pub fn computeInline(self: *Self) void {
        self.width = 0;
        self.height = 0;

        if (self.user_fn) |fun| {
            fun(self);
        }
    }

    pub fn computeBlock(self: *Self, width: f32, height: f32) void {
        const s = self.style;

        var y: f32 = s.padding_top.resolve(height);
        var content_height: f32 = 0;

        const inner_w = @maximum(0, width - s.padding_left.resolve(width) - s.padding_right.resolve(width));
        const inner_h = @maximum(0, height - s.padding_top.resolve(height) - s.padding_bottom.resolve(height));

        var next = self.first_child;
        while (next) |ch| : (next = ch.next) {
            ch.compute(inner_w, inner_h);

            // ch.align()?
            ch.x = s.padding_left.resolve(width);
            ch.y = y;

            content_height += ch.height;
            y += ch.height;
        }

        if (std.math.isNan(self.width)) {
            self.width = width;
        }

        if (std.math.isNan(self.height)) {
            self.height = content_height + s.padding_top.resolve(height) + s.padding_bottom.resolve(height);
        }
    }

    pub fn computeFlex(_: *LayoutNode, _: f32, _: f32) void {
        @panic("TODO: flex");
    }
};

test "LayoutNode.format()" {
    var n1 = LayoutNode{ .style = &.{} };
    try expectFmt("block 0 0 0 0", "{}", .{n1});

    var n2 = LayoutNode{ .style = &.{}, .first_child = &n1 };
    try expectFmt(
        \\block 0 0 0 0
        \\  block 0 0 0 0
    , "{}", .{n2});
}

test "display: none" {
    var n1 = LayoutNode{ .style = &.{ .display = .none } };
    n1.compute(0, 0);
    try expectFmt("none 0 0 0 0", "{}", .{n1});
}
