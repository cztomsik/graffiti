// subset of CSS selectors for CSS-in-JS
// TODO: string/symbol interning

const std = @import("std");
const Parser = @import("parser.zig").Parser;

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
    };

    pub fn parse(parser: *Parser) !Self {
        _ = parser;

        return Self{ .parts = &.{} };
    }

    // impl Parsable for Selector {
    //     fn parser<'a>() -> Parser<'a, Self> {
    //         let tag = || {
    //             let ident = || ident().map(Atom::from);
    //             let universal = sym("*").map(|_| SelectorPart::Universal);
    //             let local_name = ident().map(SelectorPart::LocalName);
    //             let id = sym("#") * ident().map(SelectorPart::Identifier);
    //             let class_name = sym(".") * ident().map(SelectorPart::ClassName);

    //             universal | local_name | id | class_name
    //         };

    //         // note we parse child/descendant but we flip the final order so it's parent/ancestor
    //         let child = sym(">").map(|_| Combinator::Parent);
    //         let descendant = sym(" ").map(|_| Combinator::Ancestor);
    //         let or = sym(",").map(|_| Combinator::Or);
    //         let comb = (child | descendant | or).map(SelectorPart::Combinator) | unsupported;

    //         let selector = tag() + (comb.opt() + tag()).repeat(0..);

    //         selector.map(|(head, tail)| {
    //             let mut parts = Vec::with_capacity(tail.len() + 1);

    //             // reversed (child/descendant -> parent/ancestor)
    //             for (comb, tag) in tail.into_iter().rev() {
    //                 parts.push(tag);

    //                 if let Some(comb) = comb {
    //                     parts.push(comb);
    //                 }
    //             }

    //             parts.push(head);

    //             Selector { parts }
    //         })
    //     }
    // }

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

fn expectParts(selector: []const u8, parts: []const Selector.Part) !void {
    const sel = try Parser.init(std.testing.allocator, selector).parse(Selector);

    try std.testing.expectEqualSlices(Selector.Part, parts, sel.parts);
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

    // // invalid
    // assert!(Selector::parse("").is_err());
    // assert!(Selector::parse(" ").is_err());
    // assert!(Selector::parse("a,,b").is_err());
    // assert!(Selector::parse("a>>b").is_err());

    // // bugs & edge-cases
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
