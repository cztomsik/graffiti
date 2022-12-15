const std = @import("std");
const Parser = @import("parser.zig").Parser;
const StyleRule = @import("style_rule.zig").StyleRule;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub fn StyleSheet(comptime T: type) type {
    return struct {
        rules: []const StyleRule(T),

        const Self = @This();

        pub fn eql(self: Self, other: Self) bool {
            if (self.rules.len != other.rules.len) return false;

            for (self.rules) |part, i| {
                if (!part.eql(other.rules[i])) {
                    return false;
                }
            }

            return true;
        }

        pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            for (self.rules) |r| {
                try writer.print("{}", .{r});
            }
        }

        pub fn parse(parser: *Parser) !Self {
            var rules = std.ArrayList(StyleRule(T)).init(parser.allocator);
            errdefer rules.deinit();

            // // anything until next "}}" (empty media is matched with unknown)
            // let media =
            //     sym("@") * sym("media") * (!seq(&["}", "}"]) * skip(1)).repeat(1..).map(|_| None) - sym("}") - sym("}");
            // // anything until next "}"
            // let unknown = (!sym("}") * skip(1)).repeat(1..).map(|_| None) - sym("}").opt();

            // (StyleRule::parser().map(Option::Some) | media | unknown)
            //     .repeat(0..)
            //     .map(|maybe_rules| Self::new(maybe_rules.into_iter().flatten().collect()))

            while (parser.parse(StyleRule(T)) catch null) |r| {
                try rules.append(r);
            }

            return Self{
                .rules = try rules.toOwnedSlice(),
            };
        }

        pub fn insertRule(self: *Self, rule: []const u8, index: usize) void {
            _ = self;
            _ = rule;
            _ = index;
            @panic("TODO");
        }

        pub fn deleteRule(self: *Self, rule: []const u8, index: usize) void {
            _ = self;
            _ = rule;
            _ = index;
            @panic("TODO");
        }
    };
}

// #[test]
// fn parse_sheet() -> Result<(), ParseError> {
//     let sheet = StyleSheet::parse("div { color: #fff }")?;

//     assert_eq!(sheet.rules()[0].selector(), &Selector::parse("div")?);
//     assert_eq!(sheet.rules()[0].style(), &Style::parse("color: #fff")?);
//     assert_eq!(sheet.rules()[0].style().to_string(), "color: rgba(255, 255, 255, 255)");

//     // white-space
//     assert_eq!(StyleSheet::parse(" *{}")?.rules().len(), 1);
//     assert_eq!(StyleSheet::parse("\n*{\n}\n")?.rules().len(), 1);

//     // forgiving/future-compatibility
//     assert_eq!(StyleSheet::parse(":root {} a { v: 0 }")?.rules().len(), 2);
//     assert_eq!(StyleSheet::parse("a {} @media { a { v: 0 } } b {}")?.rules().len(), 2);
//     assert_eq!(StyleSheet::parse("@media { a { v: 0 } } a {} b {}")?.rules().len(), 2);

//     Ok(())
// }

// #[test]
// fn parse_ua() {
//     let ua_css = include_str!("../../resources/ua.css");
//     let sheet = StyleSheet::parse(&ua_css).unwrap();

//     assert_eq!(sheet.rules().len(), 23);
// }
