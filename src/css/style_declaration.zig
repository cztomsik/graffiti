const std = @import("std");
const Property = @import("properties.zig").Property;
const Parser = @import("parser.zig").Parser;
const cssName = @import("parser.zig").cssName;
const expectFmt = std.testing.expectFmt;

pub const StyleDeclaration = struct {
    properties: std.ArrayList(Property),

    pub fn init(allocator: std.mem.Allocator) StyleDeclaration {
        return .{
            .properties = std.ArrayList(Property).init(allocator),
        };
    }

    pub fn deinit(self: *StyleDeclaration) void {
        self.properties.deinit();
    }

    pub fn format(self: StyleDeclaration, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        var sep = false;

        for (self.properties.items) |prop| {
            if (sep) try writer.writeAll("; ");
            sep = true;

            try writer.print("{s}: ", .{prop.@"0"});

            const v = prop.@"1"(&self.style);

            try switch (@typeInfo(@TypeOf(v))) {
                .Enum => writer.writeAll(@tagName(v)),
                .Float => writer.print("{d}", .{v}),
                // TODO: DimensionLike, ColorLike, ...
                else => writer.print("{any}", .{v}),
            };
        }
    }

    pub fn parse(parser: *Parser) !StyleDeclaration {
        _ = parser;
    }

    pub fn cssText(self: *StyleDeclaration) ![]const u8 {
        return std.fmt.allocPrint(self.properties.allocator, "{}", .{self});
    }

    pub fn setCssText(self: *StyleDeclaration, css_text: []const u8) !void {
        var parser = Parser.init(self.properties.allocator, css_text);
        self.* = try parser.parse(StyleDeclaration);
    }

    pub fn length(self: *StyleDeclaration) usize {
        return self.properties.items.len;
    }

    pub fn item(self: *StyleDeclaration, index: usize) []const u8 {
        if (index >= self.length()) return "";

        return "";
        // return self.properties.items[index].name();
    }

    pub fn getPropertyValue(self: *StyleDeclaration, prop_name: []const u8) []const u8 {
        _ = prop_name;
        _ = self;
    }

    pub fn setProperty(self: *StyleDeclaration, prop_name: []const u8, value: []const u8) void {
        _ = value;
        _ = prop_name;
        _ = self;
    }

    pub fn removeProperty(self: *StyleDeclaration, prop_name: []const u8) void {
        // get tag
        // if tag is not found, return
        // go through properties and if there is a prop with the same tag, remove it using orderedRemove(i)

        _ = prop_name;
        _ = self;
    }
};

test "basic usage" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    style.setProperty("display", "block");
    style.setProperty("flex-grow", "1");
    try expectFmt("display: block; flex-grow: 1", "{}", .{style});

    style.removeProperty("display");
    try expectFmt("flex-grow: 1", "{}", .{style});
}

test "overriding" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    style.setProperty("display", "block");
    style.setProperty("flex-grow", "1");
    style.setProperty("display", "none");

    try expectFmt("display: none; flex-grow: 1", "{}", .{style});
}

test "get property value" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    style.setProperty("display", "block");
    try std.testing.expectEqualStrings(style.getPropertyValue("display"), "block");

    style.removeProperty("display");
    try std.testing.expectEqualStrings(style.getPropertyValue("display"), "");
}

test "cssText" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    style.setProperty("display", "none");
    style.setProperty("flex-grow", "1");
    try std.testing.expectEqualStrings(try style.cssText(), "display: none; flex-grow: 1");

    try style.setCssText("display: block; flex-grow: 2");
    try std.testing.expectEqualStrings(try style.getPropertyValue("display"), "block");
    try std.testing.expectEqualStrings(try style.getPropertyValue("flex-grow"), "2");
}

test "item()" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    style.setProperty("display", "none");
    try std.testing.expectEqualStrings(style.item(0), "display");

    style.removeProperty("display");
    try std.testing.expectEqualStrings(style.item(0), "");
}
