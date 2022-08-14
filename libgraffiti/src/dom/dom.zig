const std = @import("std");

pub const NodeId = usize;
pub const NodeType = enum(u32) { element = 1, text = 3, document = 9 };

pub const Document = @import("Document.zig").Document;
pub const Node = @import("Node.zig").Node;
pub const Comment = @import("Comment.zig").Comment;
pub const Text = @import("Text.zig").Text;
pub const Element = @import("Element.zig").Element;
pub const DOMParser = @import("DOMParser.zig").DOMParser;
pub const XMLSerializer = @import("XMLSerializer.zig").XMLSerializer;

pub const NodeData = union(NodeType) {
    document,
    text: []const u8,
    element: Element,
};

test {
    _ = @import("dom_tests.zig");
}
