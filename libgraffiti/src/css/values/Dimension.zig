const std = @import("std");
const Parser = @import("../parser.zig").Parser;

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

fn expectDimension(input: []const u8, expected: Dimension) !void {
    try std.testing.expectEqual(expected, try Parser.init(std.testing.allocator, input).parse(Dimension));
}

test "Dimension.parse()" {
    try expectDimension("0", Dimension{ .px = 0 });
    try expectDimension("100%", Dimension{ .percent = 100 });
    try expectDimension("10px", Dimension{ .px = 10 });
    try expectDimension("1.2em", Dimension{ .em = 1.2 });
    try expectDimension("2.1rem", Dimension{ .rem = 2.1 });
    try expectDimension("100vw", Dimension{ .vw = 100 });
    try expectDimension("100vh", Dimension{ .vh = 100 });
    try expectDimension("auto", Dimension.auto);
    try expectDimension("vmin", Dimension.vmin);
    try expectDimension("vmax", Dimension.vmax);
}
