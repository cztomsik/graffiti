// subset of CSS selectors for CSS-in-JS
// TODO: string/symbol interning

const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const Specificity = u32;

pub const Selector = struct {
    parts: []const Part,

    const Part = union(enum) {
        // components
        unsupported,
        universal,
        local_name: []const u8,
        identifier: []const u8,
        class_name: []const u8,

        // combinators
        parent,
        ancestor,
        @"or",

        fn eql(self: Part, other: Part) bool {
            return std.meta.activeTag(self) == other and switch (self) {
                .local_name => std.mem.eql(u8, self.local_name, other.local_name),
                .identifier => std.mem.eql(u8, self.identifier, other.identifier),
                .class_name => std.mem.eql(u8, self.class_name, other.class_name),
                else => true,
            };
        }
    };

    const Self = @This();

    pub fn eql(self: Self, other: Self) bool {
        if (self.parts.len != other.parts.len) return false;

        for (self.parts, 0..) |part, i| {
            if (!part.eql(other.parts[i])) {
                return false;
            }
        }

        return true;
    }

    pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        var i = self.parts.len;
        while (i > 0) {
            i -= 1;

            try switch (self.parts[i]) {
                .unsupported => writer.print(":unsupported", .{}),
                .universal => writer.print("*", .{}),
                .local_name => |s| writer.print("{s}", .{s}),
                .identifier => |s| writer.print("#{s}", .{s}),
                .class_name => |s| writer.print(".{s}", .{s}),
                .parent => writer.print(" > ", .{}),
                .ancestor => writer.print(" ", .{}),
                .@"or" => writer.print(", ", .{}),
            };
        }
    }

    pub fn parse(parser: *Parser) !Self {
        var parts = std.ArrayList(Part).init(parser.allocator);
        errdefer parts.deinit();

        var combinator: ?Part = null;

        while (parser.tokenizer.next() catch null) |tok| {
            if (tok == .lcurly) break;

            const component: ?Part = switch (tok) {
                .star => Part.universal,
                .ident => |s| Part{ .local_name = s },
                .hash => |s| Part{ .identifier = s },
                .class_name => |s| Part{ .class_name = s },
                .colon => Part.unsupported,
                else => null,
            };

            if (component) |comp| {
                if (combinator) |comb| {
                    try parts.append(comb);
                } else if (parser.tokenizer.space_before) {
                    try parts.append(Part.ancestor);
                }

                try parts.append(comp);
                combinator = null;
            } else {
                if (combinator != null) return error.InvalidToken;

                combinator = switch (tok) {
                    .gt => Part.parent,
                    .comma => Part.@"or",
                    .plus => Part.unsupported,
                    .tilde => Part.unsupported,
                    else => return error.InvalidSelectorPart,
                };
            }
        }

        if (parts.items.len == 0 or combinator != null) {
            return error.Eof;
        }

        // save in reverse
        std.mem.reverse(Part, parts.items);

        return Self{
            .parts = try parts.toOwnedSlice(),
        };
    }

    pub fn matchElement(self: *Self, element: anytype) ?Specificity {
        // state
        var i: usize = 0;
        var current = element;
        var parent = false;
        var ancestors = false;
        var specificity: Specificity = 0;

        next_part: while (i < self.parts.len) : (i += 1) {
            switch (self.parts[i]) {
                .parent => parent = true,
                .ancestor => ancestors = true,
                // end-of-branch and we still have a match, no need to check others
                .@"or" => break :next_part,
                else => |comp| {
                    while (true) {
                        if (parent or ancestors) {
                            parent = false;
                            current = current.parentElement() orelse break;
                        }

                        if (Self.matchComponent(current, comp)) {
                            ancestors = false;
                            continue :next_part;
                        }

                        // we got no match on parent
                        if (!ancestors) {
                            break;
                        }
                    }

                    // no match, fast-forward to next OR
                    while (i < self.parts.len) : (i += 1) {
                        if (self.parts[i] == .@"or") {
                            // reset stack
                            current = element;
                            continue :next_part;
                        }
                    }

                    // or fail otherwise
                    return null;
                },
            }
        }

        // everything was fine
        return specificity;
    }

    fn matchComponent(el: anytype, comp: Part) bool {
        return switch (comp) {
            .universal => true,
            .local_name => |name| std.mem.eql(u8, el.localName(), name),
            .identifier => |id| std.mem.eql(u8, el.id(), id),
            .class_name => |cls| {
                var parts = std.mem.split(u8, el.className(), " ");
                while (parts.next()) |s| if (std.mem.eql(u8, s, cls)) return true;
                return false;
            },
            else => false,
        };
    }
};

test "Selector.format()" {
    try expectFmt("*", "{}", .{Selector{ .parts = &.{.universal} }});
    try expectFmt("div", "{}", .{Selector{ .parts = &.{.{ .local_name = "div" }} }});
    try expectFmt("#app", "{}", .{Selector{ .parts = &.{.{ .identifier = "app" }} }});
    try expectFmt(".btn", "{}", .{Selector{ .parts = &.{.{ .class_name = "btn" }} }});

    try expectFmt("* > *", "{}", .{Selector{ .parts = &.{ .universal, .parent, .universal } }});
    try expectFmt("* *", "{}", .{Selector{ .parts = &.{ .universal, .ancestor, .universal } }});
    try expectFmt("*, *", "{}", .{Selector{ .parts = &.{ .universal, .@"or", .universal } }});
}

