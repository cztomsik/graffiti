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

    /// Creates a new document.
    pub fn init(allocator: std.mem.Allocator) !*Document {
        var document = try allocator.create(Document);
        document.* = .{
            .node = .{ .owner_document = document, .node_type = .document },
            .allocator = allocator,
            .style_sheets = std.ArrayList(StyleSheet).init(allocator),
        };
        return document;
    }

    /// Returns the root element of the document, or null if there is no root element.
    pub fn documentElement(self: *Document) ?*Element {
        return if (self.node.first_child) |node| node.element() else null;
    }

    /// Returns the first <head> child of the document.
    pub fn head(self: *Document) ?*Element {
        return if (self.documentElement()) |root| root.childrenByLocalName("head").first() else null;
    }

    /// Returns the first <body> child of the document.
    pub fn body(self: *Document) ?*Element {
        return if (self.documentElement()) |root| root.childrenByLocalName("body").first() else null;
    }

    /// Creates a new element with the given local name.
    pub fn createElement(self: *Document, local_name: []const u8) !*Element {
        return Element.init(self, local_name);
    }

    /// Creates a new text node with the given data.
    pub fn createTextNode(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .owner_document = self, .node_type = .text },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    /// Creates a new comment node with the given data.
    pub fn createComment(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{ .owner_document = self, .node_type = .comment },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    /// Returns the node at the given point, or the root element if there is no such node.
    pub fn elementFromPoint(self: *Document, x: f32, y: f32) !*Node {
        try self.update();

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

    /// Updates the document after changes.
    pub fn update(self: *Document) !void {
        if (self.node.has_dirty) {
            try self.updateStyleSheets();
            self.updateStyles();
        }

        self.updateLayout();
    }

    fn updateStyleSheets(self: *Document) !void {
        const head_el = self.head() orelse return;

        if (head_el.node.has_dirty) {
            var style_els = head_el.childrenByLocalName("style");
            var i: usize = 0;

            while (style_els.next()) |el| : (i += 1) {
                if (el.node.is_dirty or el.node.has_dirty) {
                    var buf = std.ArrayList(u8).init(self.allocator);
                    defer buf.deinit();

                    var writer = buf.writer();
                    var childNodes = el.node.childNodes();

                    while (childNodes.next()) |child| {
                        if (child.node_type == .text) {
                            try writer.writeAll(@ptrCast(*CharacterData, child).data);
                        }
                    }

                    var sheet = try StyleSheet.parse(self.allocator, buf.items);
                    sheet.owner_node = el;

                    // replace or insert the sheet into the list at the correct position
                    if (self.findStyleSheet(el)) |ptr| {
                        ptr.deinit();
                        ptr.* = sheet;
                    } else {
                        try self.style_sheets.insert(i, sheet);
                    }
                }

                // remove old sheets
                while (self.style_sheets.items[i].owner_node != @as(*anyopaque, el)) {
                    self.style_sheets.items[i].deinit();
                    _ = self.style_sheets.orderedRemove(i);
                }

                el.node.has_dirty = false;
                el.node.is_dirty = false;
            }

            std.debug.assert(i == self.style_sheets.items.len);
        }
    }

    fn findStyleSheet(self: *Document, element: *Element) ?*StyleSheet {
        for (self.style_sheets.items) |*sheet| {
            if (sheet.owner_node == @as(*anyopaque, element)) return sheet;
        }
        return null;
    }

    fn updateStyles(self: *Document) void {
        const body_el = self.body() orelse return;

        if (body_el.node.has_dirty) {
            // TODO: apply sheets

            // apply inline styles for all elements
            self.applyInlineStyles(body_el);
        }
    }

    fn applyInlineStyles(self: *Document, element: *Element) void {
        element.resolved_style = .{};
        element.style.apply(&element.resolved_style);

        var children = element.children();
        while (children.next()) |ch| {
            self.applyInlineStyles(ch);
        }
    }

    fn updateLayout(self: *Document) void {
        emlay.layout(&LayoutContext{}, &self.node, self.node.size);
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
            .element => &@ptrCast(*Element, node).resolved_style,
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
