const std = @import("std");
const c = @import("c.zig");
const Canvas = @import("gfx/canvas.zig").Canvas;
const WidgetRef = @import("widget.zig").WidgetRef;

pub const Window = struct {
    allocator: std.mem.Allocator,
    glfw_window: *c.GLFWwindow,
    canvas: Canvas,
    content: WidgetRef,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, title: [*:0]const u8, width: i32, height: i32, content: WidgetRef) !Self {
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

        const glfw_window = c.glfwCreateWindow(width, height, title, null, null) orelse return error.GlfwCreateWindowFailed;

        c.glfwMakeContextCurrent(glfw_window);
        _ = gladLoadGL();

        const canvas = try Canvas.init(allocator);

        return Self{
            .allocator = allocator,
            .glfw_window = glfw_window,
            .canvas = canvas,
            .content = content,
        };
    }

    pub fn render(self: *Self) void {
        var w: i32 = undefined;
        var h: i32 = undefined;
        c.glfwGetWindowSize(self.glfw_window, &w, &h);

        // TODO: clear()
        self.canvas.begin(@intToFloat(f32, w), @intToFloat(f32, h));
        self.content.render(&self.canvas);
        self.canvas.end();

        c.glfwSwapBuffers(self.glfw_window);
    }

    pub fn deinit(self: *Self) void {
        c.glfwDestroyWindow(self.glfw_window);
    }
};

extern fn gladLoadGL() callconv(.C) c_int;
