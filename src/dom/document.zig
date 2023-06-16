// minimal subset of DOM to serve as a model API
//
// for now, it allocates every node separately, which is simple but wasteful
// maybe we can use SegmentedList or `zig-stable-array` or something else,
// but we definitely need stable pointers and same-ptr upcasting for JS

const std = @import("std");
const Node = @import("node.zig").Node;
const Element = @import("element.zig").Element;
const CharacterData = @import("character_data.zig").CharacterData;
const Style = @import("../style.zig").Style;
const StyleDeclaration = @import("../style.zig").StyleDeclaration;
const StyleSheet = @import("../style.zig").StyleSheet;

pub const Document = struct {
    node: Node,
    allocator: std.mem.Allocator,
    style_sheets: std.ArrayList(StyleSheet),

    /// Creates a new document.
    pub fn init(allocator: std.mem.Allocator) !*Document {
        var document = try allocator.create(Document);
        document.* = .{
            .node = .{
                .owner_document = document,
                .node_type = .document,
                .layout = .{
                    .style = .{ .width = .{ .fraction = 1 }, .height = .{ .fraction = 1 } },
                    .context = {},
                },
            },
            .allocator = allocator,
            .style_sheets = std.ArrayList(StyleSheet).init(allocator),
        };

        // add default stylesheet
        try document.style_sheets.append(try StyleSheet.parse(
            allocator,
            @embedFile("../../resources/ua.css"),
        ));

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
            .node = .{
                .owner_document = self,
                .node_type = .text,
                .layout = .{
                    .style = .{ .width = .{ .px = 100 }, .height = .{ .px = 20 } },
                    .context = {},
                },
            },
            .data = try self.allocator.dupe(u8, data),
        };
        return text;
    }

    /// Creates a new comment node with the given data.
    pub fn createComment(self: *Document, data: []const u8) !*CharacterData {
        var text = try self.allocator.create(CharacterData);
        text.* = .{
            .node = .{
                .owner_document = self,
                .node_type = .comment,
                .layout = .{
                    .style = .{ .display = .none },
                    .context = {},
                },
            },
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

        while (next) |node| {
            const n = &node.layout;

            // TODO: display, scroll, clip, radius, etc. and it's wrong anyway (overflow, absolute, etc.)
            if (node.node_type == .element and cur[0] >= n.pos[0] and cur[1] >= n.pos[1] and cur[0] <= (n.pos[0] + n.size[0]) and cur[1] <= (n.pos[1] + n.size[1])) {
                res = node;
                cur[0] -= n.pos[0];
                cur[1] -= n.pos[1];
                next = node.first_child;
            } else {
                next = node.next_sibling;
            }
        }

        return res;
    }

    /// Updates the document after changes.
    pub fn update(self: *Document) !void {
        if (self.node.has_dirty) {
            const force = try self.updateStyleSheets();
            self.updateTree(&self.node, force);
        }

        self.updateLayout();
    }

    // go through <style> elements and (re)parse them into StyleSheet objects if needed
    fn updateStyleSheets(self: *Document) !bool {
        const head_el = self.head() orelse return false;
        if (!head_el.node.has_dirty) return false;

        var changed = false;
        var style_els = head_el.childrenByLocalName("style");
        var i: usize = 1;

        while (style_els.next()) |el| : (i += 1) {
            if (el.node.is_dirty or el.node.has_dirty) {
                var buf = std.ArrayList(u8).init(self.allocator);
                defer buf.deinit();
                defer changed = true;

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
        }

        std.debug.assert(i == self.style_sheets.items.len);
        return changed;
    }

    // find existing sheet for the given element
    fn findStyleSheet(self: *Document, element: *Element) ?*StyleSheet {
        for (self.style_sheets.items) |*sheet| {
            if (sheet.owner_node == @as(*anyopaque, element)) return sheet;
        }
        return null;
    }

    // incrementally update the tree, applying styles and marking dirty nodes as clean
    fn updateTree(self: *Document, node: *Node, force: bool) void {
        if (force or node.is_dirty) {
            switch (node.node_type) {
                .element => updateElement(self, @ptrCast(*Element, node)),
                .text => {}, // TODO: node.layout.markDirty();
                else => {},
            }

            node.is_dirty = false;
        }

        if (force or node.has_dirty) {
            var childNodes = node.childNodes();
            while (childNodes.next()) |ch| {
                self.updateTree(ch, force);
            }

            node.has_dirty = false;
        }
    }

    // apply stylesheets and inline style to the given element
    fn updateElement(self: *Document, element: *Element) void {
        // clear
        element.node.layout.style = .{};
        element.layer_style = .{};

        for (self.style_sheets.items) |*sheet| {
            for (sheet.rules.items) |*rule| {
                if (element.matches(&rule.selector)) {
                    element.applyStyle(&rule.style);
                }
            }
        }

        // apply inline style
        element.applyStyle(&element.style);
    }

    fn updateLayout(self: *Document) void {
        self.node.layout.compute(self.node.layout.size);
    }
};
