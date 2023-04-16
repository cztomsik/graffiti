// different from https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/#tokenization
// the purpose here is to simplify parsing rather than to implement spec-compliant tokenizer
// (spaces are not generated, but the `.space_before` field can be used to tell if there was a space)

const std = @import("std");

pub const Token = union(enum) {
    ident: []const u8,
    function: []const u8,
    number: f32,
    dimension: struct { value: f32, unit: []const u8 },
    hash: []const u8,
    class_name: []const u8,
    string: []const u8,
    important,
    colon,
    semi,
    star,
    gt,
    plus,
    tilde,
    comma,
    lsquare,
    rsquare,
    lparen,
    rparen,
    lcurly,
    rcurly,
    other: u8,
};

pub const Tokenizer = struct {
    input: []const u8,
    pos: usize = 0,
    space_before: bool = false,
    semi_before: bool = false,

    const Self = @This();

    const Error = error{ Eof, InvalidCharacter };

    pub fn rest(self: *Self) []const u8 {
        return self.input[self.pos..];
    }

    pub fn peek(self: *Self) Error!u8 {
        return if (self.pos < self.input.len) self.input[self.pos] else error.Eof;
    }

    pub fn next(self: *Self) Error!Token {
        self.space_before = false;

        if (self.consumeNumber()) |num| {
            if (self.consumeSeq("%")) {
                return Token{ .dimension = .{ .value = num, .unit = "percent" } };
            }

            if (self.consumeIdent()) |unit| {
                return Token{ .dimension = .{ .value = num, .unit = unit } };
            }

            return Token{ .number = num };
        }

        if (self.consumeIdent()) |id| {
            if (self.consumeSeq("(")) {
                return Token{ .function = id };
            }

            return Token{ .ident = id };
        }

        // TODO(perf): happy-path should be first
        const ch = try self.nextCharSkipComments();

        if (std.ascii.isWhitespace(ch)) {
            defer self.space_before = true;
            return self.next();
        }

        if (ch == ';') {
            defer self.semi_before = true;
            return if (self.semi_before) self.next() else Token.semi;
        } else self.semi_before = false;

        return switch (ch) {
            '\'', '"' => Token{ .string = try self.consumeString(ch) },
            '#' => Token{ .hash = self.consumeHash() orelse "" },
            '.' => Token{ .class_name = self.consumeIdent() orelse "" },
            '*' => Token.star,
            '>' => Token.gt,
            '+' => Token.plus,
            '~' => Token.tilde,
            ':' => Token.colon,
            ',' => Token.comma,
            '[' => Token.lsquare,
            ']' => Token.rsquare,
            '(' => Token.lparen,
            ')' => Token.rparen,
            '{' => Token.lcurly,
            '}' => Token.rcurly,
            else => Token{ .other = ch },
        };
    }

    fn consumeNumber(self: *Self) ?f32 {
        const prev = self.pos;
        const s = self.consume(isNumStart, isNumeric) orelse return null;
        return std.fmt.parseFloat(f32, s) catch {
            self.pos = prev;
            return null;
        };
    }

    fn consumeIdent(self: *Self) ?[]const u8 {
        return self.consume(isIdentStart, isIdent);
    }

    fn consumeHash(self: *Self) ?[]const u8 {
        return self.consume(isIdent, isIdent);
    }

    fn consumeSeq(self: *Self, needle: []const u8) bool {
        if (std.mem.startsWith(u8, self.rest(), needle)) {
            self.pos += needle.len;
            return true;
        }

        return false;
    }

    fn consume(self: *Self, f1: anytype, f2: anytype) ?[]const u8 {
        const start = self.pos;

        if (f1(self.peek() catch return null)) {
            self.pos += 1;

            while (self.peek() catch null) |ch2| : (self.pos += 1) {
                if (!f2(ch2)) break;
            }

            return self.input[start..self.pos];
        }

        return null;
    }

    fn consumeString(self: *Self, quote: u8) ![]const u8 {
        const start = self.pos;
        var prev: u8 = '\\';
        self.pos += 1;

        while (self.peek() catch null) |ch| {
            if (ch == quote and prev != '\\') break;

            prev = ch;
            self.pos += 1;
        }

        self.pos += 1;

        return self.input[(start + 1)..(self.pos - 1)];
    }

    fn nextCharSkipComments(self: *Self) Error!u8 {
        while (self.consumeSeq("/*")) {
            while (!self.consumeSeq("*/")) self.pos += 1;
        }

        const ch = try self.peek();
        self.pos += 1;
        return ch;
    }
};

