const napigen = @import("napigen");

export fn napi_register_module_v1(env: napigen.napi_env, _: napigen.napi_value) napigen.napi_value {
    return napigen.wrap(env, .{}) catch @panic("err");
}

// const std = @import("std");
// const lib = @import("lib.zig");
// const dom = @import("dom/dom.zig");
// const WidgetRef = @import("widget.zig").WidgetRef;

// const Hello = struct {
//     pub fn render(self: *Hello, canvas: *lib.Canvas) void {
//         canvas.drawText(.{ .w = 100, .h = 20 }, "Hello from " ++ @typeName(@TypeOf(self)));
//     }
// };

// pub fn main() anyerror!void {
//     var gpa = std.heap.GeneralPurposeAllocator(.{}){};
//     const allocator = gpa.allocator();
//     // defer if (gpa.deinit()) @panic("mem leak");

//     var app = try lib.App.init(allocator);
//     defer app.deinit();

//     var window = try app.createWindow("Hello", 800, 600);
//     defer window.deinit();

//     // TODO: HtmlView + @embedFile
//     var doc = try createSampleDoc(allocator);
//     defer doc.deinit();

//     var dom_view = try lib.DomView.init(allocator);
//     defer dom_view.deinit();
//     dom_view.dom_node = doc.node;
//     window.content = WidgetRef.fromPtr(&dom_view);

//     // var hello = Hello{};
//     //window.content = WidgetRef.fromPtr(&hello);

//     while (!window.shouldClose()) {
//         app.tick();
//         window.render();
//     }
// }

// fn createSampleDoc(allocator: std.mem.Allocator) !*dom.Document {
//     var parser = dom.DOMParser.init(allocator);

//     return try parser.parseFromString(
//         \\<html>
//         \\  <body style="padding: 20px; background: #f00a; opacity: .75">
//         \\    <div style="background: 0f08; border-radius: 9px">
//         \\      Hello
//         \\      <button style="background: 00f; border-radius: 9px">Click me</button>
//         \\    </div>
//         \\  </body>
//         \\</html>
//     );
// }

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
