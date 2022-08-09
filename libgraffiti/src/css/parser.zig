const std = @import("std");
const Tokenizer = @import("tokenizer.zig").Tokenizer;
const Dimension = @import("values.zig").Dimension;

const Display = enum { block, table_row };

const Style = struct {
    display: Display,
};

pub const Parser = struct {
    allocator: std.mem.Allocator,
    tokenizer: Tokenizer,

    const Self = @This();

    pub fn init(allocator: std.mem.Allocator, input: []const u8) Self {
        return .{
            .allocator = allocator,
            .tokenizer = Tokenizer.init(input),
        };
    }

    // pub fn parseStyleSheet(self: *Self) !StyleSheet {}
    // pub fn parseStyleRule(self: *Self) !StyleRule {}
    // pub fn parseSelector(self: *Self) !Selector {}
    // pub fn parseStyle(self: *Self) !Style {}
    // pub fn parseProp(self: *Self) !StyleProp {}

    // so it can be used from style.setProperty()
    // ? but what about shorthands?
    // pub fn parsePropValue(self: *Self, prop_name: []const u8) !Style {}

    fn parseValue(self: *Self, comptime T: type) T {
        if (@typeInfo(T) == .Enum) {
            return self.parseEnum(T);
        }

        return switch (T) {
            f32 => @panic("TODO"),
            // TODO: string
            // Color => self.parseColor()
            // Dimension => self.parseDimension()
            // ...
            else => @compileError("unknown value type"),
        };
    }

    fn parseEnum(self: *Self, comptime T: type) !T {
        const tok = try self.tokenizer.next();

        if (tok == .ident) {
            inline for (@typeInfo(T).Enum.fields) |f| {
                if (std.mem.eql(u8, cssName(f.name), tok.ident)) {
                    return @intToEnum(T, f.value);
                }
            }
        }

        return error.invalid;
    }

    fn parseDimension(self: *Self) !Dimension {
        const tok = try self.tokenizer.next();

        // https://github.com/ziglang/zig/issues/6749
        const D = Dimension;

        switch (tok) {
            .number => |n| if (n == 0) return D{ .px = 0 },
            .percentage => |p| return D{ .percent = p },
            .dimension => |d| {
                if (std.mem.eql(u8, "px", d.unit)) return D{ .px = d.value };
                if (std.mem.eql(u8, "em", d.unit)) return D{ .em = d.value };
                if (std.mem.eql(u8, "rem", d.unit)) return D{ .rem = d.value };
                if (std.mem.eql(u8, "vw", d.unit)) return D{ .vw = d.value };
                if (std.mem.eql(u8, "vh", d.unit)) return D{ .vh = d.value };
            },
            .ident => |k| {
                if (std.mem.eql(u8, "auto", k)) return D.auto;
                if (std.mem.eql(u8, "vmin", k)) return D.vmin;
                if (std.mem.eql(u8, "vmax", k)) return D.vmax;
            },
            else => {},
        }

        std.debug.print("{}\n", .{tok});
        return error.invalid;
    }
};

fn cssName(comptime name: []const u8) []const u8 {
    comptime {
        var buf: [name.len:0]u8 = undefined;
        _ = std.mem.replace(u8, name, "_", "-", &buf);
        buf[buf.len] = 0;
        return &buf;
    }
}

fn testParser(input: []const u8) Parser {
    return Parser.init(std.testing.allocator, input);
}

test "enums" {
    try std.testing.expectEqual(Display.block, try testParser("block").parseEnum(Display));
    try std.testing.expectEqual(Display.table_row, try testParser("table-row").parseEnum(Display));
    try std.testing.expectError(error.invalid, testParser("err").parseEnum(Display));
}

test "dimension" {
    try std.testing.expectEqual(Dimension{ .px = 0 }, try testParser("0").parseDimension());
    try std.testing.expectEqual(Dimension{ .percent = 100 }, try testParser("100%").parseDimension());
    try std.testing.expectEqual(Dimension{ .px = 10 }, try testParser("10px").parseDimension());
    try std.testing.expectEqual(Dimension{ .em = 1.2 }, try testParser("1.2em").parseDimension());
    try std.testing.expectEqual(Dimension{ .rem = 2.1 }, try testParser("2.1rem").parseDimension());
    try std.testing.expectEqual(Dimension{ .vw = 100 }, try testParser("100vw").parseDimension());
    try std.testing.expectEqual(Dimension{ .vh = 100 }, try testParser("100vh").parseDimension());
    try std.testing.expectEqual(Dimension.auto, try testParser("auto").parseDimension());
    try std.testing.expectEqual(Dimension.vmin, try testParser("vmin").parseDimension());
    try std.testing.expectEqual(Dimension.vmax, try testParser("vmax").parseDimension());
}

test "color" {
    // TODO
}
