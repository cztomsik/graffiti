// minimal subset of DOM to serve as a model API
//
// for now, it allocates every node separately, which is simple but wasteful
// maybe we can use SegmentedList or `zig-stable-array` or something else,
// but we definitely need stable pointers and same-ptr upcasting for JS

const std = @import("std");
const emlay = @import("emlay");
const Node = @import("node.zig").Node;
const Element = @import("element.zig").Element;
const CharacterData = @import("character_data.zig").CharacterData;
const Style = @import("../style.zig").Style;
const StyleSheet = @import("../css/style_sheet.zig").StyleSheet;

pub const Document = struct {
    node: Node,
    allocator: std.mem.Allocator,
    style_sheets: std.ArrayList(StyleSheet),

    pub fn init(allocator: std.mem.Allocator) !*Document {
        var document = try allocator.create(Document);
        document.* = .{
            .node = .{ .owner_document = document, .node_type = .document },
            .allocator = allocator,
            .style_sheets = std.ArrayList(StyleSheet).init(allocator),
        };
        return document;
    }

    pub fn createElement(self: *Document, local_name: []const u8) !*Element {
        var element = try self.allocator.create(Element);
        element.* = try Element.init(self, local_name);
        return element;
    }

    pub fn createTextNode(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .owner_document = self, .node_type = .text },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    pub fn createComment(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .owner_document = self, .node_type = .comment },
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

    pub fn update(self: *Document) void {
        // apply inline styles for all elements
        self.updateStyles((self.node.first_child orelse return).cast(Element));

        emlay.layout(&LayoutContext{}, &self.node, self.node.size);
    }

    fn updateStyles(self: *Document, element: *Element) void {
        element.resolved_style = .{};
        element.style.apply(&element.resolved_style);

        var children = element.children();
        while (children.next()) |ch| {
            self.updateStyles(ch);
        }
    }
};

const LayoutContext = struct {
    pub inline fn resolve(_: @This(), dim: anytype, base: f32) f32 {
        return switch (dim) {
            .auto => std.math.nan_f32,
            .px => |v| v,
            .percent => |v| v * base,
            else => @panic("TODO"),
        };
    }

    pub inline fn style(_: @This(), node: *Node) *const Style {
        return switch (node.node_type) {
            .element => &node.cast(Element).resolved_style,
            .text => &INLINE_STYLE,
            .document => &DOCUMENT_STYLE,
            else => &HIDDEN_STYLE,
        };
    }

    pub inline fn children(_: @This(), node: *Node) Node.ChildNodesIterator {
        return node.childNodes();
    }

    pub inline fn target(_: @This(), node: *Node) *Node {
        return node;
    }

    // pub fn measure(node: *Node, ...) [2]f32 {}
};

const DOCUMENT_STYLE: Style = .{ .width = .{ .percent = 100 }, .height = .{ .percent = 100 } };
const INLINE_STYLE: Style = .{ .width = .{ .px = 100 }, .height = .{ .px = 20 } };
const HIDDEN_STYLE: Style = .{ .display = .none };
