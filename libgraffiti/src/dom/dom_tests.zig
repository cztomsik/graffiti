const std = @import("std");
const d = @import("dom.zig");

test "basic usage" {
    const a = std.testing.allocator;

    var doc = try d.Document.init(a);
    defer doc.deinit();

    var div = try doc.createElement("div");
    var hello = try doc.createTextNode("Hello");
    // div.node().appendChild(hello);
    // doc.node().appendChild(div);
    std.debug.print("{any} {any}", .{ div, hello });
}
