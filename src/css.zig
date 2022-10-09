const std = @import("std");
const expectParse = @import("css/parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

pub const Tokenizer = @import("css/tokenizer.zig");
pub const Parser = @import("css/parser.zig").Parser;

// TODO: StyleSheet(T), StyleRule(T)

// essentially, this is just a script of operations to be applied to a given struct type
// and those operations are just field assignments
// TODO: consider !important, inherit, initial
pub fn DeclarationBlock(comptime T: type) type {
    const Declaration = DeclarationUnion(T);

    return struct {
        declarations: []const Declaration = &.{},

        const Self = @This();

        pub fn eql(self: Self, other: Self) bool {
            if (self.declarations.len != other.declarations.len) return false;

            for (self.declarations) |decl, i| {
                if (!std.meta.eql(decl, other.declarations[i])) {
                    return false;
                }
            }

            return true;
        }

        pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            for (self.declarations) |p, i| {
                if (i != 0) try writer.writeAll("; ");
                try formatDeclaration(p, writer);
            }
        }

        fn formatDeclaration(declaration: Declaration, writer: anytype) !void {
            inline for (std.meta.fields(Declaration)) |f| {
                if (declaration == @field(Declaration, f.name)) {
                    const v = @field(declaration, f.name);

                    try writer.print("{s}: ", .{@tagName(declaration)});

                    return switch (@typeInfo(f.field_type)) {
                        .Enum => writer.writeAll(@tagName(v)),
                        .Float => writer.print("{d}", .{v}),
                        else => writer.print("{any}", .{v}),
                    };
                }
            }
        }

        pub fn parse(parser: *Parser) !Self {
            var declarations = std.ArrayList(Declaration).init(parser.allocator);
            errdefer declarations.deinit();

            while (true) {
                // TODO: maybe DeclarationBlock.fromKeyValue(name_str, val_str)?
                //       or I don't know, maybe Declaration should be private anyway
                //       (if we ever want to do encoding, then I don't know how the public struct would look like
                //        and even then I don't know, declarations feels like something internal... the problem
                //        is that we need some way to create "one-prop" declaration for el.style.setProperty())
                try declarations.append(parseDeclaration(parser) catch |e| {
                    if ((e == error.Eof) or ((parser.tokenizer.peek(0) catch 0) == '}')) break else continue;
                });
            }

            return Self{
                .declarations = declarations.toOwnedSlice(),
            };
        }

        fn parseDeclaration(parser: *Parser) !Declaration {
            const prop_name = try parser.expect(.ident);
            try parser.expect(.colon);

            return parseDeclarationByName(parser, prop_name);
        }

        // TODO: public only because of el.style.setProperty()
        pub fn parseDeclarationByName(parser: *Parser, prop_name: []const u8) !Declaration {
            inline for (std.meta.fields(Declaration)) |f| {
                if (propNameEql(f.name, prop_name)) {
                    const value = try parser.parse(f.field_type);
                    return @unionInit(Declaration, f.name, value);
                }
            }

            return error.UnknownProperty;
        }

        pub fn apply(self: Self, target: *T) void {
            for (self.declarations) |decl| {
                inline for (std.meta.fields(T)) |f| {
                    if (decl == @field(Declaration, f.name)) {
                        @field(target, f.name) = @field(decl, f.name);
                    }
                }
            }
        }
    };
}

// operation to be performed (field-value assignment for now)
fn DeclarationUnion(comptime T: type) type {
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
const TestBlock = DeclarationBlock(TestStyle);

test "DeclarationBlock.format()" {
    try expectFmt("display: block; opacity: 1", "{}", .{TestBlock{ .declarations = &.{
        .{ .display = .block },
        .{ .opacity = 1 },
    } }});
}

test "DeclarationBlock.parse()" {
    try expectParse(TestBlock, "", TestBlock{});
    try expectParse(TestBlock, "unknown-a: 0; unknown-b: 0", TestBlock{});

    try expectParse(TestBlock, "opacity: 0", TestBlock{ .declarations = &.{.{ .opacity = 0 }} });
    try expectParse(TestBlock, "opacity: 0; opacity: invalid-ignored", TestBlock{ .declarations = &.{.{ .opacity = 0 }} });

    try expectParse(TestBlock, "opacity: 0; flex-grow: 1", TestBlock{ .declarations = &.{
        .{ .opacity = 0 },
        .{ .flex_grow = 1 },
    } });
}

test "DeclarationBlock.apply()" {
    var target = TestStyle{};
    const block: TestBlock = .{ .declarations = &.{.{ .opacity = 0.5 }} };
    block.apply(&target);
    try std.testing.expectEqual(target.opacity, 0.5);
}

// a.toLowerCase().replace(/-_/g, '') == ...(b)
pub fn propNameEql(a: []const u8, b: []const u8) bool {
    var j: usize = 0;
    for (a) |c| {
        if (c == '-' or c == '_') continue;
        if (j >= b.len) return false;
        if (b[j] == '-' or b[j] == '_') j += 1;
        if (j >= b.len) return false;
        if (std.ascii.toLower(c) != std.ascii.toLower(b[j])) return false;
        j += 1;
    }
    return j == b.len;
}

test "propNameEql(a, b)" {
    try std.testing.expect(propNameEql("background_color", "background-color"));
    try std.testing.expect(propNameEql("background-color", "backgroundColor"));
    try std.testing.expect(!propNameEql("background-color", "xxx"));
    try std.testing.expect(!propNameEql("xxx", "background-color"));
    try std.testing.expect(!propNameEql("foo", "bar"));
}
