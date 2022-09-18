const std = @import("std");
const c = @import("c.zig");
const Window = @import("window.zig").Window;

pub const App = struct {
    allocator: std.mem.Allocator,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator) !Self {
        if (c.glfwInit() == 0) return error.GlfwInitFailed;

        return Self{
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        _ = self;
        c.glfwTerminate();
    }

    pub fn createWindow(self: *Self, title: [*:0]const u8, width: i32, height: i32) !Window {
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

        const glfw_window = c.glfwCreateWindow(width, height, title, null, null) orelse return error.GlfwCreateWindowFailed;

        return Window.init(self.allocator, glfw_window);
    }

    pub fn tick(self: *Self) void {
        _ = self;
        c.glfwWaitEvents();
    }
};
