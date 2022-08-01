const std = @import("std");
const Style = @import("style.zig").Style;

pub const NodeId = usize;

const NodeType = enum(u32) { element = 1, text = 3, document = 9 };

const Node = struct {
    id: NodeId,
    data: NodeData,
    first_child: ?*Node = null,
    next: ?*Node = null,

    const Self = @This();

    pub fn node_type(self: *Self) NodeType {
        return self.data;
    }

    pub fn text(self: *Self) []const u8 {
        return switch (self.data) {
            .text => |text| text,
            else => @panic("invalid node type"), //Error.InvalidNodeType,
        };
    }

    pub fn element(self: *Self) *Element {
        return switch (self.data) {
            .element => |*element| element,
            else => @panic("invalid node type"), //Error.InvalidNodeType,
        };
    }

    // pub fn children(self: *Self, node: NodeId) []const NodeId {
    //     return self.nodes.items[node].children.items;
    // }

    pub fn appendChild(self: *Node, child: *Node) void {
        if (self.first_child) |first| {
            var last = first;
            while (last.next) |n| last = n;

            last.next = child;
        } else {
            self.first_child = child;
        }
    }
};

const NodeData = union(NodeType) { document, text: []const u8, element: Element };

const Element = struct {
    local_name: []const u8,
    style: Style = .{},
};

const Error = error{InvalidNodeType};

const NodeStore = std.SegmentedList(Node, 64);

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: *NodeStore,
    root: *Node,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !Self {
        var doc = Self{ .allocator = allocator, .nodes = try allocator.create(NodeStore), .root = undefined };
        doc.nodes.* = .{};
        doc.root = try doc.createNode(NodeData.document);
        std.debug.assert(doc.root.id == 0);

        return doc;
    }

    pub fn deinit(self: *Self) void {
        self.nodes.deinit(self.allocator);
        self.allocator.destroy(self.nodes);
    }

    pub fn node(self: *Self, id: NodeId) *Node {
        return self.nodes.at(id);
    }

    pub fn createTextNode(self: *Self, data: []const u8) !*Node {
        return self.createNode(NodeData{ .text = data });
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Node {
        return self.createNode(NodeData{ .element = .{ .local_name = local_name } });
    }

    // helpers

    fn createNode(self: *Self, data: NodeData) !*Node {
        const id = self.nodes.len;
        const ptr = try self.nodes.addOne(self.allocator);
        ptr.* = Node{ .id = id, .data = data };
        std.debug.print("createNode {}\n", .{id});

        return ptr;
    }
};
