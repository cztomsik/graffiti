const std = @import("std");
const emlay = @import("emlay");
const Element = @import("element.zig").Element;
const CharacterData = @import("character_data.zig").CharacterData;
const Document = @import("document.zig").Document;
const Selector = @import("../css/selector.zig").Selector;

// TODO: all node types should be extern structs
//       but then we couldn't store std.mem.Allocator :-/
pub const Node = struct {
    // tree
    owner_document: *Document,
    node_type: enum { element, text, comment, document },
    parent_node: ?*Node = null,
    first_child: ?*Node = null,
    last_child: ?*Node = null,
    previous_sibling: ?*Node = null,
    next_sibling: ?*Node = null,

    // change tracking
    is_dirty: bool = true,
    has_dirty: bool = true,

    // layout
    layout: emlay.LayoutNode(void, struct {
        next_child: ?*Node,

        pub fn init(node: *emlay.LayoutNode(void, @This())) @This() {
            return .{ .next_child = @fieldParentPtr(Node, "layout", node).first_child };
        }

        pub fn next(self: *@This()) ?*emlay.LayoutNode(void, @This()) {
            const ch = self.next_child orelse return null;
            self.next_child = ch.next_sibling;
            return &ch.layout;
        }
    }),

    /// Returns the node as an element, or null otherwise.
    pub fn element(self: *Node) ?*Element {
        return if (self.node_type == .element) @ptrCast(*Element, self) else null;
    }

    /// Returns whether the node has any children.
    pub fn hasChildNodes(self: *Node) bool {
        return self.first_child != null;
    }

    /// Returns an iterator over the node's children.
    pub fn childNodes(self: *Node) ChildNodesIterator {
        return .{ .next_child = self.first_child };
    }

    /// Appends a child node to the end of the node's children.
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
        child.markDirty();
    }

    /// Inserts a child node before another child node.
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
        child.markDirty();
    }

    /// Removes a child node from the node's children.
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

    /// Returns a first element matching the given selector, or null otherwise.
    pub fn querySelector(self: *Node, selector: []const u8) !?*Element {
        var sel = try Selector.parse(self.owner_document.allocator, selector);
        defer sel.deinit(self.owner_document.allocator);

        var descendants = DescendantsIterator{ .start = self, .pos = self };

        while (descendants.next()) |node| {
            if (node.element()) |el| {
                if (el.matches(&sel)) return el;
            }
        }

        return null;
    }

    // internal check, called before inserting/removing a node
    fn checkParent(self: *Node, node: *Node, parent: ?*Node) !void {
        if (node.owner_document != self.owner_document or node.parent_node != parent) {
            return error.InvalidChild;
        }
    }

    // mark node and all ancestors dirty
    pub fn markDirty(self: *Node) void {
        self.is_dirty = true;

        // propagate up, so we can then go in-order but skip up-to-date subtrees
        // we always recompute whole subtree so we don't need to mark descendants
        var next = self.parent_node;
        while (next) |n| : (next = n.parent_node) {
            if (n.is_dirty or n.has_dirty) break;
            n.has_dirty = true;
        }
    }

    pub const ChildNodesIterator = struct {
        next_child: ?*Node,

        pub fn next(self: *ChildNodesIterator) ?*Node {
            const ch = self.next_child orelse return null;
            self.next_child = ch.next_sibling;
            return ch;
        }
    };

    // TODO: consider pubslishing node.descendants()
    //       for now, this is only planned for querySelector(All)()
    pub const DescendantsIterator = struct {
        start: *Node,
        pos: *Node,

        pub fn next(self: *DescendantsIterator) ?*Node {
            if (self.pos.first_child) |ch| {
                self.pos = ch;
            } else if (self.pos.next_sibling) |n| {
                self.pos = n;
            } else {
                var x = self.pos;

                while (true) {
                    if (x == self.start) return null;

                    if (x.parent_node.?.next_sibling) |n| {
                        self.pos = n;
                        break;
                    }

                    x = x.parent_node.?;
                }
            }

            return self.pos;
        }
    };
};
