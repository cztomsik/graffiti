const std = @import("std");
const Parser = @import("../parser.zig").Parser;

pub const BoxShadow = struct {
    offset: [2]Px,
    blur: Px,
    spread: Px,
    color: Color,

    const Self = @This();

    pub fn parse(parser: *Parser) !Self {
        _ = parser;
        @panic("TODO");
    }
};
