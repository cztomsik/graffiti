// internal

const builtin = @import("builtin");

pub usingnamespace if (!builtin.is_test) @cImport({
    @cInclude("GLFW/glfw3.h");
}) else struct {};
