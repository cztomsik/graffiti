const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;

/// A shared type for `length`, `length-percentage` and
/// `length-percentage | auto`.
pub const Dimension = union(enum) {
    auto,

    // https://www.w3.org/TR/css-values-4/#absolute-lengths
    px: f32,
    cm: f32,
    mm: f32,
    Q: f32,
    in: f32,
    pc: f32,
    pt: f32,

    // should be in a separate type but this is easier
    percent: f32,

    // https://www.w3.org/TR/css-values-4/#relative-lengths
    em: f32,
    ex: f32,
    cap: f32,
    ch: f32,
    ic: f32,
    rem: f32,
    lh: f32,
    rlh: f32,
    vw: f32,
    vh: f32,
    vi: f32,
    vb: f32,
    vmin,
    vmax,

    pub fn format(self: Dimension, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return switch (self) {
            .auto, .vmin, .vmax => writer.print("{s}", .{@tagName(self)}),
            .percent => |v| writer.print("{d}%", .{v}),
            inline else => |v| writer.print("{d}{s}", .{ v, @tagName(self) }),
        };
    }

    pub fn parse(parser: *Parser) !Dimension {
        switch (try parser.tokenizer.next()) {
            .number => |n| if (n == 0) return .{ .px = 0 },
            .dimension => |d| inline for (std.meta.fields(Dimension)) |f| {
                if (comptime f.type == f32) {
                    if (std.mem.eql(u8, d.unit, f.name)) {
                        return @unionInit(Dimension, f.name, d.value);
                    }
                }
            },
            .ident => |k| inline for (std.meta.fields(Dimension)) |f| {
                if (comptime f.type == void) {
                    if (std.mem.eql(u8, k, f.name)) {
                        return @field(Dimension, f.name);
                    }
                }
            },
            else => {},
        }

        return error.InvalidValue;
    }
};

test "Dimension.format()" {
    try std.testing.expectFmt("auto", "{}", .{@as(Dimension, Dimension.auto)});
    try std.testing.expectFmt("0px", "{}", .{Dimension{ .px = 0 }});
    try std.testing.expectFmt("10px", "{}", .{Dimension{ .px = 10 }});
    try std.testing.expectFmt("100%", "{}", .{Dimension{ .percent = 100 }});
}

test "Dimension.parse()" {
    try expectParse(Dimension, "auto", .auto);
    try expectParse(Dimension, "0", .{ .px = 0 });
    try expectParse(Dimension, "10px", .{ .px = 10 });
    try expectParse(Dimension, "100%", .{ .percent = 100 });

    try expectParse(Dimension, "xxx", error.InvalidValue);
}
