// subset of CSS selectors for CSS-in-JS
// TODO: string/symbol interning

const std = @import("std");
const Parser = @import("parser.zig").Parser;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const Specificity = u32;

pub const Selector = struct {
    parts: []const Part,

    const Self = @This();

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
            if (std.meta.activeTag(self) != std.meta.activeTag(other)) return false;

            return switch (self) {
                .local_name => std.mem.eql(u8, self.local_name, other.local_name),
                .identifier => std.mem.eql(u8, self.identifier, other.identifier),
                .class_name => std.mem.eql(u8, self.class_name, other.class_name),
                else => true,
            };
        }
    };

    pub fn eql(self: Self, other: Self) bool {
        if (self.parts.len != other.parts.len) return false;

        for (self.parts) |part, i| {
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

        while (parser.tokenizer.next() catch null) |tok| {
            try parts.append(switch (tok) {
                .star => Part.universal,
                .ident => |name| Part{ .local_name = name },
                .hash => |id| Part{ .identifier = id },
                .dot => Part{ .class_name = try parser.expect(.ident) },
                .colon => blk: {
                    _ = try parser.expect(.ident);
                    break :blk Part.unsupported;
                },

                .gt => Part.parent,
                .space => Part.ancestor,
                .comma => Part.@"or",
                .plus => Part.unsupported,
                .tilde => Part.unsupported,
                else => return error.InvalidToken,
            });
        }

        std.mem.reverse(Part, parts.items);
        return Self{ .parts = parts.toOwnedSlice() };
    }

    // pub fn match_element<C: MatchingContext>(&self, element: C::ElementRef, ctx: &C) -> Option<Specificity> {
    //     // so we can fast-forward to next OR
    //     var parts_iter = self.parts.iter();

    //     // state
    //     var current = element;
    //     var parent = false;
    //     var ancestors = false;
    //     let specificity = Specificity(0);

    //     // we are always going forward
    //     'next_part: while let Some(p) = parts_iter.next() {
    //         match p {
    //             SelectorPart::Combinator(comb) => {
    //                 match comb {
    //                     // state changes
    //                     Combinator::Parent => parent = true,
    //                     Combinator::Ancestor => ancestors = true,

    //                     // end-of-branch and we still have a match, no need to check others
    //                     Combinator::Or => break 'next_part,
    //                 }
    //             }

    //             comp => {
    //                 loop {
    //                     if parent || ancestors {
    //                         parent = false;

    //                         match ctx.parent_element(current) {
    //                             Some(parent) => current = parent,

    //                             // nothing left to match
    //                             None => break,
    //                         }
    //                     }

    //                     if Self::match_component(current, comp, ctx) {
    //                         ancestors = false;
    //                         continue 'next_part;
    //                     }

    //                     // we got no match on parent
    //                     if !ancestors {
    //                         break;
    //                     }
    //                 }

    //                 // no match, fast-forward to next OR
    //                 for p in parts_iter.by_ref() {
    //                     if p == &SelectorPart::Combinator(Combinator::Or) {
    //                         // reset stack
    //                         current = element;
    //                         continue 'next_part;
    //                     }
    //                 }

    //                 // or fail otherwise
    //                 return None;
    //             }
    //         }
    //     }

    //     // everything was fine
    //     Some(specificity)
    // }

    // fn match_component<C: MatchingContext>(el: C::ElementRef, comp: &SelectorPart, ctx: &C) -> bool {
    //     match comp {
    //         SelectorPart::Universal => true,
    //         SelectorPart::LocalName(name) => ctx.local_name(el) == &**name,
    //         SelectorPart::Identifier(id) => ctx.attribute(el, "id") == Some(id),
    //         SelectorPart::ClassName(cls) => match ctx.attribute(el, "class") {
    //             Some(s) => s.split_ascii_whitespace().any(|part| part == &**cls),
    //             _ => false,
    //         },
    //         SelectorPart::Unsupported => false,
    //         SelectorPart::Combinator(_) => unreachable!(),
    //     }
    // }

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
    try expectParts(":root", &.{.unsupported});
    try expectParts("* + *", &.{ .universal, .unsupported, .universal });
    try expectParts("* ~ *", &.{ .universal, .unsupported, .universal });

    // invalid
    // TODO
    // try expectParse(Selector, "", error.Eof);
    // try expectParse(Selector, " ", error.Eof);
    // try expectParse(Selector, "a,,b", error.invalid);
    // try expectParse(Selector, "a>>b", error.invalid);

    // bugs & edge-cases
    // try expectParts("input[type=\"submit\"]", &.{ Unsupported, LocalName("input")]);
}

fn expectMatch(selector: []const u8, index: usize) !void {
    // TODO
    // parents: [null, 0, 1, 2, 3]
    // local_names: ["html", "body", "div", "button", "span"];
    // ids: ["", "app", "panel", "", ""]
    // class_names: ["", "", "", "btn", ""]

    _ = selector;
    _ = index;

    return;
}

test "matching" {
    // invalid
    // std.testing.expect(Selector::unsupported().match_element(0, &Ctx).is_none());

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
    try expectMatch("body div .btn span", 4);

    // OR
    try expectMatch("div, span", 4);
    try expectMatch("a, b, c, span, d", 4);
    try expectMatch("html, body", 1);

    // complex
    try expectMatch("div, span.foo, #panel span", 4);
    try expectMatch("a b c d e f g, span", 4);
}
