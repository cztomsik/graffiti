const std = @import("std");
const d = @import("dom.zig");

test "basic usage" {
    const a = std.testing.allocator;

    var doc = d.Document.init(a);
    var div = doc.createElement("div");
    var hello = doc.createTextNode("Hello");
    div.node().appendChild(hello);
    doc.node().appendChild(div);
}
