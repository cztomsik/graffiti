const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;

pub const NumberOrPercentage = struct {
    value: f32,

    pub fn format(self: NumberOrPercentage, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("{d}%", .{self.value * 100.0});
    }

    pub fn parseWith(parser: *Parser) !NumberOrPercentage {
        switch (try parser.tokenizer.next()) {
            .number => |n| return .{ .value = n },
            .dimension => |d| {
                if (std.mem.eql(u8, d.unit, "percent")) {
                    return .{ .value = d.value / 100.0 };
                }
            },
            else => {},
        }

        return error.InvalidValue;
    }
};

test "NumberOrPercentage.format()" {
    try std.testing.expectFmt("0%", "{}", .{NumberOrPercentage{ .value = 0.0 }});
    try std.testing.expectFmt("10%", "{}", .{NumberOrPercentage{ .value = 0.1 }});
    try std.testing.expectFmt("100%", "{}", .{NumberOrPercentage{ .value = 1.0 }});
}

test "NumberOrPercentage.parse()" {
    try expectParse(NumberOrPercentage, "0%", .{ .value = 0.0 });
    try expectParse(NumberOrPercentage, "10%", .{ .value = 0.1 });
    try expectParse(NumberOrPercentage, "100%", .{ .value = 1.0 });

    try expectParse(NumberOrPercentage, "xxx", error.InvalidValue);
}
