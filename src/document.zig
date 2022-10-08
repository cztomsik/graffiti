// minimal subset of DOM to serve as a model API

const std = @import("std");
const Style = @import("style.zig").Style;
const LayoutNode = @import("layout.zig").LayoutNode;

pub const Node = struct {
    id: usize,
    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    next_sibling: ?*Node = null,
    data: union(enum) {
        document: *Document,
        element: *Element,
        text: []const u8,
    },

    const Self = @This();

    pub fn as(self: *Self, comptime kind: std.meta.FieldEnum(@TypeOf(self.data))) ?std.meta.fieldInfo(@TypeOf(self.data), kind).field_type {
        return switch (self.data) {
            .document => |v| if (kind == .document) v else null,
            .element => |v| if (kind == .element) v else null,
            .text => |v| if (kind == .text) v else null,
        };
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

pub const Element = struct {
    local_name: []const u8,
    attributes: std.BufMap,
    style: Style,
};

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: std.SegmentedList(Node, 64),
    elements: std.SegmentedList(Element, 32),

    const Self = @This();

    // TODO
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    pub fn init() *Self {
        const allocator = gpa.allocator();
        var self = allocator.create(Self) catch @panic("OOM");
        self.* = .{
            .allocator = allocator,
            .nodes = .{},
            .elements = .{},
        };

        // self.nodes.insert(.{ .document = self });

        return self;
    }

    pub fn deinit(self: *Self) void {
        // TODO: deinit nodes (el attrs, ...)
        self.elements.deinit(self.allocator);
        self.nodes.deinit(self.allocator);
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Node {
        var element = try self.elements.addOne(self.allocator);
        element.* = .{
            .local_name = try self.allocator.dupe(u8, local_name),
            .attributes = std.BufMap.init(self.allocator),
            .style = .{},
        };

        return self.createNode(.{
            .element = element,
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
