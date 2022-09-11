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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_rule() -> Result<(), ParseError> {
//         let selector = Selector::parse("div")?;
//         let style = Style::parse("color: #fff")?;

//         assert_eq!(StyleRule::parse("div { color: #fff }")?, StyleRule { selector, style });

//         Ok(())
//     }
// }
