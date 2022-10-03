const std = @import("std");
const napigen = @import("napigen");
const nvg = @import("nanovg");
const App = @import("app.zig").App;
const Window = @import("window.zig").Window;
const Document = @import("document.zig").Document;
const Node = @import("document.zig").Node;
const c = @import("c.zig");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

var app: *App = undefined;
var window: *Window = undefined;
var vg: nvg = undefined;

export fn napi_register_module_v1(env: napigen.napi_env, _: napigen.napi_value) napigen.napi_value {
    app = App.init() catch @panic("err");
    window = app.createWindow("Hello", 800, 600) catch @panic("err");
    vg = nvg.gl.init(allocator, .{}) catch @panic("err");

    var cx = napigen.Context{ .env = env };

    const exports = .{
        .Document_init = &Document.init,
        .Document_createElement = &Document.createElement,
        .Document_createTextNode = &Document.createTextNode,

        .Node_appendChild = &Node.appendChild,
        .Node_parentNode = &getter(Node, .parent_node),
        .Node_firstChild = &getter(Node, .first_child),
        // .Node_previousSibling = &getter(Node, .previous_sibling),
        .Node_nextSibling = &getter(Node, .next_sibling),

        .render = &render,
    };

    return cx.write(exports) catch |e| return cx.throw(e);
}

fn getter(comptime T: type, comptime field: std.meta.FieldEnum(T)) fn (*T) std.meta.fieldInfo(T, field).field_type {
    const f = std.meta.fieldInfo(T, field);
    return (struct {
        fn get(ptr: *T) f.field_type {
            return @field(ptr, f.name);
        }
    }).get;
}

export fn render() void {
    vg.beginFrame(300, 300, 1);
    vg.beginPath();
    vg.rect(10, 10, 100, 100);
    vg.fillColor(nvg.rgb(100, 100, 100));
    vg.fill();

    vg.beginPath();
    vg.rect(20, 20, 100, 100);
    vg.fillColor(nvg.rgb(255, 0, 100));
    vg.fill();

    vg.endFrame();

    c.glfwSwapBuffers(window.glfw_window);
    app.pollEvents();
}

// test {
//     _ = @import("dom/dom.zig");
//     _ = @import("css/tokenizer.zig");
//     _ = @import("css/parser.zig");
//     _ = @import("css/properties.zig");
//     _ = @import("css/Selector.zig");
//     _ = @import("css/rule.zig");
//     _ = @import("css/sheet.zig");
//     _ = @import("css/values/box_shadow.zig");
//     _ = @import("css/values/Color.zig");
//     _ = @import("css/values/Dimension.zig");
//     _ = @import("css/values/Px.zig");
//     _ = @import("layout/layout.zig");
// }
