const std = @import("std");
const nvg = @import("nanovg");
const c = @import("c.zig");
const Document = @import("document.zig").Document;
const Renderer = @import("renderer.zig").Renderer;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() anyerror!void {
    if (c.glfwInit() == 0) return error.GlfwInitFailed;
    defer c.glfwTerminate();

    c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MAJOR, 2);
    c.glfwWindowHint(c.GLFW_CONTEXT_VERSION_MINOR, 0);

    const window = c.glfwCreateWindow(800, 600, "Hello", null, null) orelse return error.GlfwCreateWindowFailed;
    defer c.glfwDestroyWindow(window);

    c.glfwMakeContextCurrent(window);

    _ = gladLoadGL();

    var renderer = try Renderer.init(allocator);
    var doc = try createSampleDoc();
    defer doc.deinit();

    while (c.glfwWindowShouldClose(window) == 0) {
        c.glfwWaitEvents();

        renderer.render(&doc);
        c.glfwSwapBuffers(window);
    }
}

fn createSampleDoc() !Document {
    var doc = try Document.init(allocator);

    const html = try doc.createElement("html");
    doc.root.appendChild(html);

    const body = try doc.createElement("body");
    body.element().style = .{
        .padding_top = .{ .px = 20 },
        .padding_right = .{ .px = 20 },
        .padding_bottom = .{ .px = 20 },
        .padding_left = .{ .px = 20 },
        .background_color = nvg.rgba(255, 0, 0, 200),
        .opacity = 0.75,
    };
    doc.root.appendChild(body);

    const div = try doc.createElement("div");
    body.appendChild(div);
    div.element().style = .{
        .background_color = nvg.rgba(0, 255, 0, 100),
        .border_radius = .{ 9, 9, 9, 9 },
    };

    const btn = try doc.createElement("button");
    btn.element().style = .{
        .background_color = nvg.rgb(0, 0, 255),
        .border_radius = .{ 9, 9, 9, 9 },
    };
    btn.appendChild(try doc.createTextNode("Click me"));

    div.appendChild(try doc.createTextNode("Hello"));
    div.appendChild(try doc.createTextNode("World"));
    div.appendChild(try doc.createTextNode("!"));
    div.appendChild(btn);

    return doc;
}

extern fn gladLoadGL() callconv(.C) c_int;

test {
    // _ = @import("css/parser.zig");
    _ = @import("css/tokenizer.zig");
}
