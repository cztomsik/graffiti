const std = @import("std");

pub const Document = @import("document.zig").Document;
pub const Node = @import("node.zig").Node;
pub const NodeId = @import("node.zig").NodeId;
pub const NodeType = @import("node.zig").NodeType;
pub const Comment = @import("comment.zig").Comment;
pub const Text = @import("text.zig").Text;
pub const Element = @import("element.zig").Element;
pub const DOMParser = @import("dom_parser.zig").DOMParser;

test {
    _ = @import("dom_tests.zig");
}
