const std = @import("std");
const napigen = @import("napigen");
const App = @import("app.zig").App;
const Document = @import("document.zig").Document;
const Node = @import("document.zig").Node;
const c = @import("c.zig");

export fn napi_register_module_v1(env: napigen.napi_env, _: napigen.napi_value) napigen.napi_value {
    var cx = napigen.Context{ .env = env };

    const exports = .{
        .App_init = &App.init,
        .App_tick = &App.tick,
        .App_createWindow = &App.createWindow,

        .Document_init = &Document.init,
        .Document_createElement = &Document.createElement,
        .Document_createTextNode = &Document.createTextNode,

        .Node_appendChild = &Node.appendChild,
        .Node_parentNode = &getter(Node, .parent_node),
        .Node_firstChild = &getter(Node, .first_child),
        // .Node_previousSibling = &getter(Node, .previous_sibling),
        .Node_nextSibling = &getter(Node, .next_sibling),
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
