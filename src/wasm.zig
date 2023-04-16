// pub fn log(comptime _: anytype, comptime _: anytype, comptime format: []const u8, args: anytype) void {
//     var logger: std.io.Writer(void, anyerror, print) = .{ .context = {} };
//     logger.print(format, args) catch @panic("log");
// }

// fn print(_: void, msg: []const u8) !usize {
//     jsPrint(msg.ptr, msg.len);
//     return msg.len;
// }

// extern fn jsPrint(ptr: [*]const u8, len: usize) void;

// export fn memset(ptr: [*]u8, value: u32, num: usize) [*]u8 {
//     std.mem.set(u8, ptr[0..num], @truncate(u8, value));
//     return ptr;
// }

// export fn memcpy(dst: [*]u8, src: [*]const u8, num: usize) [*]u8 {
//     std.mem.copy(u8, dst[0..num], src[0..num]);
//     return dst;
// }

// const malloc_alignment = 16;

// export fn malloc(size: usize) callconv(.C) ?*anyopaque {
//     const buffer = allocator.allocAdvanced(u8, malloc_alignment, size + malloc_alignment, .exact) catch return null;
//     std.mem.writeIntNative(usize, buffer[0..@sizeOf(usize)], buffer.len);
//     return buffer.ptr + malloc_alignment;
// }

// export fn realloc(ptr: ?*anyopaque, size: usize) callconv(.C) ?*anyopaque {
//     const p = ptr orelse return malloc(size);
//     defer free(p);
//     if (size == 0) return null;
//     const actual_buffer = @ptrCast([*]u8, p) - malloc_alignment;
//     const len = std.mem.readIntNative(usize, actual_buffer[0..@sizeOf(usize)]);
//     const new = malloc(size);
//     return memmove(new, actual_buffer + malloc_alignment, len);
// }

// export fn memmove(dest: ?*anyopaque, src: ?*anyopaque, n: usize) ?*anyopaque {
//     const csrc = @ptrCast([*]u8, src)[0..n];
//     const cdest = @ptrCast([*]u8, dest)[0..n];

//     // Create a temporary array to hold data of src
//     var buf: [1 << 12]u8 = undefined;
//     const temp = if (n <= buf.len) buf[0..n] else @ptrCast([*]u8, malloc(n))[0..n];
//     defer if (n > buf.len) free(@ptrCast(*anyopaque, temp));

//     for (csrc) |c, i|
//         temp[i] = c;

//     for (temp) |c, i|
//         cdest[i] = c;

//     return dest;
// }

// export fn free(ptr: ?*anyopaque) callconv(.C) void {
//     const actual_buffer = @ptrCast([*]u8, ptr orelse return) - 16;
//     const len = std.mem.readIntNative(usize, actual_buffer[0..@sizeOf(usize)]);
//     allocator.free(actual_buffer[0..len]);
// }
