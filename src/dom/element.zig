const std = @import("std");
const Node = @import("node.zig").Node;
const StyleDeclaration = @import("../css/style_declaration.zig").StyleDeclaration;

pub const Element = struct {
    node: Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: StyleDeclaration,

    pub fn getAttribute(self: *Element, name: []const u8) ?[]const u8 {
        return self.attributes.get(name);
    }

    pub fn setAttribute(self: *Element, name: []const u8, value: []const u8) !void {
        try self.attributes.put(name, value);
        self.node.markDirty();
    }

    pub fn removeAttribute(self: *Element, name: []const u8) void {
        self.attributes.remove(name);
        self.node.markDirty();
    }
};
