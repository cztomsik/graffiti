const builtin = @import("builtin");
const std = @import("std");

pub const Event = struct {
    kind: enum { mouse_move, scroll, mouse_down, mouse_up, key_down, key_press, key_up },
    target: ?*anyopaque = null,
    x: f64 = 0,
    y: f64 = 0,
    which: u32 = 0,
    char: u32 = 0,
};

pub const Window = opaque {
    const Self = @This();

    pub fn init(title: [*:0]const u8, width: i32, height: i32) !*Self {
        if (builtin.is_test) return undefined;

        // requested GL features
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
        c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

        const window = c.glfwCreateWindow(width, height, title, null, null) orelse return error.GlfwCreateWindowFailed;
        _ = c.glfwSetCursorPosCallback(window, &handleGlfwCursorPos);
        _ = c.glfwSetScrollCallback(window, &handleGlfwScroll);
        _ = c.glfwSetMouseButtonCallback(window, &handleGlfwMouseButton);
        _ = c.glfwSetCharCallback(window, &handleGlfwChar);
        _ = c.glfwSetKeyCallback(window, &handleGlfwKey);

        c.glfwMakeContextCurrent(window);
        _ = gladLoadGL();

        return @ptrCast(*Self, window);
    }

    fn handle(self: *Self) *c.GLFWwindow {
        return @ptrCast(*c.GLFWwindow, self);
    }

    pub fn deinit(self: *Self) void {
        c.glfwDestroyWindow(self.handle());
    }

    pub fn shouldClose(self: *Self) bool {
        return c.glfwWindowShouldClose(self.handle()) == c.GLFW_TRUE;
    }

    // TODO: title, setTitle

    pub fn size(self: *Self) [2]f32 {
        var res: [2]i32 = .{ 0, 0 };
        if (!builtin.is_test) c.glfwGetWindowSize(self.handle(), &res[0], &res[1]);
        return .{ @intToFloat(f32, res[0]), @intToFloat(f32, res[1]) };
    }

    // TODO: resize, show, hide, focus, blur, ...

    pub fn scale(self: *Self) [2]f32 {
        var res: [2]f32 = .{ 0, 0 };
        if (!builtin.is_test) c.glfwGetWindowContentScale(self.handle(), &res[0], &res[1]);
        return res;
    }

    pub fn swapBuffers(self: *Self) void {
        if (!builtin.is_test) c.glfwSwapBuffers(self.handle());
    }
};

pub fn init() !void {
    if (c.glfwInit() == 0) return error.GlfwInitFailed;
}

pub fn waitEvents() void {
    c.glfwWaitEvents();
}

pub fn nextEvent() ?Event {
    return events.popOrNull();
}

// TODO: async in self-hosted
// var events_buf: [32]Event = undefined;
// var events = std.event.Channel(Event).init(&events_buf);
var events = std.ArrayList(Event).init(std.heap.c_allocator);

// GLFW below

const c = if (!builtin.is_test) @cImport({
    @cInclude("GLFW/glfw3.h");
}) else struct {};

extern fn gladLoadGL() callconv(.C) c_int;

fn handleGlfwCursorPos(w: ?*c.GLFWwindow, x: f64, y: f64) callconv(.C) void {
    pushEvent(.{ .target = w, .kind = .mouse_move, .x = x, .y = y });
}

fn handleGlfwScroll(w: ?*c.GLFWwindow, x: f64, y: f64) callconv(.C) void {
    pushEvent(.{ .target = w, .kind = .scroll, .x = x, .y = y });
}

fn handleGlfwMouseButton(w: ?*c.GLFWwindow, _: c_int, action: c_int, _: c_int) callconv(.C) void {
    var pos: [2]f64 = .{ 0, 0 };
    c.glfwGetCursorPos(w, &pos[0], &pos[1]);
    pushEvent(.{ .target = w, .kind = if (action == c.GLFW_PRESS) .mouse_down else .mouse_up, .x = pos[0], .y = pos[0] });
}

fn handleGlfwKey(w: ?*c.GLFWwindow, key: c_int, _: c_int, action: c_int, _: c_int) callconv(.C) void {
    // TODO: key -> which
    pushEvent(.{ .target = w, .kind = if (action == c.GLFW_PRESS) .key_down else .key_up, .which = @intCast(u32, key) });
}

fn handleGlfwChar(w: ?*c.GLFWwindow, char: c_uint) callconv(.C) void {
    pushEvent(.{ .target = w, .kind = .key_press, .char = char });
}

fn pushEvent(event: Event) void {
    events.insert(0, event) catch @panic("OOM");
}
