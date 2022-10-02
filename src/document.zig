// minimal subset of DOM to serve as a model API

const std = @import("std");

pub const NodeId = usize;

pub const NodeType = enum(u32) {
    element = 1,
    text = 3,
    // comment = 8,
    document = 9,
    // document_fragment = 11,
};

pub const Node = struct {
    id: NodeId,
    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    next_sibling: ?*Node = null,
    data: union(NodeType) {
        document: *Document,
        element: struct { local_name: []const u8, attributes: std.BufMap },
        text: []const u8,
    },

    const Self = @This();

    pub fn as(self: *Self, comptime kind: std.meta.FieldEnum(@TypeOf(self.data))) ?std.meta.fieldInfo(@TypeOf(self.data), kind).field_type {
        switch (self.data) {
            .document => |el| if (kind == .document) el else null,
            .element => |el| if (kind == .element) el else null,
            .text => |el| if (kind == .text) el else null,
        }
    }

    pub fn appendChild(self: *Self, child: *Node) void {
        if (self.first_child) |first| {
            var last = first;
            while (last.next_sibling) |n| last = n;
            last.next_sibling = child;
        } else {
            self.first_child = child;
        }

        child.parent_node = self;
    }
};

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: std.SegmentedList(Node, 64),

    const Self = @This();

    // TODO
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    pub fn init() *Self {
        const allocator = gpa.allocator();
        var self = allocator.create(Self) catch @panic("OOM");
        self.* = .{
            .allocator = allocator,
            .nodes = .{},
        };

        // self.nodes.insert(.{ .document = self });

        return self;
    }

    pub fn deinit(self: *Self) void {
        // TODO: deinit nodes (el attrs, ...)
        self.nodes.deinit(self.allocator);
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Node {
        return self.createNode(.{
            .element = .{
                .local_name = try self.allocator.dupe(u8, local_name),
                .attributes = std.BufMap.init(self.allocator),
            },
        });
    }

    pub fn createTextNode(self: *Self, data: []const u8) !*Node {
        return self.createNode(.{
            .text = try self.allocator.dupe(u8, data),
        });
    }

    // helpers

    fn createNode(self: *Self, data: anytype) !*Node {
        const id = self.nodes.len;
        const node = try self.nodes.addOne(self.allocator);
        node.* = Node{ .id = id, .data = data };
        return node;
    }
};
