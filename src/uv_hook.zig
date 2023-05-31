// libuv event loop integration
//
// this is needed because everything UI-related must be done in the main thread
// which is occupied by node.js event loop so we need to define few libuv hooks
// and wait until node.js is done with its work, and only then we can block the
// main thread and wait for UI events, we also need to wake up the main thread
// if anything node.js related happens (e.g. I/O) so the JS code can handle it

const std = @import("std");
const napigen = @import("napigen");
const platform = @import("platform.zig");
const napi = @import("napi.zig");

// globals
var js: *napigen.JsContext = undefined;
var uv_loop: ?*napigen.uv_loop_s = undefined;
var prepare_handle: uv_prepare_t = undefined;
var idle_handle: uv_idle_t = undefined;
var awaker: std.Thread = undefined;

pub fn init(cx: *napigen.JsContext) !void {
    // save for later
    js = cx;

    // get the libuv handle
    try napigen.check(napigen.napi_get_uv_event_loop(js.env, &uv_loop));

    // setup prepare and idle tasks/handles
    // - prepare will be called at the end of the tick
    // - idle task will be scheduled by prepare and will stop itself
    _ = uv_prepare_init(uv_loop, &prepare_handle);
    _ = uv_prepare_start(&prepare_handle, &waitEvents);
    _ = uv_idle_init(uv_loop, &idle_handle);

    // last part, if some I/O happens we need to wake up the main thread somehow
    // so we spawn a thread which will monitor the libuv loop and wake up the
    // main thread when needed
    awaker = try std.Thread.spawn(.{}, monitorUvLoop, .{});
    awaker.detach();
}

// prepare task because we need to read timeout
fn waitEvents(_: [*c]uv_prepare_t) callconv(.C) void {
    // wait or poll depending on the libuv timeout
    switch (uv_backend_timeout(uv_loop)) {
        0 => platform.pollEvents(),
        -1 => platform.waitEvents(),
        else => |t| platform.waitEventsTimeout(@intToFloat(f64, t) / 1000),
    }

    // prepare JS scope
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

// idle task continues after the prepare is done but we also need to stop again
// because the timeout is always zero if there are any active idle tasks
fn render(_: [*c]uv_idle_t) callconv(.C) void {
    napi.update();

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

// libuv headers
// we can't target uv.h directly but this should be enough
const uv_prepare_t = [128]u8;
const uv_idle_t = [128]u8;
extern fn uv_backend_timeout(loop: ?*napigen.uv_loop_s) c_int;
extern fn uv_backend_fd(loop: ?*napigen.uv_loop_s) c_int;
extern fn uv_prepare_init(loop: ?*napigen.uv_loop_s, prepare: [*c]uv_prepare_t) c_int;
extern fn uv_prepare_start(prepare: [*c]uv_prepare_t, cb: ?*const fn ([*c]uv_prepare_t) callconv(.C) void) c_int;
extern fn uv_idle_init(loop: ?*napigen.uv_loop_s, idle: [*c]uv_idle_t) c_int;
extern fn uv_idle_start(idle: [*c]uv_idle_t, cb: ?*const fn ([*c]uv_idle_t) callconv(.C) void) c_int;
extern fn uv_idle_stop(idle: [*c]uv_idle_t) callconv(.C) c_int;
