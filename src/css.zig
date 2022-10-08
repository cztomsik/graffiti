const std = @import("std");
const expectParse = @import("css/parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const Tokenizer = @import("css/tokenizer.zig");
pub const Parser = @import("css/parser.zig").Parser;

// TODO: StyleSheet(T), StyleRule(T)

// a "script" of arbitrary field assignments (for a given struct type)
pub fn DeclarationBlock(comptime T: type) type {
    const StyleProp = Prop(T);

    return struct {
        props: []const StyleProp = &.{},

        const Self = @This();

        pub fn eql(self: Self, other: Self) bool {
            if (self.props.len != other.props.len) return false;

            for (self.props) |prop, i| {
                if (!std.meta.eql(prop, other.props[i])) {
                    return false;
                }
            }

            return true;
        }

        pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            for (self.props) |p, i| {
                if (i != 0) try writer.writeAll("; ");
                try formatProp(p, writer);
            }
        }

        fn formatProp(prop: StyleProp, writer: anytype) !void {
            inline for (std.meta.fields(StyleProp)) |f| {
                if (prop == @field(StyleProp, f.name)) {
                    const v = @field(prop, f.name);

                    try writer.print("{s}: ", .{@tagName(prop)});

                    return switch (@typeInfo(f.field_type)) {
                        .Enum => writer.writeAll(@tagName(v)),
                        .Float => writer.print("{d}", .{v}),
                        else => writer.print("{any}", .{v}),
                    };
                }
            }
        }

        pub fn parse(parser: *Parser) !Self {
            var props = std.ArrayList(StyleProp).init(parser.allocator);
            errdefer props.deinit();

            while (true) {
                // TODO: shorthands
                try props.append(parseProp(parser) catch |e| {
                    if ((e == error.Eof) or ((parser.tokenizer.peek(0) catch 0) == '}')) break else continue;
                });
            }

            return Self{
                .props = props.toOwnedSlice(),
            };
        }

        fn parseProp(parser: *Parser) !StyleProp {
            const prop_name = try parser.expect(.ident);

            try parser.expect(.colon);

            inline for (std.meta.fields(StyleProp)) |f| {
                if (std.mem.eql(u8, prop_name, Parser.cssName(f.name))) {
                    const value = try parser.parse(f.field_type);
                    return @unionInit(StyleProp, f.name, value);
                }
            }

            return error.UnknownProperty;
        }

        pub fn apply(self: Self, target: *T) void {
            for (self.props) |p| {
                inline for (std.meta.fields(T)) |f| {
                    if (p == @field(Prop(T), f.name)) {
                        @field(target, f.name) = @field(p, f.name);
                    }
                }
            }
        }
    };
}

fn Prop(comptime T: type) type {
    const fields = std.meta.fields(T);
    var union_fields: [fields.len]std.builtin.Type.UnionField = undefined;
    inline for (fields) |f, i| {
        union_fields[i] = .{
            .name = f.name,
            .field_type = f.field_type,
            .alignment = f.alignment,
        };
    }

    return @Type(.{
        .Union = .{
            .layout = .Auto,
            .tag_type = std.meta.FieldEnum(T),
            .fields = &union_fields,
            .decls = &.{},
        },
    });
}

test {
    std.testing.refAllDecls(@This());
}

const TestStyle = struct { display: enum { none, block } = .block, opacity: f32 = 1, flex_grow: f32 = 0 };
const Decl = DeclarationBlock(TestStyle);

test "DeclarationBlock.format()" {
    try expectFmt("display: block; opacity: 1", "{}", .{Decl{ .props = &.{
        .{ .display = .block },
        .{ .opacity = 1 },
    } }});
}

test "DeclarationBlock.parse()" {
    try expectParse(Decl, "", Decl{});
    try expectParse(Decl, "unknown-a: 0; unknown-b: 0", Decl{});

    try expectParse(Decl, "opacity: 0", Decl{ .props = &.{.{ .opacity = 0 }} });
    try expectParse(Decl, "opacity: 0; opacity: invalid-ignored", Decl{ .props = &.{.{ .opacity = 0 }} });

    try expectParse(Decl, "opacity: 0; flex-grow: 1", Decl{ .props = &.{
        .{ .opacity = 0 },
        .{ .flex_grow = 1 },
    } });
}

test "DeclarationBlock.apply()" {
    var target = TestStyle{};
    const decl: Decl = .{ .props = &.{.{ .opacity = 0.5 }} };
    decl.apply(&target);
    try std.testing.expectEqual(target.opacity, 0.5);
}
