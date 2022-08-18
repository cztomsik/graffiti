const std = @import("std");
const Node = @import("dom.zig").Node;

pub const Text = struct {
    node: *Node,
    data: []const u8,
};
