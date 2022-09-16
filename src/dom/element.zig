const std = @import("std");
const Node = @import("dom.zig").Node;
const Style = @import("../style.zig").Style;

pub const Element = struct {
    node: *Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: ?*Style = null,

    const Self = @This();

    // pub fn matches(self: *Self, selector: ?) bool {}

    pub fn id(self: *Self) []const u8 {
        return self.getAttribute("id") orelse "";
    }

    pub fn className(self: *Self) []const u8 {
        return self.getAttribute("class") orelse "";
    }

    // pub fn classList(self: *Self) ClassList {}

    pub fn hasAttribute(self: *Self, att: []const u8) bool {
        return self.attributes.get(att) != null;
    }

    pub fn getAttribute(self: *Self, att: []const u8) ?[]const u8 {
        if (std.mem.eql(u8, att, "style")) {
            std.debug.print("TODO: arena-allocated .style.cssText()\n", .{});
        }

        return self.attributes.get(att);
    }

    pub fn setAttribute(self: *Self, att: []const u8, val: []const u8) !void {
        if (std.mem.eql(u8, att, "style")) {
            std.debug.print("TODO: parse & set style {s}\n", .{val});
        }

        try self.attributes.put(att, val);
    }

    pub fn removeAttribute(self: *Self, att: []const u8) void {
        self.attributes.remove(att);
    }
};
