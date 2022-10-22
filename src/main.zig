const builtin = @import("builtin");
const std = @import("std");

const document = @import("document.zig");
const renderer = @import("renderer.zig");

pub const Document = document.Document;
pub const Node = document.Node;
pub const Element = document.Element;
pub const Renderer = renderer.Renderer;

comptime {
    if (!builtin.is_test) {
        _ = @import("napi.zig");
    }
}

test {
    std.testing.refAllDecls(@This());
}
