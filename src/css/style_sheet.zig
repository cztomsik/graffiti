const std = @import("std");
const Parser = @import("parser.zig").Parser;
const StyleRule = @import("style_rule.zig").StyleRule;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const StyleSheet = struct {
    rules: std.ArrayList(StyleRule),
    owner_node: ?*anyopaque = null,

    /// Creates a new, empty style sheet.
    pub fn init(allocator: std.mem.Allocator) StyleSheet {
        return StyleSheet{
            .rules = std.ArrayList(StyleRule).init(allocator),
        };
    }

    /// Deinitializes the style sheet.
    pub fn deinit(self: *StyleSheet) void {
        self.rules.deinit();
    }

    pub fn format(self: StyleSheet, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        for (self.rules) |r| {
            try writer.print("{}\n", .{r});
        }
    }

    pub fn parse(allocator: std.mem.Allocator, input: []const u8) !StyleSheet {
        var parser = Parser.init(allocator, input);
        var sheet = StyleSheet.init(allocator);
        errdefer sheet.deinit();

        while (parser.parse(StyleRule) catch null) |r| {
            try sheet.rules.append(r);
        }

        return sheet;
    }

    /// Inserts a rule at given index.
    pub fn insertRule(self: *StyleSheet, rule: []const u8, index: usize) !usize {
        if (index > self.rules.items.len) return error.IndexSizeError;

        var parser = Parser.init(self.rules.allocator, rule);
        var res = try parser.parse(StyleRule);

        try self.rules.insert(index, res);
        return index;
    }

    /// Deletes a rule at given index.
    pub fn deleteRule(self: *StyleSheet, index: usize) void {
        if (index >= self.rules.len) return error.IndexSizeError;

        self.rules.orderedRemove(index);
    }
};

// test "basic usage" {
//     var sheet = StyleSheet.init(std.testing.allocator);
//     defer sheet.deinit();

//     try sheet.insertRule("div { color: #fff }", 0);
//     try sheet.insertRule("div { color: #000 }", 0);
//     try expectFmt(sheet,
//         \\div { color: rgba(0, 0, 0, 255); }
//         \\div { color: rgba(255, 255, 255, 255); }
//     );

//     try sheet.deleteRule(0);
//     try expectFmt(sheet,
//         \\div { color: rgba(255, 255, 255, 255); }
//     );
// }

// test "parsing" {
//     var sheet = try StyleSheet.parse(std.testing.allocator, "div { color: #fff }");
//     defer sheet.deinit();

//     try std.testing.expectEqual(sheet.rules.len, 1);
//     try expectFmt(sheet,
//         \\div { color: rgba(255, 255, 255, 255); }
//     );
// }

// test "white-space" {
//     var sheet1 = try StyleSheet.parse(std.testing.allocator, " *{}");
//     defer sheet1.deinit();

//     try std.testing.expectEqual(sheet1.rules.len, 1);
//     try expectFmt(sheet1,
//         \\* {  }
//     );

//     var sheet2 = try StyleSheet.parse(std.testing.allocator, "\n*{\n}\n");
//     defer sheet2.deinit();
//     try std.testing.expectEqual(sheet2.rules.len, 1);
//     try expectFmt(sheet2,
//         \\* {  }
//     );
// }

// test "forgiving/future-compatibility" {
//     var sheet1 = try StyleSheet.parse(":root {} a { v: 0 }");
//     defer sheet1.deinit();
//     try std.testing.expectEqual(sheet1.rules.len, 2);

//     var sheet2 = try StyleSheet.parse("a {} @media { a { v: 0 } } b {}");
//     defer sheet2.deinit();
//     try std.testing.expectEqual(sheet2.rules.len, 2);

//     var sheet3 = try StyleSheet.parse("@media { a { v: 0 } } a {} b {}");
//     defer sheet3.deinit();
//     try std.testing.expectEqual(sheet3.rules.len, 2);
// }

// test "parse ua.css" {
//     const sheet = try StyleSheet.parse(std.testing.allocator, @embedFile("../../resources/ua.css"));
//     defer sheet.deinit();

//     try std.testing.expectEqual(sheet.rules.len, 23);
// }
