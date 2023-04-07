const builtin = @import("builtin");
const std = @import("std");

pub const Node = @import("dom/node.zig").Node;
pub const Element = @import("dom/element.zig").Element;
pub const CharacterData = @import("dom/character_data.zig").CharacterData;
pub const Document = @import("dom/document.zig").Document;

pub const Renderer = @import("renderer.zig").Renderer;
pub const Window = @import("platform.zig").Window;

comptime {
    if (!builtin.is_test) {
        _ = @import("napi.zig");
    }
}

test {
    std.testing.refAllDeclsRecursive(Document);
}