fn isIdentStart(ch: u8) bool {
    return std.ascii.isAlphabetic(ch) or ch == '_' or ch == '-';
}

fn isIdent(ch: u8) bool {
    return isIdentStart(ch) or std.ascii.isDigit(ch);
}

fn isNumStart(ch: u8) bool {
    return isNumeric(ch) or ch == '-' or ch == '+';
}

fn isNumeric(ch: u8) bool {
    return std.ascii.isDigit(ch) or ch == '.';
}

fn expectTokens(input: []const u8, tokens: []const std.meta.Tag(Token)) !void {
    var tokenizer = Tokenizer{ .input = input };

    for (tokens) |tag| {
        const tok = try tokenizer.next();
        errdefer std.debug.print("token: {any}\n", .{tok});

        try std.testing.expectEqual(tag, tok);
    }

    try std.testing.expectError(error.Eof, tokenizer.next());
    try std.testing.expectEqual(input.len, tokenizer.pos);
}

test {
    try expectTokens("", &.{});
    try expectTokens(" ", &.{});
    try expectTokens(" \n \t \n ", &.{});
    try expectTokens("/* */", &.{});
    try expectTokens(" /**/ /**/ ", &.{});

    try expectTokens(";", &.{.semi});
    try expectTokens(";;", &.{.semi});
    try expectTokens(";; ;;", &.{.semi});
    try expectTokens(" ; ; ; ;", &.{.semi});

    try expectTokens("+", &.{.plus});
    // try expectTokens("-", &.{.other});

    try expectTokens("()[]{}", &.{ .lparen, .rparen, .lsquare, .rsquare, .lcurly, .rcurly });

    try expectTokens("block", &.{.ident});
    try expectTokens("10px", &.{.dimension});
    try expectTokens("-10px", &.{.dimension});
    try expectTokens("ident2", &.{.ident});

    try expectTokens("ff0", &.{.ident});
    try expectTokens("00f", &.{.dimension});
    try expectTokens("#00f", &.{.hash});
    try expectTokens("rgb(0, 0, 1)", &.{ .function, .number, .comma, .number, .comma, .number, .rparen });

    try expectTokens("0 10px", &.{ .number, .dimension });
    try expectTokens("0 0 10px 0", &.{ .number, .number, .dimension, .number });
    try expectTokens("-10.1 +1.1 .123px -10.1%", &.{ .number, .number, .dimension, .dimension });

    try expectTokens("a b", &.{ .ident, .ident });
    try expectTokens(".a .b", &.{ .class_name, .class_name });
    try expectTokens(" a .b #c *", &.{ .ident, .class_name, .hash, .star });

    // try expectTokens("!important", &.{.important});
    // try expectTokens("! important", &.{ .other, .ident });
    try expectTokens("-webkit-xxx", &.{.ident});
    try expectTokens("--var", &.{.ident});

    try expectTokens(
        "parent .btn { /**/ padding: 10px }",
        &.{ .ident, .class_name, .lcurly, .ident, .colon, .dimension, .rcurly },
    );

    try expectTokens("'foo'", &.{.string});
    try expectTokens("\"foo bar\"", &.{.string});
    try expectTokens("'\\''", &.{.string});
    try expectTokens("prop: url('foo bar')", &.{ .ident, .colon, .function, .string, .rparen });
    try expectTokens("[foo=\"bar\"]", &.{ .lsquare, .ident, .other, .string, .rsquare });

    try expectTokens(
        "@media { a b { left: 10% } }",
        &.{ .other, .ident, .lcurly, .ident, .ident, .lcurly, .ident, .colon, .dimension, .rcurly, .rcurly },
    );

    try expectTokens("/**/ a /**/ b {}", &.{ .ident, .ident, .lcurly, .rcurly });
}
