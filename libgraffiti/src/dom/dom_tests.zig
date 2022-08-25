const std = @import("std");
const d = @import("dom.zig");

test "basic usage" {
    const a = std.testing.allocator;

    var doc = try d.Document.init(a);
    defer doc.deinit();

    var div = try doc.createElement("div");
    try div.setAttribute("class", "header");
    doc.node.appendChild(div.node);

    var hello = try doc.createTextNode("Hello");
    div.node.appendChild(hello.node);

    // try std.testing.expect(false);
}

test "inline style" {
    // div.setAttribute("style", "background: blue");
    // div.style().getPropertyValue() == "rgba(255, 0, 0, 255)";
    // div.style().removeProperty("background");
    // div.getAttribute("style") == "";
}
