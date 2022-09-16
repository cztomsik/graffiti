const std = @import("std");
const expectFmt = std.testing.expectFmt;
const LayoutStyle = @import("style.zig").LayoutStyle;
const Dimension = @import("style.zig").Dimension;

// mixins
const flex = @import("flex.zig");
const block = @import("block.zig");

pub const LayoutNode = struct {
    // TODO: *const Style because of sharing (interning, tests, &MyWidget.LAYOUT_STYLE)
    style: LayoutStyle = .{},

    // links
    first_child: ?*Self = null,
    next: ?*Self = null,

    // result
    x: f32 = 0,
    y: f32 = 0,
    width: f32 = 0,
    height: f32 = 0,

    // custom extensions
    user_ptr: ?*anyopaque = null,
    user_fn: ?std.meta.FnPtr(fn (*Self) void) = null,

    pub const Self = @This();

    pub fn format(self: *const Self, comptime _: []const u8, opts: std.fmt.FormatOptions, writer: anytype) !void {
        const indent = opts.width orelse 0;

        try writer.writeByteNTimes(' ', indent);
        try writer.print("{s} {d} {d} {d} {d}", .{ @tagName(self.style.display), self.x, self.y, self.width, self.height });

        var next = self.first_child;
        while (next) |ch| : (next = ch.next) {
            try writer.print("\n{:[w]}", .{ .v = ch, .w = indent + 2 });
        }
    }

    pub fn compute(self: *Self, width: f32, height: f32) void {
        self.width = self.style.width.resolve(width);
        self.height = self.style.height.resolve(height);

        switch (self.style.display) {
            .none => {
                self.width = 0;
                self.height = 0;
            },
            .block => self.computeBlock(width, height),
            .flex => self.computeFlex(width, height),
            .@"inline" => self.computeInline(),
        }

        // std.debug.print("{*} {} {d:.2} -> {d:.2}@{d:.2}\n", .{ node, node.style.display, parent_size.width, node.layout.size.width, node.layout.size.height });

        return;
    }

    pub fn computeInline(self: *Self) void {
        self.width = 100;
        self.height = 40;

        if (self.user_fn != null) {
            @panic("call user fn");
            // self.width = 10 * @intToFloat(f32, t.len);
            // self.height = 40;
        } else {
            self.width = 0;
            self.height = 0;
        }
    }

    usingnamespace block;
    usingnamespace flex;
};

test "LayoutNode.format()" {
    var n1 = LayoutNode{};
    try expectFmt("block 0 0 0 0", "{}", .{n1});

    var n2 = LayoutNode{ .first_child = &n1 };
    try expectFmt(
        \\block 0 0 0 0
        \\  block 0 0 0 0
    , "{}", .{n2});
}

test "display: none" {
    var n1 = LayoutNode{ .style = .{ .display = .none } };
    n1.compute(0, 0);
    try expectFmt("none 0 0 0 0", "{}", .{n1});
}
