const std = @import("std");
const nvg = @import("nanovg");
const c = @cImport({
    @cInclude("GLFW/glfw3.h");
});
const dom = @import("dom/dom.zig");
const Renderer = @import("renderer.zig").Renderer;

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    // defer if (gpa.deinit()) @panic("mem leak");

    if (c.glfwInit() == 0) return error.GlfwInitFailed;
    defer c.glfwTerminate();

    c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
    c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

    const window = c.glfwCreateWindow(800, 600, "Hello", null, null) orelse return error.GlfwCreateWindowFailed;
    defer c.glfwDestroyWindow(window);

    c.glfwMakeContextCurrent(window);

    _ = gladLoadGL();

    var doc = try createSampleDoc(allocator);
    defer doc.deinit();

    var renderer = try Renderer.init(allocator);
    defer renderer.deinit();

    while (c.glfwWindowShouldClose(window) == 0) {
        c.glfwWaitEvents();

        var w: i32 = undefined;
        var h: i32 = undefined;
        c.glfwGetWindowSize(window, &w, &h);
        renderer.render(&doc, @intToFloat(f32, w), @intToFloat(f32, h));
        c.glfwSwapBuffers(window);
    }
}

fn createSampleDoc(allocator: std.mem.Allocator) !dom.Document {
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

extern fn gladLoadGL() callconv(.C) c_int;

test {
    _ = @import("dom/dom.zig");
    // _ = @import("css/tokenizer.zig");
    // _ = @import("css/parser.zig");
    // _ = @import("layout/layout.zig");
}
