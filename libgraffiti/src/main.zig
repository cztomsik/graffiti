const std = @import("std");
const lib = @import("lib.zig");
const c = @import("c.zig");
const dom = @import("dom/dom.zig");
const WidgetRef = @import("widget.zig").WidgetRef;
// const Renderer = @import("renderer.zig").Renderer;

const Hello = struct {
    pub fn render(self: *Hello, canvas: *lib.Canvas) void {
        canvas.drawText(.{ .w = 100, .h = 20 }, "Hello from " ++ @typeName(@TypeOf(self)));
    }
};

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    // defer if (gpa.deinit()) @panic("mem leak");

    if (c.glfwInit() == 0) return error.GlfwInitFailed;
    defer c.glfwTerminate();

    // var doc = try createSampleDoc(allocator);
    // defer doc.deinit();

    // var renderer = try Renderer.init(allocator);
    // defer renderer.deinit();

    var hello = Hello{};
    var window = try lib.Window.init(allocator, "Hello", 800, 600, WidgetRef.fromPtr(&hello));
    defer window.deinit();

    while (c.glfwWindowShouldClose(window.glfw_window) == 0) {
        c.glfwWaitEvents();
        window.render();
    }
}

fn createSampleDoc(allocator: std.mem.Allocator) !*dom.Document {
    var parser = dom.DOMParser.init(allocator);

    return try parser.parseFromString(
        \\<html>
        \\  <body style="padding: 20px; background: #f00a; opacity: .75">
        \\    <div style="background: 0f08; border-radius: 9px">
        \\      Hello
        \\      <button style="background: 00f; border-radius: 9px">Click me</button>
        \\    </div>
        \\  </body>
        \\</html>
    );
}

test {
    _ = @import("dom/dom.zig");
    _ = @import("css/tokenizer.zig");
    _ = @import("css/parser.zig");
    _ = @import("css/properties.zig");
    _ = @import("css/Selector.zig");
    _ = @import("css/values/BoxShadow.zig");
    _ = @import("css/values/Color.zig");
    _ = @import("css/values/Dimension.zig");
    _ = @import("css/values/Px.zig");
    _ = @import("layout/layout.zig");
}
