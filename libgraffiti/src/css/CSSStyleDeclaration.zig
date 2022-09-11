const std = @import("std");

pub const CSSStyleDeclaration = struct {
    const Self = @This();

    // pub fn length(self: *Self) usize {}

    // pub fn item(self: *Self, index: usize) {}

    // pub fn cssText(self: *Self) []const u8 {}

    pub fn setCssText(self: *Self, css_text: []const u8) void {
        std.debug.print("TODO: parse {any} {s}\n", .{ self, css_text });
    }

    // pub fn getPropertyValue(self: *Self, prop: []const u8) {}

    // pub fn getPropertyPriority(self: *Self, prop: []const u8) {}

    // pub fn setProperty(self: *Self, prop: []const u8, val: []const u8, important: bool) {}

    // pub fn removeProperty(self: *Self, prop: []const u8) {}
};
