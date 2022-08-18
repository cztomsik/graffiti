const std = @import("std");
const Node = @import("dom.zig").Node;
const NodeId = @import("dom.zig").NodeId;
const NodeType = @import("dom.zig").NodeType;
const NodeData = @import("dom.zig").NodeData;

pub const Document = struct {
    node: *Node,
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
        // TODO: go through alive nodes and call freeNode on them?

        self.nodes.deinit(self.allocator);
        self.allocator.destroy(self.nodes);
    }

    pub fn node(self: *Self, id: NodeId) *Node {
        return self.nodes.at(id);
    }

    pub fn createTextNode(self: *Self, data: []const u8) !*Node {
        return self.createNode(NodeData{
            .text = try self.allocator.dupe(u8, data),
        });
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Node {
        return self.createNode(NodeData{ .element = .{
            .local_name = local_name,
            .attributes = std.StringHashMap([]const u8).init(self.allocator),
        } });
    }

    pub fn freeNode(self: *Self, n: *Node) void {
        switch (n.data) {
            .text => |t| self.allocator.free(t),
            else => {},
        }

        // TODO: mark as free, reuse in createNode
    }

    // helpers

    fn createNode(self: *Self, data: NodeData) !*Node {
        // if (self.free_head) |x| { .. self.free_head = x.next }

        const id = self.nodes.len;
        const ptr = try self.nodes.addOne(self.allocator);
        ptr.* = Node{ .document = self, .id = id, .data = data };
        // std.debug.print("createNode {} {any}\n", .{ id, data });

        return ptr;
    }
};

const NodeStore = std.SegmentedList(Node, 64);
