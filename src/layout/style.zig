const std = @import("std");

// longhand because it's easier to map from CSS
pub const LayoutStyle = struct {
    display: Display = .block,

    width: Dimension = Dimension.auto,
    height: Dimension = Dimension.auto,
    // min_width: Dimension = Dimension.auto,
    // min_height: Dimension = Dimension.auto,
    // max_width: Dimension = Dimension.auto,
    // max_height: Dimension = Dimension.auto,

    padding_top: Dimension = Dimension.ZERO,
    padding_right: Dimension = Dimension.ZERO,
    padding_bottom: Dimension = Dimension.ZERO,
    padding_left: Dimension = Dimension.ZERO,

    // margin_top: Dimension = Dimension.ZERO,
    // margin_right: Dimension = Dimension.ZERO,
    // margin_bottom: Dimension = Dimension.ZERO,
    // margin_left: Dimension = Dimension.ZERO,

    // border_top: Dimension = Dimension.ZERO,
    // border_right: Dimension = Dimension.ZERO,
    // border_bottom: Dimension = Dimension.ZERO,
    // border_left: Dimension = Dimension.ZERO,

    // TODO: position, top, right, left

    flex_grow: f32 = 0,
    flex_shrink: f32 = 1,
    flex_basis: Dimension = Dimension.auto,

    flex_direction: FlexDirection = .row,
    flex_wrap: FlexWrap = .no_wrap,

    align_content: AlignContent = .stretch,
    align_items: AlignItems = .stretch,
    align_self: AlignSelf = .auto,
    justify_content: JustifyContent = .flex_start,
};

pub const Display = enum {
    none,
    block,
    flex,
    @"inline",
};

pub const FlexDirection = enum {
    row,
    column,
    row_reverse,
    column_reverse,
};

pub const FlexWrap = enum {
    no_wrap,
    wrap,
    wrap_reverse,
};

pub const AlignContent = enum {
    flex_start,
    center,
    flex_end,
    stretch,
    space_between,
    space_around,
    space_evenly,
};

pub const AlignItems = enum {
    flex_start,
    center,
    flex_end,
    baseline,
    stretch,
};

pub const AlignSelf = enum {
    auto,
    flex_start,
    center,
    flex_end,
    baseline,
    stretch,
};

pub const JustifyContent = enum {
    flex_start,
    center,
    flex_end,
    space_between,
    space_around,
    space_evenly,
};

pub const Dimension = union(enum) {
    auto,
    px: f32,
    percent: f32,

    const Self = @This();

    pub const ZERO = Self{ .px = 0 };

    pub fn resolve(self: Self, base: f32) f32 {
        return switch (self) {
            .auto => std.math.nan_f32,
            .px => |v| v,
            .percent => |v| v / 100 * base,
        };
    }
};
