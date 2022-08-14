const std = @import("std");
const Document = @import("dom.zig").Document;
const NodeId = @import("dom.zig").NodeId;
const NodeType = @import("dom.zig").NodeType;
const NodeData = @import("dom.zig").NodeData;
const Element = @import("dom.zig").Element;

pub const Node = struct {
    document: *Document,
    id: NodeId,
    data: NodeData,

    parent: ?*Node = null,
    first_child: ?*Node = null,
    next: ?*Node = null,

    const Self = @This();

    pub fn nodeType(self: *Self) NodeType {
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

    pub fn appendChild(self: *Self, child: *Self) void {
        // TODO: assert self.data != .text and child.parent == null

        if (self.first_child) |first| {
            var last = first;
            while (last.next) |n| last = n;

            last.next = child;
        } else {
            self.first_child = child;
        }

        child.parent = self;
    }

    // TODO: insertBefore()
    // TODO: removeChild()
};
