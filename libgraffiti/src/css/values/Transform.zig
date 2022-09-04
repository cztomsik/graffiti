const std = @import("std");
const Parser = @import("../parser.zig").Parser;

pub const Transform = union(enum) {
    translate: [2]f32,
    scale: [2]f32,
    rotate: f32,

    const Self = @This();

    pub fn parse(parser: *Parser) !Self {
        _ = parser;
        @panic("TODO");
    }
};
