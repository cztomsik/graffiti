const builtin = @import("builtin");

comptime {
    if (!builtin.is_test) {
        _ = @import("napi.zig");
    }
}

test {
    _ = @import("css.zig");
    // _ = @import("document.zig");
    // _ = @import("layout.zig");
}
