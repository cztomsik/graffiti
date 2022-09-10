const std = @import("std");

pub const Document = @import("Document.zig").Document;
pub const Node = @import("Node.zig").Node;
pub const NodeId = @import("Node.zig").NodeId;
pub const NodeType = @import("Node.zig").NodeType;
pub const Comment = @import("Comment.zig").Comment;
pub const Text = @import("Text.zig").Text;
pub const Element = @import("Element.zig").Element;
pub const DOMParser = @import("DOMParser.zig").DOMParser;
pub const XMLSerializer = @import("XMLSerializer.zig").XMLSerializer;

test {
    _ = @import("dom_tests.zig");
}
