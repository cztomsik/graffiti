const std = @import("std");
const Parser = @import("parser.zig").Parser;
const cssName = @import("parser.zig").cssName;
const expectParse = @import("parser.zig").expectParse;
const expectFmt = std.testing.expectFmt;

// TODO values should not allocate (fixbuf allocator with empty slice?)
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

// declaration block represented as a wrapper over user-provided Style struct
// - setting property will immediately change the style and set a flag
// - removing will unset flag (and restore the default value)
//
// Style is potentially big but all of this is alloc-free
// and that should outweight any loss, if there is any at all
//
// this also implies that value types can't allocate
// they have to be either fixed-size (with max., for example)
// or they have to be interned (likely-forever)
// IMHO this is good trade-off for the overall simplicity
pub fn StyleDeclaration(comptime T: type) type {
    const props = T.cssMapping.properties;
    const defaults = T{};

    return struct {
        flags: std.StaticBitSet(props.len) = .{ .mask = 0 },
        style: T = .{},

        const Self = @This();

        pub fn format(self: Self, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            var sep = false;
            inline for (props) |prop, i| {
                if (self.flags.isSet(i)) {
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
        }

        pub fn parse(parser: *Parser) !Self {
            var res = Self{};

            while (parser.expect(.ident) catch null) |prop_name| {
                try parser.expect(.colon);

                const val_start = parser.tokenizer.pos;
                while (parser.tokenizer.next() catch null) |t2| if (t2 == .semi or t2 == .rcurly) break;

                res.setProperty(prop_name, parser.tokenizer.input[val_start..parser.tokenizer.pos]);
            }

            return res;
        }

        pub fn length(self: *Self) usize {
            return self.flags.count();
        }

        pub fn item(self: *Self, index: usize) []const u8 {
            var i: usize = 0;

            inline for (props) |prop, j| {
                if (self.flags.isSet(j)) {
                    if (i == index) return prop.@"0" else i += 1;
                }
            } else return "";
        }

        pub fn getProperty() void {
            // TODO
        }

        pub fn setProperty(self: *Self, prop_name: []const u8, value: []const u8) void {
            inline for (props) |prop, i| {
                const V = @typeInfo(@TypeOf(prop.@"1")).Fn.return_type.?;

                if (std.mem.eql(u8, prop.@"0", prop_name)) {
                    var parser = Parser.init(gpa.allocator(), value);
                    var val = parser.parse(V) catch {
                        return std.log.debug("ignored invalid {s}: {s}\n", .{ prop_name, value });
                    };
                    prop.@"2"(&self.style, val);
                    self.flags.set(i);
                }
            }
        }

        pub fn removeProperty(self: *Self, prop_name: []const u8) void {
            inline for (props) |prop, i| {
                if (std.mem.eql(u8, prop.@"0", prop_name)) {
                    const default = prop.@"1"(&defaults);
                    prop.@"2"(&self.style, default);
                    self.flags.unset(i);
                }
            }
        }
    };
}

pub fn setterName(comptime name: []const u8) []const u8 {
    return "set" ++ [_]u8{std.ascii.toUpper(name[0])} ++ name[1..];
}

// }

// const Decl = StyleDeclaration(struct {
//     display: enum { none, block } = .block,
//     opacity: f32 = 1,
//     flex_grow: f32 = 0,
// });

// test "StyleDeclaration.length()" {
//     var s = Decl{};
//     try std.testing.expectEqual(s.length(), 0);

//     s.flags[0] = 1;
//     try std.testing.expectEqual(s.length(), 1);
// }

// test "StyleDeclaration.item()" {
//     var s = Decl{};
//     try std.testing.expectEqualStrings(s.item(0), "");

//     s.flags[0] = 1;
//     s.flags[2] = 1;
//     try std.testing.expectEqualStrings(s.item(0), "display");
//     try std.testing.expectEqualStrings(s.item(1), "flex-grow");
// }

// test "StyleDeclaration.format()" {
//     var s = Decl{};
//     try expectFmt("", "{}", .{s});

//     s.setProperty("display", "none");
//     try expectFmt("display: none", "{}", .{s});

//     s.setProperty("flex-grow", "1");
//     try expectFmt("display: none; flex-grow: 1", "{}", .{s});
// }

// test "StyleDeclaration.parse()" {
//     try expectParse(Decl, "", Decl{});
//     try expectParse(Decl, "display: block", Decl{ .flags = .{ 1, 0, 0 } });
//     try expectParse(Decl, "unknown: 0; opacity: 0", Decl{ .flags = .{ 0, 1, 0 }, .data = .{ .opacity = 0 } });
//     try expectParse(Decl, "opacity: 0; opacity: invalid", Decl{ .flags = .{ 0, 1, 0 }, .data = .{ .opacity = 0 } });
// }
