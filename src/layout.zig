// TODO: this is just a PoC, it sort-of works but there are no tests
//       and I realy need to figure that out somehow before proceeding any further
//       (min/max, absolute/relative, wrap, reverse, ordering, ...)

const std = @import("std");
const Style = @import("style.zig").Style;
const isNan = std.math.isNan;
const expectFmt = std.testing.expectFmt;

// enums
pub const Display = enum { none, flex };
pub const FlexDirection = enum { row, column }; // , row_reverse, column_reverse }; // TODO
pub const FlexWrap = enum { no_wrap, wrap, wrap_reverse }; // TODO
pub const AlignContent = enum { flex_start, center, flex_end, stretch, space_between, space_around, space_evenly }; // TODO
pub const AlignItems = enum { flex_start, center, flex_end, baseline, stretch }; // TODO
pub const AlignSelf = enum { auto, flex_start, center, flex_end, baseline, stretch }; // TODO
pub const JustifyContent = enum { flex_start, center, flex_end, space_between, space_around, space_evenly }; // TODO
pub const Position = enum { static, absolute, relative }; // TODO

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
    const Self = @This();

    first_child: ?*Self = null,
    next_sibling: ?*Self = null,
    style: *Style,
    text: ?[]const u8 = null, // TODO: Text
    pos: [2]f32 = .{ 0, 0 },
    size: [2]f32 = .{ 0, 0 },

    pub fn compute(self: *LayoutNode, size: [2]f32) void {
        // std.debug.print("-------\n", .{});
        self.size = size;
        computeNode(self, &Style{}, size);
    }
};

fn computeNode(node: *LayoutNode, style: *const Style, size: [2]f32) void {
    // std.debug.print("{} {s} {d} {d}\n", .{ node.id, @tagName(node.data), node.size[0], node.size[1] });

    const is_row = style.flex_direction == .row; // or style.flex_direction == .row_reverse;
    const main: u1 = if (is_row) 0 else 1;
    const cross: u1 = ~main;

    var flex_space = node.size[main];
    var grows: f32 = 0;
    var shrinks: f32 = 0;

    var next = node.first_child;
    while (next) |ch| : (next = ch.next_sibling) {
        if (ch.text) |t| {
            ch.size = .{ 10 * @intToFloat(f32, t.len), 40 };
        } else {
            grows += ch.style.flex.grow;
            shrinks += ch.style.flex.shrink;

            ch.size[0] = ch.style.width.resolve(size[0]);
            ch.size[1] = ch.style.height.resolve(size[1]);
            if (isNan(ch.size[main])) ch.size[main] = 0;
            if (isNan(ch.size[cross])) ch.size[cross] = size[cross]; // TODO: - margin[w/h]
            const basis = ch.style.flex.basis.resolve(size[main]);
            if (!isNan(basis)) ch.size[main] = basis;

            // TODO: skip if we can, but items should not directly cause overflow (text or child-child with given size)
            computeNode(ch, ch.style, ch.size);
        }

        node.size[cross] = @max(node.size[cross], ch.size[cross]);
        flex_space -= @max(0, ch.size[main]);
    }

    node.size[main] = @max(node.size[main], -flex_space);

    var pos: [2]f32 = .{
        @max(0, style.padding.left.resolve(size[0])),
        @max(0, style.padding.top.resolve(size[1])),
    };

    // grow/shrink, position, reverse, align, stretch, margin, ...
    next = node.first_child;
    while (next) |ch| : (next = ch.next_sibling) {
        ch.pos = pos;

        if (flex_space > 0 and ch.style.flex.grow > 0) {
            ch.size[main] += (flex_space / grows) * ch.style.flex.grow;
        }

        if (flex_space < 0 and ch.style.flex.shrink > 0) {
            ch.size[main] += (flex_space / shrinks) * ch.style.flex.shrink;
        }

        // ch.pos[main] += @max(0, ch.style.margin_left/top.resolve())
        // pos[main] += @max(0, ch.style.margin_x/y.resolve())

        // TODO: align

        computeNode(ch, ch.style, ch.size);

        // advance
        pos[main] += ch.size[main];
    }
}
