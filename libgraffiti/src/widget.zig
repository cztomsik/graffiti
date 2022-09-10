const std = @import("std");
const Canvas = @import("gfx/canvas.zig").Canvas;

pub const WidgetRef = struct {
    widget: *anyopaque,
    vtable: *const VTable,

    const Self = @This();

    pub fn fromPtr(value: anytype) Self {
        comptime std.debug.assert(std.meta.trait.is(.Pointer)(@TypeOf(value)));

        return .{
            .widget = value,
            .vtable = VTable.forType(@TypeOf(value.*)),
        };
    }

    pub fn render(self: *Self, canvas: *Canvas) void {
        self.vtable.render(self.widget, canvas);
    }
};

const VTable = struct {
    render: std.meta.FnPtr(fn (*anyopaque, *Canvas) void),

    pub fn forType(comptime T: type) *const VTable {
        const Helper = struct {
            pub fn render(target: *anyopaque, canvas: *Canvas) void {
                T.render(@ptrCast(*T, target), canvas);
            }
        };

        return &.{
            .render = Helper.render,
        };
    }
};
