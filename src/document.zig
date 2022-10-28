// minimal subset of DOM to serve as a model API

const std = @import("std");
const css = @import("css.zig");
const Style = @import("style.zig").Style;

pub const StyleDeclaration = css.StyleDeclaration(Style);

pub const Node = struct {
    id: usize,
    document: *Document,
    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    next_sibling: ?*Node = null,
    data: union(enum) {
        document: *Document,
        element: *Element,
        text: Text,
    },
    pos: [2]f32 = .{ 0, 0 },
    size: [2]f32 = .{ 0, 0 },

    const Self = @This();

    pub fn as(self: *Self, comptime kind: std.meta.FieldEnum(@TypeOf(self.data))) ?std.meta.fieldInfo(@TypeOf(self.data), kind).field_type {
        return switch (self.data) {
            kind => |v| v,
            else => null,
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

    pub fn getAttribute(self: *Self, name: []const u8) ?[]const u8 {
        return self.attributes.get(name);
    }

    pub fn setAttribute(self: *Self, name: []const u8, value: []const u8) !void {
        // TODO: mark dirty
        try self.attributes.put(name, value);
    }

    pub fn removeAttribute(self: *Self, name: []const u8) void {
        // TODO: mark dirty
        self.attributes.remove(name);
    }
};

pub const Text = struct {
    data: []const u8,
};

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: std.SegmentedList(Node, 64),
    elements: std.SegmentedList(Element, 32),

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !*Self {
        var doc = try allocator.create(Self);
        doc.* = .{
            .allocator = allocator,
            .nodes = .{},
            .elements = .{},
        };

        // doc.root = try doc.createNode(.{ .document = doc });

        return doc;
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
            .text = .{ .data = try self.allocator.dupe(u8, data) },
        });
    }

    pub fn elementFromPoint(self: *Self, x: f32, y: f32) *Node {
        // TODO: body/documentElement
        var res = self.nodes.at(0);
        var next: ?*Node = res;
        var cur: [2]f32 = .{ x, y };

        while (next) |n| {
            // std.debug.print("{} {d}@{d} {d}x{d} <- {d},{d}\n", .{ n.id, n.pos[0], n.pos[1], n.size[0], n.size[1], cur[0], cur[1] });

            // TODO: display, scroll, clip, radius, etc. and it's wrong anyway (overflow, absolute, etc.)
            if (n.data == .element and cur[0] >= n.pos[0] and cur[1] >= n.pos[1] and cur[0] <= (n.pos[0] + n.size[0]) and cur[1] <= (n.pos[1] + n.size[1])) {
                // std.debug.print("res = {}\n", .{n.id});

                res = n;
                cur[0] -= n.pos[0];
                cur[1] -= n.pos[1];
                next = n.first_child;
            } else {
                next = n.next_sibling;
            }
        }

        return res;
    }

    // helpers

    fn createNode(self: *Self, data: anytype) !*Node {
        const id = self.nodes.len;
        const node = try self.nodes.addOne(self.allocator);
        node.* = Node{ .id = id, .document = self, .data = data };
        return node;
    }
};
