const std = @import("std");
const c = @import("c.zig");
const Window = @import("window.zig").Window;

pub const App = struct {
    allocator: std.mem.Allocator,

    const Self = @This();

    // TODO
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    pub fn init() !*Self {
        if (c.glfwInit() == 0) return error.GlfwInitFailed;

        const allocator = gpa.allocator();
        var self = try allocator.create(Self);

        self.* = Self{
            .allocator = allocator,
        };

        return self;
    }

    pub fn deinit(self: *Self) void {
        c.glfwTerminate();
        self.allocator.destroy(self);
    }

    pub fn createWindow(self: *Self, title: []const u8, width: i32, height: i32) !*Window {
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

        const cstr = std.cstr.addNullByte(self.allocator, title) catch unreachable;
        defer self.allocator.free(cstr);

        const glfw_window = c.glfwCreateWindow(width, height, @ptrCast([*c]const u8, cstr), null, null) orelse return error.GlfwCreateWindowFailed;

        return try Window.init(self.allocator, glfw_window);
    }

    pub fn tick(self: *Self) void {
        _ = self;
        c.glfwWaitEvents();
    }
};
