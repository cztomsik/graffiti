const builtin = @import("builtin");
const std = @import("std");

const document = @import("document.zig");
const renderer = @import("renderer.zig");
const platform = @import("platform.zig");

pub const Node = document.Node;
pub const Element = document.Element;
pub const Document = document.Document;
pub const Renderer = renderer.Renderer;
pub const Window = platform.Window;

comptime {
    if (!builtin.is_test) {
        _ = @import("napi.zig");
    }
}

test {
    std.testing.refAllDecls(@This());
}
