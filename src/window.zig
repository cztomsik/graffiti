const std = @import("std");
const c = @import("c.zig");
const Canvas = @import("gfx/canvas.zig").Canvas;
const WidgetRef = @import("widget.zig").WidgetRef;

pub const Window = struct {
    allocator: std.mem.Allocator,
    glfw_window: *c.GLFWwindow,
    canvas: Canvas,
    content: ?WidgetRef = null,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, glfw_window: *c.GLFWwindow) !Self {
        c.glfwMakeContextCurrent(glfw_window);
        _ = gladLoadGL();

        const canvas = try Canvas.init(allocator);

        return Self{
            .allocator = allocator,
            .glfw_window = glfw_window,
            .canvas = canvas,
        };
    }

    pub fn deinit(self: *Self) void {
        c.glfwDestroyWindow(self.glfw_window);
    }

    pub fn shouldClose(self: *Self) bool {
        return c.glfwWindowShouldClose(self.glfw_window) == c.GLFW_TRUE;
    }

    pub fn render(self: *Self) void {
        var w: i32 = undefined;
        var h: i32 = undefined;
        c.glfwGetWindowSize(self.glfw_window, &w, &h);

        if (self.content) |*content| {
            // TODO: clear()
            self.canvas.begin(@intToFloat(f32, w), @intToFloat(f32, h));
            content.render(&self.canvas);
            self.canvas.end();
        }

        c.glfwSwapBuffers(self.glfw_window);
    }
};

extern fn gladLoadGL() callconv(.C) c_int;
