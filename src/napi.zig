const std = @import("std");
const napigen = @import("napigen");
const lib = @import("main.zig");
const platform = @import("platform.zig");
const uv_hook = @import("uv_hook.zig");

// globals
var window: *lib.Window = undefined;
var document: *lib.Document = undefined;
var renderer: lib.Renderer = undefined;

comptime {
    napigen.defineModule(initModule);
}

fn initModule(js: *napigen.JsContext, exports: napigen.napi_value) !napigen.napi_value {
    // export init() function which will init native window, and return the globals for JS
    try js.setNamedProperty(exports, "init", try js.createFunction(init));

    // function wrappers and field getters we want to generate
    // `&` means we want to get a pointer to the field
    const defs = .{
        .Node = .{ .appendChild, .insertBefore, .removeChild, .querySelector, .markDirty },
        .Element = .{ .local_name, &.style, .getAttribute, .setAttribute, .removeAttribute, .matches },
        .CharacterData = .{ .data, .setData },
        .Document = .{ .createElement, .createTextNode, .elementFromPoint },
        .CSSStyleDeclaration = .{ .length, .item, .getPropertyValue, .setProperty, .removeProperty, .cssText, .setCssText },
    };

    // generate bindings
    inline for (std.meta.fields(@TypeOf(defs))) |f| {
        const T = @field(lib, f.name);

        inline for (@field(defs, f.name)) |member| {
            var fun = try js.createFunction(if (comptime std.meta.trait.is(.Pointer)(@TypeOf(member)))
                refGetter(T, member.*)
            else if (comptime std.meta.trait.hasFn(@tagName(member))(T))
                @field(T, @tagName(member))
            else
                valGetter(T, member));

            const name = if (comptime std.meta.trait.is(.Pointer)(@TypeOf(member)))
                @tagName(member.*)
            else
                @tagName(member);
            try js.setNamedProperty(exports, f.name ++ "_" ++ name, fun);
        }
    }

    return exports;
}

fn valGetter(comptime T: type, comptime field: std.meta.FieldEnum(T)) fn (*T) std.meta.fieldInfo(T, field).type {
    return (struct {
        fn get(ptr: *T) std.meta.fieldInfo(T, field).type {
            return @field(ptr, @tagName(field));
        }
    }).get;
}

fn refGetter(comptime T: type, comptime field: std.meta.FieldEnum(T)) fn (*T) *std.meta.fieldInfo(T, field).type {
    return (struct {
        fn get(ptr: *T) *std.meta.fieldInfo(T, field).type {
            return &@field(ptr, @tagName(field));
        }
    }).get;
}

fn init(js: *napigen.JsContext) !napigen.napi_value {
    try platform.init();

    window = try platform.Window.init("Hello", 800, 600);
    document = try lib.Document.init(napigen.allocator);
    renderer = try lib.Renderer.init(napigen.allocator);

    // hook into libuv
    try uv_hook.init(js);

    return js.write(.{
        .document = document,
        .window = window,
    });
}

pub fn napigenRead(js: *napigen.JsContext, comptime T: type, value: napigen.napi_value) !T {
    return switch (T) {
        *const lib.Selector => {
            var ptr = try js.arena.allocator().create(lib.Selector);
            ptr.* = lib.Selector.parse(js.arena.allocator(), try js.readString(value)) catch return error.napi_invalid_arg;
            return ptr;
        },
        else => js.defaultRead(T, value),
    };
}

// called from uv_hook.zig
pub fn update() !void {
    document.node.layout.size = window.size();
    try document.update();
    renderer.render(document, window.size(), window.scale());
    window.swapBuffers();
}
