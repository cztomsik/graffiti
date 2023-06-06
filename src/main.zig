const builtin = @import("builtin");
const std = @import("std");

// export public types
pub const Node = @import("dom/node.zig").Node;
pub const Element = @import("dom/element.zig").Element;
pub const CharacterData = @import("dom/character_data.zig").CharacterData;
pub const Document = @import("dom/document.zig").Document;
pub const CSSStyleDeclaration = @import("style.zig").StyleDeclaration;
pub const Renderer = @import("renderer.zig").Renderer;
pub const Window = @import("platform.zig").Window;

// generate N-API bindings
pub usingnamespace if (!builtin.is_test) @import("napi.zig") else struct {};

// ref all decls for testing
test {
    std.testing.refAllDeclsRecursive(Document);
}
