const std = @import("std");
const Style = @import("style.zig").Style;

pub const NodeId = usize;

const NodeType = enum(u32) {
    element = 1,
    text = 3,
    document = 9,
};

const Node = struct { data: NodeData, children: std.ArrayList(NodeId), style: Style = .{} };

const NodeData = union(NodeType) { document, text: []const u8, element: Element };

const Element = struct {
    local_name: []const u8,
};

const Error = error{InvalidNodeType};

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: std.ArrayList(Node),

    const Self = @This();

    pub const ROOT = 0;

    pub fn init(allocator: std.mem.Allocator) !Self {
        var doc = Self{ .allocator = allocator, .nodes = std.ArrayList(Node).init(allocator) };
        _ = try doc.createNode(NodeData.document);

        return doc;
    }

    pub fn deinit(self: *Self) void {
        self.nodes.deinit();
    }

    pub fn createTextNode(self: *Self, data: []const u8) !NodeId {
        return self.createNode(NodeData{ .text = data });
    }

    pub fn text(self: *const Self, text_node: NodeId) []const u8 {
        return switch (self.nodes.items[text_node].data) {
            .text => |text| text,
            else => @panic("invalid node type"), //Error.InvalidNodeType,
        };
    }

    pub fn createElement(self: *Self, local_name: []const u8) !NodeId {
        return self.createNode(NodeData{ .element = .{ .local_name = local_name } });
    }

    pub fn element_style(self: *const Self, element: NodeId) *const Style {
        return &self.nodes.items[element].style;
    }

    pub fn set_element_style(self: *Self, element: NodeId, style: Style) void {
        self.nodes.items[element].style = style;
    }

    pub fn children(self: *const Self, node: NodeId) []const NodeId {
        return self.nodes.items[node].children.items;
    }

    pub fn appendChild(self: *Self, parent: NodeId, child: NodeId) !void {
        try self.nodes.items[parent].children.append(child);
    }

    pub fn node_type(self: *const Self, node: NodeId) NodeType {
        return self.nodes.items[node].data;
    }

    // helpers

    fn createNode(self: *Self, data: NodeData) !NodeId {
        const id = self.nodes.items.len;
        try self.nodes.append(Node{ .data = data, .children = std.ArrayList(NodeId).init(self.allocator) });
        return id;
    }
};
