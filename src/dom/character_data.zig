const Node = @import("node.zig").Node;

pub const CharacterData = struct {
    node: Node,
    data: []const u8,

    pub fn setData(self: *CharacterData, data: []const u8) !void {
        self.node.owner_document.allocator.free(self.data);
        self.data = try self.node.owner_document.allocator.dupe(u8, data);
    }
};
