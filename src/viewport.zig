const std = @import("std");
const Document = @import("document.zig").Document;
const Style = @import("style.zig").Style;
const Renderer = @import("renderer.zig").Renderer;
const LayoutNode = @import("layout.zig").LayoutNode;
const css = @import("css.zig");

pub const Viewport = struct {
    size: [2]f32,
    renderer: Renderer,
    document: *Document,
    // TODO: stable pointers - ChunkList or SegmentedList
    layout_nodes: std.ArrayList(LayoutNode),
    ua_sheet: css.StyleSheet(Style),

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, document: *Document) !Self {
        var parser = css.Parser.init(allocator, @embedFile("../resources/ua.css"));
        var ua_sheet = try parser.parse(css.StyleSheet(Style));

        std.debug.print("{}\n{} {}\n", .{ ua_sheet, @sizeOf(Style), @sizeOf(css.StyleDeclaration(Style)) });

        return Self{
            .size = .{ 400, 300 }, // TODO
            .renderer = try Renderer.init(allocator),
            .document = document, // TODO: refcounted
            .layout_nodes = std.ArrayList(LayoutNode).init(allocator),
            .ua_sheet = ua_sheet,
        };
    }

    pub fn deinit(self: *Self) void {
        self.renderer.deinit();
        self.layout_nodes.deinit();
        self.document.deinit();
        // TODO: self.ua_sheet.deinit();
    }

    pub fn render(self: *Self, size: [2]f32, scale: [2]f32) void {
        self.update();
        self.renderer.render(&self.layout_nodes.items[0], size, scale);
    }

    pub fn update(self: *Self) void {
        self.layout_nodes.resize(self.document.nodes.len());

        // TODO: incremental update
        // next = next_sibling orelse next.parent_node.next_sibling orelse next.parent_node.parent_node.next_sibling ...
        // if display: none -> skip subtree
        // if has_dirty -> go through all !dirty
        // if dirty -> go through all
        // if element -> compute style
        // if text -> single-line text layout using textstyle resolved from parent
        // if first_child/next_sibling changed -> repair links
        // if new -> init LayoutNode

        self.layout_nodes.items[0].compute(self.size);
    }

    // pub fn nodeAt(self: *Self, x: f32, y: f32) *Node {
    //     self.update();
    //     var res = self.document.root();
    //     var next: ?*Node = res;
    //     var cur: [2]f32 = .{ x, y };

    //     while (next) |n| {
    //         // std.debug.print("{} {d}@{d} {d}x{d} <- {d},{d}\n", .{ n.id, n.pos[0], n.pos[1], n.size[0], n.size[1], cur[0], cur[1] });

    //         // TODO: display, scroll, clip, radius, etc. and it's wrong anyway (overflow, absolute, etc.)
    //         if (cur[0] >= n.pos[0] and cur[1] >= n.pos[1] and cur[0] <= (n.pos[0] + n.size[0]) and cur[1] <= (n.pos[1] + n.size[1])) {
    //             // std.debug.print("res = {}\n", .{n.id});

    //             res = n;
    //             cur[0] -= n.pos[0];
    //             cur[1] -= n.pos[1];
    //             next = n.first_child;
    //         } else {
    //             next = n.next_sibling;
    //         }
    //     }

    //     return res;
    // }
};
