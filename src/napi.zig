const std = @import("std");
const napigen = @import("napigen");
const lib = @import("root");
const platform = @import("platform.zig");
const Style = @import("style.zig").Style;
const css = @import("css.zig");

// globals
var window: *platform.Window = undefined;
var document: *lib.Document = undefined;
var renderer: lib.Renderer = undefined;

comptime {
    napigen.defineModule(&initModule);
}

fn Text_data(node: *lib.Node) ![]const u8 {
    return if (node.as(.text)) |t| t else error.InvalidNode;
}

fn Text_setData(node: *lib.Node, data: []const u8) !void {
    return if (node.data == .text) {
        node.data = .{ .text = try node.document.allocator.dupe(u8, data) };
    } else error.InvalidNode;
}

fn Element_setStyle(node: *lib.Node, style: []const u8) !void {
    if (node.as(.element)) |el| {
        var parser = css.Parser.init(napigen.allocator, style);
        el.style = try parser.parse(css.StyleDeclaration(Style));
        // std.log.debug("style = {any}", .{el.style});
    }
}

fn Element_setStyleProp(node: *lib.Node, prop_name: []const u8, prop_value: []const u8) !void {
    if (node.as(.element)) |el| {
        el.style.setProperty(prop_name, prop_value);
        // std.log.debug("style = {any}", .{el.style});
    }
}

fn initModule(js: *napigen.JsContext, exports: napigen.napi_value) !napigen.napi_value {
    try js.setNamedProperty(exports, "Document_createElement", try js.createFunction(&lib.Document.createElement));
    try js.setNamedProperty(exports, "Document_createTextNode", try js.createFunction(&lib.Document.createTextNode));
    try js.setNamedProperty(exports, "Document_elementFromPoint", try js.createFunction(&lib.Document.elementFromPoint));
    try js.setNamedProperty(exports, "Node_appendChild", try js.createFunction(&lib.Node.appendChild));
    try js.setNamedProperty(exports, "Node_parentNode", try js.createFunction(&getter(lib.Node, .parent_node)));
    try js.setNamedProperty(exports, "Node_firstChild", try js.createFunction(&getter(lib.Node, .first_child)));
    // try js.setNamedProperty(exports, "Node_previousSibling", try js.createFunction(&getter(Node, .previous_sibling)));
    try js.setNamedProperty(exports, "Node_nextSibling", try js.createFunction(&getter(lib.Node, .next_sibling)));
    try js.setNamedProperty(exports, "Element_setStyle", try js.createFunction(&Element_setStyle));
    try js.setNamedProperty(exports, "Element_setStyleProp", try js.createFunction(&Element_setStyleProp));
    try js.setNamedProperty(exports, "Text_data", try js.createFunction(&Text_data));
    try js.setNamedProperty(exports, "Text_setData", try js.createFunction(&Text_setData));
    try js.setNamedProperty(exports, "init", try js.createFunction(&init));

    return exports;
}

fn getter(comptime T: type, comptime field: std.meta.FieldEnum(T)) fn (*T) std.meta.fieldInfo(T, field).type {
    return (struct {
        fn get(ptr: *T) std.meta.fieldInfo(T, field).type {
            return @field(ptr, @tagName(field));
        }
    }).get;
}

fn init(js: *napigen.JsContext) !napigen.napi_value {
    try platform.init();

    window = try platform.Window.init("Hello", 800, 600);
    document = try lib.Document.init(napigen.allocator);
    renderer = try lib.Renderer.init(napigen.allocator);

    // hook into libuv
    try UvHook.init(js);

    return js.write(.{
        .document = document,
        .window = window,
    });
}

const UvHook = struct {
    var js: *napigen.JsContext = undefined;
    var uv_loop: ?*napigen.uv_loop_s = undefined;
    var prepare_handle: uv_prepare_t = undefined;
    var idle_handle: uv_idle_t = undefined;
    var awaker: std.Thread = undefined;

    fn init(cx: *napigen.JsContext) !void {
        js = cx;

        try napigen.check(napigen.napi_get_uv_event_loop(js.env, &uv_loop));

        _ = uv_prepare_init(uv_loop, &prepare_handle);
        _ = uv_prepare_start(&prepare_handle, &waitEvents);
        _ = uv_idle_init(uv_loop, &idle_handle);

        // last part, if some I/O happens we need to wake up the main thread
        awaker = try std.Thread.spawn(.{}, monitorUvLoop, .{});
        awaker.detach();
    }

    // prepare task because we need to read timeout
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
        defer napigen.check(napigen.napi_close_handle_scope(js.env, scope)) catch @panic("close scope");

        // handle events in JS
        while (platform.nextEvent()) |ev| {
            const js_win = js.write(ev.target) catch @panic("get Window");
            const handleEvent = js.getNamedProperty(js_win, "handleEvent") catch @panic("get handleEvent");
            _ = js.callFunction(js_win, handleEvent, .{ev}) catch |e| js.throw(e);
        }

        // re-render in idle task
        _ = uv_idle_start(&idle_handle, &render);
    }

    // idle task because it continues after the prepare is done (and we don't get blocked if there's no more work)
    // but we also need to stop again because the timeout is always zero if there are any active idle tasks
    fn render(_: [*c]uv_idle_t) callconv(.C) void {
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

    // we can't target uv.h directly but 128 should be enough
    const uv_prepare_t = [128]u8;
    const uv_idle_t = [128]u8;
    extern fn uv_backend_timeout(loop: ?*napigen.uv_loop_s) c_int;
    extern fn uv_backend_fd(loop: ?*napigen.uv_loop_s) c_int;
    extern fn uv_prepare_init(loop: ?*napigen.uv_loop_s, prepare: [*c]uv_prepare_t) c_int;
    extern fn uv_prepare_start(prepare: [*c]uv_prepare_t, cb: ?*const fn ([*c]uv_prepare_t) callconv(.C) void) c_int;
    extern fn uv_idle_init(loop: ?*napigen.uv_loop_s, idle: [*c]uv_idle_t) c_int;
    extern fn uv_idle_start(idle: [*c]uv_idle_t, cb: ?*const fn ([*c]uv_idle_t) callconv(.C) void) c_int;
    extern fn uv_idle_stop(idle: [*c]uv_idle_t) callconv(.C) c_int;
};