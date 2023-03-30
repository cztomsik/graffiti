const builtin = @import("builtin");
const std = @import("std");

// TODO: pub lib.dom.*?
pub usingnamespace @import("dom/node.zig");
pub usingnamespace @import("dom/element.zig");
pub usingnamespace @import("dom/character_data.zig");
pub usingnamespace @import("dom/document.zig");

pub const Renderer = @import("renderer.zig").Renderer;
pub const Window = @import("platform.zig").Window;

comptime {
    if (!builtin.is_test) {
        _ = @import("napi.zig");
    }
}

test {
    std.testing.refAllDecls(@This());
    std.testing.refAllDecls(@import("css.zig"));
}
