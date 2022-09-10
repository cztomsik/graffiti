const std = @import("std");
const Document = @import("dom.zig").Document;
const NodeId = @import("dom.zig").NodeId;
const NodeType = @import("dom.zig").NodeType;
const Element = @import("dom.zig").Element;
const Text = @import("dom.zig").Text;
const Comment = @import("dom.zig").Comment;

pub const NodeId = usize;

pub const NodeType = enum(u32) {
    element = 1,
    text = 3,
    comment = 8,
    document = 9,
    document_fragment = 11,
};

pub const Node = struct {
    document: *Document,
    id: NodeId, // TODO: or lookup-table in Document?

    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    next_sibling: ?*Node = null,

    data: union(NodeType) {
        element: *Element,
        text: *Text,
        comment: *Comment,
        document: *Document,
        document_fragment,
    },

    const Self = @This();

    pub fn nodeType(self: *Self) NodeType {
        return self.data;
    }

    pub fn text(self: *Self) ?*Text {
        return switch (self.data) {
            .text => |ptr| ptr,
            else => null,
        };
    }

    pub fn element(self: *Self) ?*Element {
        return switch (self.data) {
            .element => |ptr| ptr,
            else => null,
        };
    }

    pub fn childNodes(self: *Self) ChildNodesIter {
        return .{ .next = self.first_child };
    }

    pub fn appendChild(self: *Self, child: *Self) void {
        // TODO: assert self.data != .text and child.parent == null

        if (self.first_child) |first| {
            var last = first;
            while (last.next_sibling) |n| last = n;

            last.next_sibling = child;
        } else {
            self.first_child = child;
        }

        child.parent_node = self;
    }

    // TODO: insertBefore()
    // TODO: replaceChild()
    // TODO: removeChild()
    // TODO: querySelector()
    // TODO: querySelectorAll()
};

const ChildNodesIter = struct {
    next: ?*Node,

    pub fn next(self: ChildNodesIter) ?*Node {
        defer self.next = next.next_sibling;
        return self.next;
    }
};
