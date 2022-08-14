const std = @import("std");

const Style = @import("../style.zig").Style;

pub const Element = struct {
    local_name: []const u8,
    attributes: std.StringHashMap([]const u8),
    style: Style = .{},

    const Self = @This();

    // TODO: arena-allocated .style.cssText()
    pub fn getAttribute(self: *Self, att: []const u8) ?[]const u8 {
        return self.attributes.get(att);
    }

    pub fn setAttribute(self: *Self, att: []const u8, val: []const u8) !void {
        if (std.mem.eql(u8, att, "style")) {
            std.debug.print("TODO: parse & set style {s}\n", .{val});
        }

        if (try self.attributes.fetchPut(att, try self.attributes.allocator.dupe(u8, val))) |kv| {
            self.attributes.allocator.free(kv.value);
        }
    }

    pub fn removeAttribute(self: *Self, att: []const u8) void {
        if (self.attributes.fetchRemove(att)) |kv| {
            self.attributes.allocator.free(kv.v);
        }
    }
};
