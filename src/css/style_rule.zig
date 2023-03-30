const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;
const StyleDeclaration = @import("style_declaration.zig").StyleDeclaration;
const Selector = @import("selector.zig").Selector;
const expectFmt = std.testing.expectFmt;

pub const StyleRule = struct {
    selector: Selector,
    style: StyleDeclaration,

    pub fn parse(parser: *Parser) !StyleRule {
        return .{
            .selector = try parser.parse(Selector),
            .style = try parser.parse(StyleDeclaration),
        };
    }

    pub fn format(self: StyleRule, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("{} {{ {} }}", .{ self.selector, self.style_declaration });
    }
};

// test "StyleRule.format()" {
//     const Style = struct { border_radius: f32 = 0 };
//     try expectFmt("* { border-radius: 0 }", "{}", .{StyleRule(Style){
//         .selector = .{ .parts = &.{.universal} },
//         .style_declaration = .{},
//     }});
// }

// test "StyleRule.parse()" {
//     const Style = struct { opacity: f32 = 1 };
//     const Rule = StyleRule(Style);
//     try expectParse(Rule, "* { opacity: 0.5 }", .{
//         .selector = .{ .parts = &.{.universal} },
//         .style_declaration = StyleDeclaration(Style){
//             .style = .{ .opacity = 0.5 },
//         },
//     });

//     try expectParse(Rule, "", error.Eof);
//     try expectParse(Rule, "xxx", error.Eof);
// }
