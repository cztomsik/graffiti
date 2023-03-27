// minimal subset of DOM to serve as a model API
//
// for now, it allocates every node separately, which is simple but wasteful
// maybe we can use SegmentedList or `zig-stable-array` or something else,
// but we definitely need stable pointers and upcasting because of JS bindings

const std = @import("std");
const css = @import("css.zig");
const Style = @import("style.zig").Style;

pub const StyleDeclaration = css.StyleDeclaration(Style);

// TODO: these could/should be extern structs but
//       then we can't store std.mem.Allocator in Document :-/
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

    pub fn children(self: *Node) ChildrenIter {
        return .{ .next = self.first_child };
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

    // fn markDirty(self: *Self) void {
    //     self.flags.is_dirty = true;

    //     // propagate up, so we can go in-order but skip up-to-date subtrees
    //     // we always recompute whole subtree so we don't need to mark descendants
    //     var next = self.parent_node;
    //     while (next) |n| : (next = n.parent_node) {
    //         if (n.flags.is_dirty or n.flags.has_dirty) break;
    //         n.flags.has_dirty = true;
    //     }
    // }
};

pub const Element = struct {
    node: Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: StyleDeclaration,

    pub fn getAttribute(self: *Element, name: []const u8) ?[]const u8 {
        return self.attributes.get(name);
    }

    pub fn setAttribute(self: *Element, name: []const u8, value: []const u8) !void {
        try self.attributes.put(name, value);
    }

    pub fn removeAttribute(self: *Element, name: []const u8) void {
        self.attributes.remove(name);
    }
};

pub const CharacterData = struct {
    node: Node,
    data: []const u8,

    pub fn setData(self: *CharacterData, data: []const u8) !void {
        self.allocator.free(self.data);
        self.data = try self.allocator.dupe(u8, data);
    }
};

pub const Document = struct {
    node: Node,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !*Document {
        var document = try allocator.create(Document);
        document.* = .{
            .node = .{ .document = document, .node_type = .document },
            .allocator = allocator,
        };
        return document;
    }

    pub fn createElement(self: *Document, local_name: []const u8) !*Element {
        var element = try self.allocator.create(Element);
        element.* = .{
            .node = .{ .document = self, .node_type = .element },
            .local_name = try self.allocator.dupe(u8, local_name),
            .attributes = std.BufMap.init(self.allocator),
            .style = .{},
        };
        return element;
    }

    pub fn createTextNode(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .document = self, .node_type = .text },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    pub fn createComment(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .document = self, .node_type = .comment },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    pub fn elementFromPoint(self: *Document, x: f32, y: f32) *Node {
        var res = self.node.first_child orelse @panic("no root element");
        var next: ?*Node = res;
        var cur: [2]f32 = .{ x, y };

        while (next) |n| {
            // TODO: display, scroll, clip, radius, etc. and it's wrong anyway (overflow, absolute, etc.)
            if (n.node_type == .element and cur[0] >= n.pos[0] and cur[1] >= n.pos[1] and cur[0] <= (n.pos[0] + n.size[0]) and cur[1] <= (n.pos[1] + n.size[1])) {
                res = n;
                cur[0] -= n.pos[0];
                cur[1] -= n.pos[1];
                next = n.first_child;
            } else {
                next = n.next_sibling;
            }
        }

        return res;
    }
};

pub const ChildrenIter = struct {
    next: ?*Node,

    pub fn next(self: *ChildrenIter) ?*Node {
        if (self.next) |n| {
            self.next = n.next_sibling;
            return n;
        }

        return null;
    }
};
