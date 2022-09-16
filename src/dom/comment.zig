const std = @import("std");
const Node = @import("dom.zig").Node;

pub const Comment = struct {
    node: *Node,
    data: []const u8,
};
