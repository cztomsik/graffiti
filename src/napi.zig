const std = @import("std");
const napigen = @import("napigen");
const lib = @import("root");
const platform = @import("platform.zig");

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
        .Node = .{ .parent_node, .first_child, .next_sibling, .appendChild, .insertBefore, .removeChild },
        .Element = .{ .local_name, &.style, .getAttribute, .setAttribute, .removeAttribute },
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
        document.node.size = window.size();
        document.update();
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
