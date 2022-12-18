const std = @import("std");
const platform = @import("platform.zig");
const Viewport = @import("viewport.zig").Viewport;
const Document = @import("document.zig").Document;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    var doc = try Document.init(allocator);
    var el = try doc.createElement("div");
    var hello = try doc.createTextNode("hello");
    _ = el;
    _ = hello;
    // el.appendChild(hello);
    // doc.root().appendChild(el);

    try platform.init();
    defer platform.deinit();

    var win = try platform.Window.init("Hello", 400, 300);
    defer win.deinit();

    var vp = try Viewport.init(allocator, doc);
    defer vp.deinit();

    while (!win.shouldClose()) {
        vp.render(win.size(), win.scale());
        win.swapBuffers();
        platform.waitEvents();
    }
}
