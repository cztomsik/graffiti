const std = @import("std");
const emlay = @import("emlay");
const css = @import("../css/mod.zig");
const Node = @import("node.zig").Node;
const Document = @import("document.zig").Document;
const Selector = @import("../css/mod.zig").Selector;
const LayerStyle = @import("../style.zig").LayerStyle;
const StyleDeclaration = @import("../style.zig").StyleDeclaration;

pub const Element = struct {
    node: Node,
    local_name: []const u8,
    attributes: std.BufMap,
    style: StyleDeclaration,
    layer_style: LayerStyle,

    pub fn init(document: *Document, local_name: []const u8) !*Element {
        var element = try document.allocator.create(Element);
        element.* = .{
            .node = .{
                .owner_document = document,
                .node_type = .element,
                .layout = .{},
            },
            .local_name = try document.allocator.dupe(u8, local_name),
            .attributes = std.BufMap.init(document.allocator),
            .style = StyleDeclaration.init(document.allocator),
            .layer_style = .{},
        };
        return element;
    }

    pub fn deinit(self: *Element) void {
        self.node.owner_document.allocator.free(self.local_name);
        self.attributes.deinit();
        self.style.deinit();
    }

    /// Returns an iterator over the children of this element.
    pub fn children(self: *Element) ChildrenIterator {
        return .{ .nodes = self.node.childNodes() };
    }

    /// Returns an iterator over the children of this element with the given local name.
    pub fn childrenByLocalName(self: *Element, local_name: []const u8) ChildrenByLocalName {
        return .{ .children = self.children(), .local_name = local_name };
    }

    /// Returns whether this element has any attributes.
    pub fn hasAttributes(self: *Element) bool {
        return self.attributes.hash_map.count() > 0;
    }

    /// Returns whether this element has the given attribute.
    pub fn hasAttribute(self: *Element, name: []const u8) bool {
        return self.attributes.hash_map.contains(name);
    }

    /// Returns an iterator over the names of the attributes of this element.
    pub fn getAttributeNames(self: *Element) AttributeNamesIterator {
        return .{ .attributes = self.attributes.hash_map.iterator() };
    }

    /// Returns the value of the given attribute, or null if it does not exist.
    pub fn getAttribute(self: *Element, name: []const u8) ?[]const u8 {
        return self.attributes.get(name);
    }

    /// Sets the value of the given attribute.
    pub fn setAttribute(self: *Element, name: []const u8, value: []const u8) !void {
        try self.attributes.put(name, value);
        self.node.markDirty();
    }

    /// Removes the given attribute.
    pub fn removeAttribute(self: *Element, name: []const u8) void {
        self.attributes.remove(name);
        self.node.markDirty();
    }

    /// Returns whether this element matches the given selector.
    pub fn matches(self: *Element, selector: *const Selector) bool {
        const Cx = struct {
            pub fn parentElement(el: *Element) ?*Element {
                const parent = el.node.parent_node orelse return null;
                return parent.element();
            }

            pub fn localName(el: *Element) []const u8 {
                return el.local_name;
            }

            pub fn id(el: *Element) []const u8 {
                return el.getAttribute("id") orelse "";
            }

            pub fn className(el: *Element) []const u8 {
                return el.getAttribute("class") orelse "";
            }
        };

        return selector.matchElement(Cx, self) != null;
    }

    /// Applies the given style to this element.
    pub fn applyStyle(self: *Element, style: *const StyleDeclaration) void {
        for (style.properties.items) |p| {
            switch (p) {
                inline else => |value, tag| {
                    const v = if (comptime @TypeOf(value) == css.Dimension) convertDim(value) else value;

                    if (comptime @hasField(emlay.Style, @tagName(tag))) {
                        @field(self.node.layout.style, @tagName(tag)) = v;
                    } else {
                        @field(self.layer_style, @tagName(tag)) = v;
                    }
                },
            }
        }

        if (self.node.layout.style.display == .none) {
            self.layer_style.visibility = .hidden;
        }
    }

    pub const ChildrenIterator = struct {
        nodes: Node.ChildNodesIterator,

        pub fn next(self: *ChildrenIterator) ?*Element {
            const node = self.nodes.next() orelse return null;
            return node.element() orelse self.next();
        }
    };

    pub const AttributeNamesIterator = struct {
        attributes: std.StringHashMap([]const u8).Iterator,

        pub fn next(self: *AttributeNamesIterator) ?[]const u8 {
            const entry = self.attributes.next() orelse return null;
            return entry.key_ptr.*;
        }
    };

    // TODO: intern local names and remove/replace this with one O(1) comparison in every step
    pub const ChildrenByLocalName = struct {
        children: ChildrenIterator,
        local_name: []const u8,

        pub fn next(self: *ChildrenByLocalName) ?*Element {
            const el = self.children.next() orelse return null;
            if (std.mem.eql(u8, el.local_name, self.local_name)) return el;
            return self.next();
        }
    };
};

fn convertDim(value: css.Dimension) emlay.Dimension {
    return switch (value) {
        .auto => .auto,
        .px => |v| .{ .px = v },
        .percent => |v| .{ .percent = v },
        else => @panic("TODO"),
    };
}
