// TODO: we will probably need to a different way of storing the properties
//       because some of the props are slices/arrays and even if we used just
//       fixed size arrays, the size of the Property enum would be huge.
//       BTW: it's already unnecessarily big so it would probably make sense
//       to do it even if we didn't have the slices/arrays.

const std = @import("std");
const Property = @import("properties.zig").Property;
const Shorthand = @import("properties.zig").Shorthand;
const Parser = @import("parser.zig").Parser;
const cssName = @import("parser.zig").cssName;
const expectFmt = std.testing.expectFmt;

const PropertyId = std.meta.Tag(Property);

/// Collection of longhand properties to be applied to an element.
/// Shorthand properties are expanded into their longhand components.
pub const StyleDeclaration = struct {
    properties: std.ArrayList(Property),

    /// Init a new instance.
    pub fn init(allocator: std.mem.Allocator) StyleDeclaration {
        return .{
            .properties = std.ArrayList(Property).init(allocator),
        };
    }

    /// Release all resources.
    pub fn deinit(self: *StyleDeclaration) void {
        self.properties.deinit();
    }

    /// Returns the serialized CSS text representation.
    pub fn cssText(self: *StyleDeclaration, allocator: std.mem.Allocator) ![]const u8 {
        return std.fmt.allocPrint(allocator, "{}", .{self});
    }

    /// Replace all properties with the ones parsed from the given CSS text.
    pub fn setCssText(self: *StyleDeclaration, css_text: []const u8) !void {
        var parser = Parser.init(self.properties.allocator, css_text);
        self.deinit();
        self.* = try parser.parse(StyleDeclaration);
    }

    /// Returns the number of longhand properties.
    pub fn length(self: *StyleDeclaration) u32 {
        return @truncate(u32, self.properties.items.len);
    }

    /// Returns the name of the property at the given index.
    pub fn item(self: *StyleDeclaration, index: u32) []const u8 {
        if (index >= self.length()) return "";

        return propName(self.properties.items[index]);
    }

    /// Returns the serialized value of the property at the given index,
    /// or an empty string if the index is out of bounds.
    /// Shorthand properties are supported if all of their components are present.
    pub fn getPropertyValue(self: *StyleDeclaration, allocator: std.mem.Allocator, prop_name: []const u8) ![]const u8 {
        inline for (std.meta.fields(Property)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                const prop = self.properties.items[self.find(@field(PropertyId, f.name)) orelse return ""];
                return std.fmt.allocPrint(allocator, "{}", .{fmtPropValue(prop)});
            }
        }

        inline for (std.meta.fields(Shorthand)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                var shorthand: f.type = undefined;

                inline for (std.meta.fields(f.type)) |sf| {
                    const prop = self.properties.items[self.find(@field(PropertyId, sf.name)) orelse return ""];
                    @field(shorthand, sf.name) = @field(prop, sf.name);
                }

                return std.fmt.allocPrint(allocator, "{}", .{shorthand});
            }
        }

        return "";
    }

    /// Sets the value of the property with the given name.
    /// Shorthand properties will be expanded.
    pub fn setProperty(self: *StyleDeclaration, prop_name: []const u8, value: []const u8) !void {
        var parser = Parser.init(self.properties.allocator, value);

        inline for (std.meta.fields(Property)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                const val = try parser.parse(f.type);
                try self.add(@unionInit(Property, f.name, val));
            }
        }

        inline for (std.meta.fields(Shorthand)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                const shorthand = try parser.parse(f.type);

                inline for (std.meta.fields(f.type)) |sf| {
                    try self.add(@unionInit(
                        Property,
                        sf.name,
                        @field(shorthand, sf.name),
                    ));
                }
            }
        }
    }

    /// Removes the property with the specified name.
    /// Shorthand properties will be expanded (all properties will be removed).
    pub fn removeProperty(self: *StyleDeclaration, prop_name: []const u8) void {
        inline for (std.meta.fields(Property)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                self.remove(@field(PropertyId, f.name));
            }
        }

        inline for (std.meta.fields(Shorthand)) |f| {
            if (std.mem.eql(u8, cssName(f.name), prop_name)) {
                inline for (std.meta.fields(f.type)) |f2| {
                    self.remove(@field(PropertyId, f2.name));
                }
            }
        }
    }

    pub fn format(self: StyleDeclaration, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        for (self.properties.items, 0..) |prop, i| {
            if (i > 0) try writer.writeAll("; ");

            try writer.print("{s}: {}", .{ propName(prop), fmtPropValue(prop) });
        }
    }

    pub fn parse(parser: *Parser) !StyleDeclaration {
        var res = StyleDeclaration.init(parser.allocator);

        while (parser.expect(.ident) catch null) |prop_name| {
            try parser.expect(.colon);

            const val_start = parser.tokenizer.pos;
            while (parser.tokenizer.next() catch null) |t2| if (t2 == .semi or t2 == .rcurly) break;

            res.setProperty(prop_name, parser.tokenizer.input[val_start..parser.tokenizer.pos]) catch continue;
        }

        return res;
    }

    pub fn apply(self: *StyleDeclaration, target: anytype) void {
        for (self.properties.items) |p| {
            inline for (std.meta.fields(Property)) |f| {
                if (p == @field(PropertyId, f.name)) {
                    @field(target, f.name) = @field(p, f.name);
                }
            }
        }
    }

    // helpers

    fn propName(prop: Property) []const u8 {
        inline for (std.meta.fields(Property)) |f| {
            if (prop == @field(PropertyId, f.name)) return cssName(f.name);
        } else unreachable;
    }

    fn find(self: *StyleDeclaration, tag: PropertyId) ?usize {
        for (self.properties.items, 0..) |prop, i| {
            if (prop == tag) return i;
        }

        return null;
    }

    fn add(self: *StyleDeclaration, prop: Property) !void {
        if (self.find(prop)) |i| {
            self.properties.items[i] = prop;
            return;
        }

        try self.properties.append(prop);
    }

    fn remove(self: *StyleDeclaration, tag: PropertyId) void {
        if (self.find(tag)) |i| {
            _ = self.properties.orderedRemove(i);
        }
    }
};

