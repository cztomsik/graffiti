const std = @import("std");
const Parser = @import("../parser.zig").Parser;
const expectParse = @import("../parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const Dimension = union(enum) {
    auto,
    px: f32,
    percent: f32,
    em: f32,
    rem: f32,
    vw: f32,
    vh: f32,
    vmin,
    vmax,

    const Self = @This();

    pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return switch (self) {
            .px => |v| writer.print("{d}px", .{v}),
            .percent => |v| writer.print("{d}%", .{v}),
            .em => |v| writer.print("{d}em", .{v}),
            .rem => |v| writer.print("{d}rem", .{v}),
            .vw => |v| writer.print("{d}vw", .{v}),
            .vh => |v| writer.print("{d}vh", .{v}),
            .auto, .vmin, .vmax => writer.print("{s}", .{@tagName(self)}),
        };
    }

    pub fn parse(parser: *Parser) !Self {
        const tok = try parser.tokenizer.next();

        // https://github.com/ziglang/zig/issues/6749
        const D = Self;

        switch (tok) {
            .number => |n| if (n == 0) return D{ .px = 0 },
            .percentage => |p| return D{ .percent = p },
            .dimension => |d| {
                if (std.mem.eql(u8, "px", d.unit)) return D{ .px = d.value };
                if (std.mem.eql(u8, "em", d.unit)) return D{ .em = d.value };
                if (std.mem.eql(u8, "rem", d.unit)) return D{ .rem = d.value };
                if (std.mem.eql(u8, "vw", d.unit)) return D{ .vw = d.value };
                if (std.mem.eql(u8, "vh", d.unit)) return D{ .vh = d.value };
            },
            .ident => |k| {
                if (std.mem.eql(u8, "auto", k)) return D.auto;
                if (std.mem.eql(u8, "vmin", k)) return D.vmin;
                if (std.mem.eql(u8, "vmax", k)) return D.vmax;
            },
            else => {},
        }

        return error.invalid;
    }
};

test "Dimension.format()" {
    try expectFmt("0px", "{}", .{Dimension{ .px = 0 }});
    try expectFmt("100%", "{}", .{Dimension{ .percent = 100 }});
    try expectFmt("1.25em", "{}", .{Dimension{ .em = 1.25 }});
    try expectFmt("1.25rem", "{}", .{Dimension{ .rem = 1.25 }});
    try expectFmt("1.25vw", "{}", .{Dimension{ .vw = 1.25 }});
    try expectFmt("1.25vh", "{}", .{Dimension{ .vh = 1.25 }});
    // TODO: check in stage2? report bug? zig is ignoring .format() probably because it inlines .auto as comptime?
    try expectFmt("auto", "{}", .{@as(Dimension, Dimension.auto)});
    try expectFmt("vmin", "{}", .{@as(Dimension, Dimension.vmin)});
    try expectFmt("vmax", "{}", .{@as(Dimension, Dimension.vmax)});
}

test "Dimension.parse()" {
    try expectParse(Dimension, "0", .{ .px = 0 });
    try expectParse(Dimension, "100%", .{ .percent = 100 });
    try expectParse(Dimension, "10px", .{ .px = 10 });
    try expectParse(Dimension, "1.2em", .{ .em = 1.2 });
    try expectParse(Dimension, "2.1rem", .{ .rem = 2.1 });
    try expectParse(Dimension, "100vw", .{ .vw = 100 });
    try expectParse(Dimension, "100vh", .{ .vh = 100 });
    try expectParse(Dimension, "auto", .auto);
    try expectParse(Dimension, "vmin", .vmin);
    try expectParse(Dimension, "vmax", .vmax);

    try expectParse(Dimension, "xxx", error.invalid);
}
