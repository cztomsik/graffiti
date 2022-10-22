const std = @import("std");
const napigen = @import("napigen");
const lib = @import("root");
const platform = @import("platform.zig");
const Style = @import("style.zig").Style;
const Node = lib.Node;
const Document = lib.Document;
const Element = lib.Element;
const css = @import("css.zig");

// globals
var allocator = std.heap.c_allocator;
var window: *platform.Window = undefined;
var document: *lib.Document = undefined;
var renderer: lib.Renderer = undefined;
var js: napigen.Context = undefined;
var handleEventRef: napigen.napi_ref = undefined;
var uv_loop: ?*napigen.uv_loop_s = undefined;
var prepare_handle: uv_prepare_t = undefined;
var idle_handle: uv_idle_t = undefined;
var awaker: std.Thread = undefined;

const JsInitOpts = struct {
    handleEvent: napigen.napi_value,
};

fn init(opts: JsInitOpts) !napigen.napi_value {
    try platform.init();

    window = try platform.Window.init("Hello", 800, 600);
    document = try lib.Document.init(allocator);
    renderer = try lib.Renderer.init(allocator);

    // save JS handler for later
    try napigen.check(napigen.napi_create_reference(js.env, opts.handleEvent, 1, &handleEventRef));

    // hook into libuv
    try napigen.check(napigen.napi_get_uv_event_loop(js.env, &uv_loop));
    _ = uv_prepare_init(uv_loop, &prepare_handle);
    _ = uv_prepare_start(&prepare_handle, &waitEvents);
    _ = uv_idle_init(uv_loop, &idle_handle);

    // last part, if some I/O happens we need to wake up the main thread
    awaker = try std.Thread.spawn(.{}, monitorUvLoop, .{});
    awaker.detach();

    return js.write(.{
        .document = document,
        // TODO: weakref bug
        .window = window,
    });
}

// prepare because we need to read timeout
fn waitEvents(_: [*c]uv_prepare_t) callconv(.C) void {
    const timeout = uv_backend_timeout(uv_loop);
    // std.debug.print("waitEvents {}\n", .{timeout});

    switch (timeout) {
        0 => platform.pollEvents(),
        -1 => platform.waitEvents(),
        else => |t| platform.waitEventsTimeout(@intToFloat(f64, t) / 1000),
    }

    // prepare JS
    var scope: napigen.napi_handle_scope = undefined;
    napigen.check(napigen.napi_open_handle_scope(js.env, &scope)) catch @panic("open scope");
    var handleEvent: napigen.napi_value = undefined;
    _ = napigen.check(napigen.napi_get_reference_value(js.env, handleEventRef, &handleEvent)) catch |e| js.throw(e);

    // handle events in JS
    while (platform.nextEvent()) |ev| {
        _ = js.call(void, handleEvent, .{ev}) catch |e| js.throw(e);
    }

    // bye-bye
    napigen.check(napigen.napi_close_handle_scope(js.env, scope)) catch @panic("close scope");

    // continue later
    _ = uv_idle_start(&idle_handle, &update);
}

// idle because it continues after the prepare is done (and we don't get blocked if there's no more work)
// but we also need to stop again because the timeout is always zero if there are any active idle tasks
fn update(_: [*c]uv_idle_t) callconv(.C) void {
    // re-render
    renderer.render(document, window.size(), window.scale());
    window.swapBuffers();

    // stop so we can read the timeout again
    _ = uv_idle_stop(&idle_handle);
}

fn monitorUvLoop() void {
    while (true) {
        // TODO: this wouldn't work because the timeout can change to lower value
        //       and then we are in trouble again - maybe we could use semaphore and wait before
        //       the main thread is done with processing? but I'm not sure if that would help
        //
        //       also, in theory, null should be enough but for some reason it waits even after I/O
        //
        // var timeout = uv_backend_timeout(uv_loop);
        // var tv: std.os.timespec = .{ .tv_sec = @divTrunc(timeout, 1000), .tv_nsec = @mod(timeout, 1000) * 1_000_000 };
        // var events = std.mem.zeroes([1]std.os.Kevent);
        // var res = std.os.kevent(uv_fd, &.{}, &events, &tv) catch @panic("kevent err");

        // super-simple for now (but we limit I/O to 200ms)
        std.time.sleep(200_000_000);

        platform.wakeUp();
    }
}

export fn napi_register_module_v1(env: napigen.napi_env, _: napigen.napi_value) napigen.napi_value {
    js = napigen.Context{ .env = env };

    const exports = .{
        .Document_createElement = &Document.createElement,
        .Document_createTextNode = &Document.createTextNode,
        .Document_elementFromPoint = &Document.elementFromPoint,

        .Node_appendChild = &Node.appendChild,
        .Node_parentNode = &getter(Node, .parent_node),
        .Node_firstChild = &getter(Node, .first_child),
        // .Node_previousSibling = &getter(Node, .previous_sibling),
        .Node_nextSibling = &getter(Node, .next_sibling),
        .Element_setStyle = &Element_setStyle,
        .Element_setStyleProp = &Element_setStyleProp,

        .init = &init,
    };

    return js.write(exports) catch |e| return js.throw(e);
}

fn getter(comptime T: type, comptime field: std.meta.FieldEnum(T)) fn (*T) std.meta.fieldInfo(T, field).field_type {
    const f = std.meta.fieldInfo(T, field);
    return (struct {
        fn get(ptr: *T) f.field_type {
            return @field(ptr, f.name);
        }
    }).get;
}

fn Element_setStyle(node: *Node, style: []const u8) !void {
    if (node.as(.element)) |el| {
        var parser = css.Parser.init(allocator, style);
        el.style = try parser.parse(css.StyleDeclaration(Style));
        // std.log.debug("style = {any}", .{el.style});
    }
}

fn Element_setStyleProp(node: *Node, prop_name: []const u8, prop_value: []const u8) !void {
    if (node.as(.element)) |el| {
        el.style.setProperty(prop_name, prop_value);
        // std.log.debug("style = {any}", .{el.style});
    }
}

// we can't target uv.h directly but we just need enough space
const uv_prepare_t = [128]u8;
const uv_idle_t = [128]u8;
extern fn uv_backend_timeout(loop: ?*napigen.uv_loop_s) c_int;
extern fn uv_backend_fd(loop: ?*napigen.uv_loop_s) c_int;
extern fn uv_prepare_init(loop: ?*napigen.uv_loop_s, prepare: [*c]uv_prepare_t) c_int;
extern fn uv_prepare_start(prepare: [*c]uv_prepare_t, cb: ?*const fn ([*c]uv_prepare_t) callconv(.C) void) c_int;
extern fn uv_idle_init(loop: ?*napigen.uv_loop_s, idle: [*c]uv_idle_t) c_int;
extern fn uv_idle_start(idle: [*c]uv_idle_t, cb: ?*const fn ([*c]uv_idle_t) callconv(.C) void) c_int;
extern fn uv_idle_stop(idle: [*c]uv_idle_t) callconv(.C) c_int;
