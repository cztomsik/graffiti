const builtin = @import("builtin");
const std = @import("std");

const c = if (!builtin.is_test) @cImport({
    @cInclude("GLFW/glfw3.h");
}) else struct {
    pub const GLFWwindow = opaque {};
    pub extern fn glfwSwapBuffers(window: ?*GLFWwindow) callconv(.C) void;
    pub extern fn glfwPollEvents() callconv(.C) void;
};

pub const Window = struct {
    glfw_window: *c.GLFWwindow,

    const Self = @This();

    pub fn init(title: [*:0]const u8, width: i32, height: i32) !Self {
        if (builtin.is_test) unreachable;

        // TODO: once
        if (c.glfwInit() == 0) return error.GlfwInitFailed;

        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

        const glfw_window = c.glfwCreateWindow(width, height, title, null, null) orelse return error.GlfwCreateWindowFailed;

        c.glfwMakeContextCurrent(glfw_window);
        _ = gladLoadGL();

        return Self{
            .glfw_window = glfw_window,
        };
    }

    pub fn deinit(self: *Self) void {
        c.glfwDestroyWindow(self.glfw_window);
    }

    pub fn shouldClose(self: *Self) bool {
        return c.glfwWindowShouldClose(self.glfw_window) == 1;
    }

    pub fn pollEvents(self: *Self) void {
        _ = self;
        c.glfwPollEvents();
    }

    pub fn getSize(self: *Self) [2]i32 {
        var res: [2]i32 = undefined;
        c.glfwGetWindowSize(self.glfw_window, &res[0], &res[1]);

        return res;
    }

    pub fn getContentScale(self: *Self) [2]f32 {
        var res: [2]f32 = undefined;
        c.glfwGetWindowContentScale(self.glfw_window, &res[0], &res[1]);

        return res;
    }

    pub fn swapBuffers(self: *Self) void {
        c.glfwSwapBuffers(self.glfw_window);
    }
};

extern fn gladLoadGL() callconv(.C) c_int;
