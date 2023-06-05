const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;

/// A shared type for `angle`, `angle-percentage`
pub const Angle = union(enum) {
    // https://www.w3.org/TR/css-values-4/#angles
    deg: f32,
    grad: f32,
    rad: f32,
    turn: f32,

    // should be in a separate type but this is easier
    percent: f32,

    pub fn format(self: Angle, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return switch (self) {
            .percent => |v| writer.print("{d}%", .{v}),
            inline else => |v| writer.print("{d}{s}", .{ v, @tagName(self) }),
        };
    }

    pub fn parseWith(parser: *Parser) !Angle {
        switch (try parser.tokenizer.next()) {
            .number => |n| return .{ .deg = n },
            .dimension => |d| inline for (std.meta.fields(Angle)) |f| {
                if (comptime f.type == f32) {
                    if (std.mem.eql(u8, d.unit, f.name)) {
                        return @unionInit(Angle, f.name, d.value);
                    }
                }
            },
            else => {},
        }

        return error.InvalidValue;
    }

    pub fn value(self: Angle) f32 {
        var v = switch (self) {
            .deg => |v| v / 360.0,
            .grad => |v| v / 400.0,
            .rad => |v| v / std.math.tau,
            .turn => |v| v,
            .percent => |v| v / 100.0,
        };

        v = @mod(v, 1.0);
        if (v < 0.0) v += 1;

        return v;
    }
};

test "Angle.format()" {
    try std.testing.expectFmt("0deg", "{}", .{Angle{ .deg = 0 }});
    try std.testing.expectFmt("10deg", "{}", .{Angle{ .deg = 10 }});
    try std.testing.expectFmt("100%", "{}", .{Angle{ .percent = 100 }});
}

test "Angle.parse()" {
    try expectParse(Angle, "0", .{ .deg = 0 });
    try expectParse(Angle, "10", .{ .deg = 10 });
    try expectParse(Angle, "10deg", .{ .deg = 10 });
    try expectParse(Angle, "100%", .{ .percent = 100 });

    try expectParse(Angle, "xxx", error.InvalidValue);
}

test "Angle.value()" {
    try std.testing.expectEqual((Angle{ .deg = 0.0 }).value(), 0.0);
    try std.testing.expectEqual((Angle{ .deg = 36.0 }).value(), 0.1);
    try std.testing.expectEqual((Angle{ .deg = 180.0 }).value(), 0.5);
    try std.testing.expectEqual((Angle{ .deg = 360.0 }).value(), 0.0);

    try std.testing.expectEqual((Angle{ .grad = 0.0 }).value(), 0.0);
    try std.testing.expectEqual((Angle{ .grad = 40.0 }).value(), 0.1);
    try std.testing.expectEqual((Angle{ .grad = 200.0 }).value(), 0.5);
    try std.testing.expectEqual((Angle{ .grad = 400.0 }).value(), 0.0);

    try std.testing.expectEqual((Angle{ .rad = 0.0 }).value(), 0.0);
    try std.testing.expectEqual((Angle{ .rad = std.math.tau / 10.0 }).value(), 0.1);
    try std.testing.expectEqual((Angle{ .rad = std.math.tau / 2.0 }).value(), 0.5);
    try std.testing.expectEqual((Angle{ .rad = std.math.tau }).value(), 0.0);

    try std.testing.expectEqual((Angle{ .turn = 0.0 }).value(), 0.0);
    try std.testing.expectEqual((Angle{ .turn = 0.1 }).value(), 0.1);
    try std.testing.expectEqual((Angle{ .turn = 0.5 }).value(), 0.5);
    try std.testing.expectEqual((Angle{ .turn = 1.0 }).value(), 0.0);
}
