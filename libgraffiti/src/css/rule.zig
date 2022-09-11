const std = @import("std");
const Parser = @import("parser.zig").Parser;
const Selector = @import("selector.zig").Selector;
const Style = @import("style.zig").Style;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const StyleRule = struct {
    selector: Selector,
    style: Style,

    const Self = @This();

    pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("{} {{ {} }}", .{ self.selector, self.style });
    }

    pub fn parse(parser: *Parser) !Self {
        const selector = try parser.parse(Selector);

        try parser.expect(.lcurly);

        const style = try parser.parse(Style);

        try parser.expect(.rcurly);

        return .{
            .selector = selector,
            .style = style,
        };
    }
};

test "StyleRule.format()" {
    try expectFmt("* {  }", "{}", .{StyleRule{ .selector = Selector.UNIVERSAL, .style = .{} }});
}

test "StyleRule.parse()" {
    try expectParse(StyleRule, "* { opacity: 0 }", StyleRule{ .selector = Selector.UNIVERSAL, .style = Style{ .props = &.{.{ .opacity = 0 }} } });

    try expectParse(StyleRule, "", error.Eof);
    try expectParse(StyleRule, "xxx", error.Eof);
}
