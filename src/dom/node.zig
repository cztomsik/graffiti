const Document = @import("document.zig").Document;

// TODO: all node types should be extern structs
//       but then we couldn't store std.mem.Allocator :-/
pub const Node = struct {
    // tree
    document: *Document,
    node_type: enum { element, text, comment, document },
    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    last_child: ?*Node = null,
    previous_sibling: ?*Node = null,
    next_sibling: ?*Node = null,

    // layout
    pos: [2]f32 = .{ 0, 0 },
    size: [2]f32 = .{ 0, 0 },

    pub fn cast(self: *Node, comptime T: type) *T {
        return @ptrCast(*T, self);
    }

    pub fn children(self: *Node) ChildrenIterator {
        return .{ .next_item = self.first_child };
    }

    pub fn appendChild(self: *Node, child: *Node) !void {
        try self.checkParent(child, null);

        if (self.last_child) |last| {
            last.next_sibling = child;
            child.previous_sibling = last;
        } else {
            self.first_child = child;
        }

        self.last_child = child;
        child.parent_node = self;
    }

    pub fn insertBefore(self: *Node, child: *Node, before: *Node) !void {
        try self.checkParent(child, null);
        try self.checkParent(before, self);

        if (before.previous_sibling) |prev| {
            prev.next_sibling = child;
            child.previous_sibling = prev;
        } else {
            self.first_child = child;
        }

        child.next_sibling = before;
        before.previous_sibling = child;
        child.parent_node = self;
    }

    pub fn removeChild(self: *Node, child: *Node) !void {
        try self.checkParent(child, self);

        if (child.previous_sibling) |prev| {
            prev.next_sibling = child.next_sibling;
        } else {
            self.first_child = child.next_sibling;
        }

        if (child.next_sibling) |next| {
            next.previous_sibling = child.previous_sibling;
        } else {
            self.last_child = child.previous_sibling;
        }

        child.next_sibling = null;
        child.previous_sibling = null;
        child.parent_node = null;
    }

    fn checkParent(self: *Node, node: *Node, parent: ?*Node) !void {
        if (node.document != self.document or node.parent_node != parent) {
            return error.InvalidChild;
        }
    }

    pub fn markDirty(self: *Node) void {
        _ = self;
        //     self.flags.is_dirty = true;

        //     // propagate up, so we can go in-order but skip up-to-date subtrees
        //     // we always recompute whole subtree so we don't need to mark descendants
        //     var next = self.parent_node;
        //     while (next) |n| : (next = n.parent_node) {
        //         if (n.flags.is_dirty or n.flags.has_dirty) break;
        //         n.flags.has_dirty = true;
        //     }
    }
};

pub const ChildrenIterator = struct {
    next_item: ?*Node,

    pub fn next(self: *ChildrenIterator) ?*Node {
        if (self.next_item) |n| {
            self.next_item = n.next_sibling;
            return n;
        }

        return null;
    }
};

// TODO: consider pubslishing node.descendants()
//       for now, this is only planned for querySelectorAll()
// pub const DescendantsIterator = struct {
//     start: *Node,
//     pos: *Node,

//     pub fn next(self: *DescendantsIterator) ?*Node {
//         if (self.pos.first_child) |ch| {
//             self.pos = ch;
//         } else if (self.pos.next_sibling) |n| {
//             self.pos = n;
//         } else {
//             var x = self.pos;

//             while (true) {
//                 if (x == self.start) return null;

//                 if (x.parent_node.?.next_sibling) |n| {
//                     self.pos = n;
//                     break;
//                 }

//                 x = x.parent_node.?;
//             }
//         }

//         return self.pos;
//     }
// };
