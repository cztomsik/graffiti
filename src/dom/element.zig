const std = @import("std");
const Node = @import("node.zig").Node;
const Document = @import("document.zig").Document;
const StyleDeclaration = @import("../css/style_declaration.zig").StyleDeclaration;
const Style = @import("../style.zig").Style;

pub const Element = struct {
    node: Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: StyleDeclaration,
    resolved_style: Style,

    pub fn init(document: *Document, local_name: []const u8) !Element {
        return .{
            .node = .{ .document = document, .node_type = .element },
            .local_name = try document.allocator.dupe(u8, local_name),
            .attributes = std.BufMap.init(document.allocator),
            .style = StyleDeclaration.init(document.allocator),
            .resolved_style = .{},
        };
    }

    pub fn deinit(self: *Element) void {
        self.node.document.allocator.free(self.local_name);
        self.attributes.deinit();
        self.style.deinit();
    }

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
