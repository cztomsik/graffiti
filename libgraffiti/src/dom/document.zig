const std = @import("std");
const Style = @import("../style.zig").Style;

pub const NodeId = usize;

pub const NodeType = enum(u32) { element = 1, text = 3, document = 9 };

pub const Node = struct {
    document: *Document,
    id: NodeId,
    data: NodeData,

    parent: ?*Node = null,
    first_child: ?*Node = null,
    next: ?*Node = null,

    const Self = @This();

    pub fn nodeType(self: *Self) NodeType {
        return self.data;
    }

    pub fn text(self: *Self) []const u8 {
        return switch (self.data) {
            .text => |text| text,
            else => @panic("invalid node type"), //Error.InvalidNodeType,
        };
    }

    pub fn element(self: *Self) *Element {
        return switch (self.data) {
            .element => |*element| element,
            else => @panic("invalid node type"), //Error.InvalidNodeType,
        };
    }

    // pub fn children(self: *Self, node: NodeId) []const NodeId {
    //     return self.nodes.items[node].children.items;
    // }

    pub fn appendChild(self: *Self, child: *Self) void {
        // TODO: assert self.data != .text and child.parent == null

        if (self.first_child) |first| {
            var last = first;
            while (last.next) |n| last = n;

            last.next = child;
        } else {
            self.first_child = child;
        }

        child.parent = self;
    }

    // TODO: insertBefore()
    // TODO: removeChild()
};

const NodeData = union(NodeType) {
    document,
    text: []const u8,
    element: Element,
};

const Element = struct {
    local_name: []const u8,
    attributes: std.StringHashMap([]const u8),
    style: Style = .{},

    const Self = @This();

    // TODO: arena-allocated .style.cssText()
    pub fn getAttribute(self: *Self, att: []const u8) ?[]const u8 {
        return self.attributes.get(att);
    }

    pub fn setAttribute(self: *Self, att: []const u8, val: []const u8) !void {
        if (std.mem.eql(u8, att, "style")) {
            std.debug.print("TODO: parse & set style {s}\n", .{val});
        }

        if (try self.attributes.fetchPut(att, try self.attributes.allocator.dupe(u8, val))) |kv| {
            self.attributes.allocator.free(kv.value);
        }
    }

    pub fn removeAttribute(self: *Self, att: []const u8) void {
        if (self.attributes.fetchRemove(att)) |kv| {
            self.attributes.allocator.free(kv.v);
        }
    }
};

const NodeStore = std.SegmentedList(Node, 64);

pub const Document = struct {
    allocator: std.mem.Allocator,
    nodes: *NodeStore,
    root: *Node,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !Self {
        var doc = Self{ .allocator = allocator, .nodes = try allocator.create(NodeStore), .root = undefined };
        doc.nodes.* = .{};
        doc.root = try doc.createNode(NodeData.document);
        std.debug.assert(doc.root.id == 0);

        return doc;
    }

    pub fn deinit(self: *Self) void {
        // TODO: go through alive nodes and call freeNode on them?

        self.nodes.deinit(self.allocator);
        self.allocator.destroy(self.nodes);
    }

    pub fn node(self: *Self, id: NodeId) *Node {
        return self.nodes.at(id);
    }

    pub fn createTextNode(self: *Self, data: []const u8) !*Node {
        return self.createNode(NodeData{
            .text = try self.allocator.dupe(u8, data),
        });
    }

    pub fn createElement(self: *Self, local_name: []const u8) !*Node {
        return self.createNode(NodeData{ .element = .{
            .local_name = local_name,
            .attributes = std.StringHashMap([]const u8).init(self.allocator),
        } });
    }

    pub fn freeNode(self: *Self, n: *Node) void {
        switch (n.data) {
            .text => |t| self.allocator.free(t),
            else => {},
        }

        // TODO: mark as free, reuse in createNode
    }

    // helpers

    fn createNode(self: *Self, data: NodeData) !*Node {
        // if (self.free_head) |x| { .. self.free_head = x.next }

        const id = self.nodes.len;
        const ptr = try self.nodes.addOne(self.allocator);
        ptr.* = Node{ .document = self, .id = id, .data = data };
        // std.debug.print("createNode {} {any}\n", .{ id, data });

        return ptr;
    }
};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let mut doc = Document::new();
//         assert_eq!(doc.node_type(Document::ROOT), NodeType::Document);
//         assert_eq!(doc.children(Document::ROOT), &[]);

//         let div = doc.create_element("div");
//         assert_eq!(doc.node_type(div), NodeType::Element);
//         assert_eq!(doc.style(div), None);
//         assert_eq!(doc.local_name(div), "div");
//         assert_eq!(doc.children(div), &[]);

//         let hello = doc.create_text_node("hello");
//         assert_eq!(doc.node_type(hello), NodeType::Text);
//         assert_eq!(doc.text(hello), "hello");
//         doc.set_text(hello, "hello world");
//         assert_eq!(doc.text(hello), "hello world");

//         let other = doc.create_text_node("test");

//         doc.append_child(div, hello);
//         assert_eq!(doc.children(div), &[hello]);

//         doc.append_child(div, other);
//         assert_eq!(doc.children(div), &[hello, other]);

//         doc.remove_child(div, other);
//         assert_eq!(doc.children(div), &[hello]);
//     }

//     #[test]
//     fn qsa() {
//         let mut doc = Document::new();
//         let div = doc.create_element("div");

//         assert_eq!(doc.attribute(div, "id"), None);

//         doc.set_attribute(div, "id", "panel");
//         assert_eq!(doc.attribute(div, "id").as_deref(), Some("panel"));

//         // even before connecting, browsers do the same
//         assert!(doc.element_matches(div, "div#panel"));

//         doc.append_child(Document::ROOT, div);
//         assert_eq!(doc.query_selector(Document::ROOT, "div#panel"), Some(div));
//     }

//     /*
//     #[test]
//     fn inline_style() {
//         let mut doc = Document::new();
//         let div = doc.create_element("div");

//         doc[div].el_mut().set_style("display: block");
//         assert_eq!(doc[div].el().style().to_string(), "display:block;");

//         // doc[div].el_mut().style_mut().set_property("width", "100px");
//         // assert_eq!(
//         //     doc[div].el().attribute("style").as_deref(),
//         //     Some("display:block;width:100px;")
//         // );

//         doc[div].el_mut().set_style("");
//         assert_eq!(doc[div].el().style().to_string(), "");
//     }
//     */
// }
