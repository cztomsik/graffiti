const std = @import("std");
const napigen = @import("napigen");
const lib = @import("root");
const Window = @import("window.zig").Window;
const Document = @import("document.zig").Document;
const Node = @import("document.zig").Node;
const Renderer = @import("renderer.zig").Renderer;
const Style = @import("style.zig").Style;
const css = @import("css.zig");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

var window: Window = undefined;
var renderer: Renderer = undefined;

export fn napi_register_module_v1(env: napigen.napi_env, _: napigen.napi_value) napigen.napi_value {
    window = Window.init("Hello", 800, 600) catch @panic("err");
    renderer = Renderer.init(allocator) catch @panic("err");

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
        .Element_setStyle = &Element_setStyle,
        .Element_setStyleProp = &Element_setStyleProp,

        .render = &renderDoc,
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

fn renderDoc(doc: *Document) void {
    renderer.render(doc);
    window.swapBuffers();
    window.pollEvents();
}

fn Element_setStyle(node: *Node, style: []const u8) !void {
    if (node.as(.element)) |el| {
        // TODO: cx.allocator
        var parser = css.Parser.init(allocator, style);
        var block = try parser.parse(css.DeclarationBlock(Style));
        std.log.debug("parsed {any}", .{block});
        el.style = .{};
        block.apply(&el.style);
        std.log.debug("style = {any}", .{el.style});
    }
}

fn Element_setStyleProp(node: *Node, prop_name: []const u8, prop_value: []const u8) !void {
    if (node.as(.element)) |el| {
        // TODO: cx.allocator
        // TODO: hm, it's weird to even allocate... parsing prop value probably shouldn't need any allocation
        var parser = css.Parser.init(allocator, prop_value);
        var decl = css.DeclarationBlock(Style).parseDeclaration(&parser, prop_name) catch return;
        var block = css.DeclarationBlock(Style){ .declarations = &.{decl} };
        std.log.debug("parsed {any}", .{block});
        block.apply(&el.style);
        std.log.debug("style = {any}", .{el.style});
    }
}
