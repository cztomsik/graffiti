// minimal subset of DOM to serve as a model API

const std = @import("std");
const css = @import("css.zig");
const Style = @import("style.zig").Style;

const StyleDeclaration = css.StyleDeclaration(Style);

pub const Node = struct {
    id: usize,
    document: *Document,
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
    node: *Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: StyleDeclaration,

    const Self = @This();

    fn getAttribute(self: *Self, name: []const u8) ?[]const u8 {
        // TODO: mark dirty
        return self.attributes.get(name);
    }

    fn setAttribute(self: *Self, name: []const u8, value: []const u8) !void {
        // TODO: mark dirty
        try self.attributes.put(name, value);
    }

    fn removeAttribute(self: *Self, name: []const u8) ?[]const u8 {
        // TODO: mark dirty
        try self.attributes.remove(name);
    }
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
        var node = try self.createNode(.{
            .element = element,
        });

        element.* = .{
            .node = node,
            .local_name = try self.allocator.dupe(u8, local_name),
            .attributes = std.BufMap.init(self.allocator),
            .style = .{},
        };

        return node;
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
        node.* = Node{ .id = id, .document = self, .data = data };
        return node;
    }
};
