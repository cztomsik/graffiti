const std = @import("std");
const Node = @import("dom.zig").Node;
const NodeId = @import("dom.zig").NodeId;
const NodeType = @import("dom.zig").NodeType;
const Element = @import("dom.zig").Element;
const Text = @import("dom.zig").Text;
const Comment = @import("dom.zig").Comment;
const DocumentFragment = @import("dom.zig").DocumentFragment;

pub const Document = struct {
    node: *Node,
    allocator: std.mem.Allocator,
    nodes: Store(Node),
    elements: Store(Element),
    texts: Store(Text),

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !*Self {
        var document = try allocator.create(Document);

        document.* = Self{
            .node = undefined,
            .allocator = allocator,
            .nodes = try Store(Node).init(allocator),
            .elements = try Store(Element).init(allocator),
            .texts = try Store(Text).init(allocator),
        };

        var node = try document.createNode(.{ .document = document });
        document.node = node;

        return document;
    }

    pub fn deinit(self: *Self) void {
        self.nodes.deinit();

        var els = self.elements.list.iterator(0);
        while (els.next()) |el| el.attributes.deinit();
        self.elements.deinit();

        self.texts.deinit();
        self.allocator.destroy(self);
    }

    pub fn nodeById(self: *Self, id: NodeId) *Node {
        return self.nodes.list.at(id);
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Element {
        var element = try self.elements.insert(Element{
            .node = undefined,
            .local_name = local_name,
            .attributes = std.BufMap.init(self.allocator),
        });
        var node = try self.createNode(.{ .element = element });
        element.node = node;

        return element;
    }

    pub fn createTextNode(self: *Self, data: []const u8) !*Text {
        var text = try self.texts.insert(Text{ .node = undefined, .data = data });
        var node = try self.createNode(.{ .text = text });
        text.node = node;

        return text;
    }

    // pub fn createComment(self: *Self, data: []const u8) !*Comment {
    //     var comment = try self.comments.insert(Comment{ .data = data });
    //     var node = try self.createNode(.{ .comment = comment });
    //     comment.node = node;

    //     return comment;
    // }

    // pub fn createDocumentFragment(self: *Self) !*Node {
    //     return self.createNode(.{.document_fragment}));
    // }

    // helpers

    fn createNode(self: *Self, data: anytype) !*Node {
        const id = self.nodes.list.len;
        return self.nodes.insert(Node{ .document = self, .id = id, .data = data });
    }
};

fn Store(comptime T: type) type {
    const List = std.SegmentedList(T, 64);

    return struct {
        allocator: std.mem.Allocator,
        list: *List,

        const Self = @This();

        pub fn init(allocator: std.mem.Allocator) !Self {
            var list = try allocator.create(List);
            list.* = .{};

            return Self{ .allocator = allocator, .list = list };
        }

        pub fn insert(self: *Self, value: T) !*T {
            const ptr = try self.list.addOne(self.allocator);
            ptr.* = value;

            return ptr;
        }

        pub fn deinit(self: *Self) void {
            self.list.deinit(self.allocator);
            self.allocator.destroy(self.list);
        }
    };
}