fn expectParts(selector: []const u8, parts: []const Selector.Part) !void {
    try expectParse(Selector, selector, Selector{ .parts = parts });
}

test "parsing" {
    // simple
    try expectParts("*", &.{.universal});
    try expectParts("body", &.{.{ .local_name = "body" }});
    try expectParts("h2", &.{.{ .local_name = "h2" }});
    try expectParts("#app", &.{.{ .identifier = "app" }});
    try expectParts(".btn", &.{.{ .class_name = "btn" }});

    // combined
    try expectParts(".btn.btn-primary", &.{ .{ .class_name = "btn-primary" }, .{ .class_name = "btn" } });
    try expectParts("*.test", &.{ .{ .class_name = "test" }, .universal });
    try expectParts("div#app.test", &.{ .{ .class_name = "test" }, .{ .identifier = "app" }, .{ .local_name = "div" } });

    // combined with combinators
    try expectParts("div .btn", &.{ .{ .class_name = "btn" }, .ancestor, .{ .local_name = "div" } });
    try expectParts("body > div.test div#test", &.{
        .{ .identifier = "test" },
        .{ .local_name = "div" },
        .ancestor,
        .{ .class_name = "test" },
        .{ .local_name = "div" },
        .parent,
        .{ .local_name = "body" },
    });

    // multi
    try expectParts("html, body", &.{ .{ .local_name = "body" }, .@"or", .{ .local_name = "html" } });
    try expectParts("body > div, div button span", &.{
        .{ .local_name = "span" },
        .ancestor,
        .{ .local_name = "button" },
        .ancestor,
        .{ .local_name = "div" },
        .@"or",
        .{ .local_name = "div" },
        .parent,
        .{ .local_name = "body" },
    });

    // unsupported for now
    try expectParts(":root", &.{ .{ .local_name = "root" }, .unsupported });
    try expectParts("* + *", &.{ .universal, .unsupported, .universal });
    try expectParts("* ~ *", &.{ .universal, .unsupported, .universal });

    // this should be invalid but it makes rule parsing easier
    try expectParts("* {", &.{.universal});

    // invalid
    try expectParse(Selector, "", error.Eof);
    try expectParse(Selector, " ", error.Eof);
    try expectParse(Selector, "foo + {", error.Eof);
    try expectParse(Selector, "a,,b", error.InvalidToken);
    try expectParse(Selector, "a>>b", error.InvalidToken);

    // TODO: bugs & edge-cases
    // try expectParts("input[type=\"submit\"]", &.{ Unsupported, LocalName("input")]);
}

fn expectMatch(selector: []const u8, index: usize) !void {
    var parser = Parser.init(std.testing.allocator, selector);
    var sel = try parser.parse(Selector);
    defer std.testing.allocator.free(sel.parts);

    const parents = [_]?usize{ null, 0, 1, 2, 3 };
    const local_names = [_][]const u8{ "html", "body", "div", "button", "span" };
    const ids = [_][]const u8{ "", "app", "panel", "", "" };
    const class_names = [_][]const u8{ "", "", "", "btn", "" };

    const Element = struct {
        index: usize,

        const Self = @This();

        fn parentElement(self: Self) ?Self {
            return if (parents[self.index]) |parent| .{ .index = parent } else null;
        }

        fn localName(self: Self) []const u8 {
            return local_names[self.index];
        }

        fn id(self: Self) []const u8 {
            return ids[self.index];
        }

        fn className(self: Self) []const u8 {
            return class_names[self.index];
        }
    };

    try std.testing.expect(sel.matchElement(Element{ .index = index }) != null);
}

test "matching" {
    // basic
    try expectMatch("*", 0);
    try expectMatch("html", 0);
    try expectMatch("body", 1);
    try expectMatch("#app", 1);
    try expectMatch("div", 2);
    try expectMatch("#panel", 2);
    try expectMatch("button", 3);
    try expectMatch(".btn", 3);
    try expectMatch("span", 4);

    // combined
    try expectMatch("body#app", 1);
    try expectMatch("div#panel", 2);
    try expectMatch("button.btn", 3);

    // parent
    try expectMatch("button > span", 4);
    try expectMatch("div#panel > button.btn > span", 4);

    // ancestor
    try expectMatch("button span", 4);
    try expectMatch("div#panel span", 4);
    try expectMatch("html body div button span", 4);
    try expectMatch("body div .btn span", 4);

    // OR
    try expectMatch("div, span", 4);
    try expectMatch("a, b, c, span, d", 4);
    try expectMatch("html, body", 1);

    // complex
    try expectMatch("div, span.foo, #panel span", 4);
    try expectMatch("a b c d e f g, span", 4);
}