fn fmtPropValue(prop: Property) std.fmt.Formatter(formatPropValue) {
    return .{ .data = prop };
}

fn formatPropValue(prop: Property, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
    switch (prop) {
        inline else => |v| try switch (@typeInfo(@TypeOf(v))) {
            .Enum => writer.writeAll(@tagName(v)),
            .Float => writer.print("{d}", .{v}),
            else => writer.print("{any}", .{v}),
        },
    }
}

test "basic usage" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    try style.setProperty("display", "flex");
    try style.setProperty("flex-grow", "1");
    try expectFmt("display: flex; flex-grow: 1", "{}", .{style});

    try style.setProperty("display", "none");
    try expectFmt("display: none; flex-grow: 1", "{}", .{style});

    style.removeProperty("display");
    try expectFmt("flex-grow: 1", "{}", .{style});

    try style.setProperty("flex", "2");
    // try expectFmt("flex-grow: 2; flex-shrink: 1; flex-basis: 0%", "{}", .{style});

    style.removeProperty("flex");
    try expectFmt("", "{}", .{style});
}

test "getPropertyValue()" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();

    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    try style.setProperty("display", "flex");
    try std.testing.expectEqualStrings("flex", try style.getPropertyValue(arena.allocator(), "display"));

    style.removeProperty("display");
    try std.testing.expectEqualStrings("", try style.getPropertyValue(arena.allocator(), "display"));

    try style.setProperty("flex", "2");
    // try std.testing.expectEqualStrings("2 1 0%", try style.getPropertyValue(arena.allocator(), "flex"));

    style.removeProperty("flex-grow");
    try std.testing.expectEqualStrings("", try style.getPropertyValue(arena.allocator(), "flex"));
}

test "cssText" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();

    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    try style.setProperty("display", "flex");
    try style.setProperty("flex-grow", "1");

    try std.testing.expectEqualStrings("display: flex; flex-grow: 1", try style.cssText(arena.allocator()));

    try style.setCssText("display: none; flex-grow: 2");
    try std.testing.expectEqualStrings("none", try style.getPropertyValue(arena.allocator(), "display"));
    try std.testing.expectEqualStrings("2", try style.getPropertyValue(arena.allocator(), "flex-grow"));
}

test "length(), item()" {
    var style = StyleDeclaration.init(std.testing.allocator);
    defer style.deinit();

    try style.setProperty("display", "none");
    try std.testing.expectEqual(style.length(), 1);
    try std.testing.expectEqualStrings("display", style.item(0));

    style.removeProperty("display");
    try std.testing.expectEqual(style.length(), 0);
    try std.testing.expectEqualStrings("", style.item(0));

    try style.setProperty("flex", "1");
    try std.testing.expectEqual(style.length(), 3);
    try std.testing.expectEqualStrings("flex-grow", style.item(0));
    try std.testing.expectEqualStrings("flex-shrink", style.item(1));
    try std.testing.expectEqualStrings("flex-basis", style.item(2));

    style.removeProperty("flex");
    try std.testing.expectEqual(style.length(), 0);
}
