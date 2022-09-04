// different from https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/#tokenization
// the purpose here is to simplify parsing rather than to implement spec-compliant tokenizer
// spaces are only generated in selectors (after ident/*/] and before ident/#/./*)

const std = @import("std");

pub const Token = union(enum) {
    ident: []const u8,
    function: []const u8,
    number: f32,
    percentage: f32,
    dimension: struct { value: f32, unit: []const u8 },
    hash: []const u8,
    string: []const u8,
    important,
    space,
    colon,
    semi,
    star,
    dot,
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

const TokenTag = @typeInfo(Token).Union.tag_type.?;

pub const Tokenizer = struct {
    input: []const u8,
    pos: usize = 0,
    space: enum { skip, wait, emit } = .skip,

    const Self = @This();

    const Error = error{ Eof, InvalidCharacter };

    pub fn next(self: *Self) Error!Token {
        const ch = try self.peek(0);

        if (std.ascii.isSpace(ch)) {
            self.pos += 1;

            if (self.space == .wait) {
                self.space = .emit;
            }

            return try self.next();
        }

        if (ch == '/' and (self.peek(1) catch 0) == '*') {
            self.pos += 2;
            while ((try self.peek(0)) != '*' and (try self.peek(1)) != '/') self.pos += 1;
            self.pos += 2;
            return try self.next();
        }

        if (self.space == .emit and (ch == '#' or ch == '.' or ch == '*' or isIdentStart(ch))) {
            self.space = .skip;
            return Token.space;
        }

        if (isIdentStart(ch)) {
            self.space = .wait;

            const ident = self.consume(isIdent);

            if ((self.peek(0) catch 0) == '(') {
                self.pos += 1;
                return Token{ .function = ident };
            }

            return Token{ .ident = ident };
        }

        if (isNumeric(ch)) {
            const start = self.pos;

            if (std.fmt.parseFloat(f32, self.consume(isNumeric)) catch null) |num| {
                if (self.peek(0) catch 0 == '%') {
                    self.pos += 1;
                    return Token{ .percentage = num };
                }

                if (isIdentStart(self.peek(0) catch 0)) {
                    return Token{ .dimension = .{ .value = num, .unit = self.consume(isIdent) } };
                }

                return Token{ .number = num };
            } else {
                // reset (put everything back)
                self.pos = start;
            }
        }

        self.pos += 1;
        self.space = .skip;

        return switch (ch) {
            '\'', '"' => .{ .string = try self.consumeString(ch) },
            '#' => Token{ .hash = self.consume(isIdent) },
            '*' => blk: {
                self.space = .wait;
                break :blk Token.star;
            },
            '.' => Token.dot,
            '+' => Token.plus,
            '~' => Token.tilde,
            ':' => Token.colon,
            ';' => Token.semi,
            ',' => Token.comma,
            '[' => Token.lsquare,
            ']' => blk: {
                self.space = .wait;
                break :blk Token.rsquare;
            },
            '(' => Token.lparen,
            ')' => Token.rparen,
            '{' => Token.lcurly,
            '}' => Token.rcurly,
            else => Token{ .other = ch },
        };
    }

    fn consume(self: *Self, fun: anytype) []const u8 {
        const start = self.pos;
        while (fun(self.peek(0) catch 0)) self.pos += 1;

        return self.input[start..self.pos];
    }

    fn consumeString(self: *Self, quote: u8) ![]const u8 {
        const start = self.pos;
        var prev: u8 = '\\';
        self.pos += 1;

        while (self.peek(0) catch null) |ch| {
            if (ch == quote and prev != '\\') break;

            prev = ch;
            self.pos += 1;
        }

        self.pos += 1;

        return self.input[(start + 1)..(self.pos - 1)];
    }

    fn peek(self: *Self, n: usize) !u8 {
        const i = self.pos + n;

        return if (i < self.input.len) self.input[i] else error.Eof;
    }
};

fn isIdentStart(ch: u8) bool {
    return ch == '_' or ch == '-' or std.ascii.isAlpha(ch);
}

fn isIdent(ch: u8) bool {
    return isIdentStart(ch) or std.ascii.isDigit(ch);
}

fn isNumeric(ch: u8) bool {
    return ch == '.' or std.ascii.isDigit(ch);
}

fn expectTokens(input: []const u8, tokens: []const TokenTag) !void {
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
    // try expectTokens(";;", &.{.semi});
    // try expectTokens(";; ;;", &.{.semi});
    // try expectTokens(" ; ; ; ;", &.{.semi});

    try expectTokens("()[]{}", &.{ .lparen, .rparen, .lsquare, .rsquare, .lcurly, .rcurly });

    try expectTokens("block", &.{.ident});
    try expectTokens("10px", &.{.dimension});
    // try expectTokens("-10px", &.{.dimension});
    try expectTokens("ident2", &.{.ident});

    try expectTokens("ff0", &.{.ident});
    try expectTokens("00f", &.{.dimension});
    try expectTokens("#00f", &.{.hash});

    try expectTokens("0 10px", &.{ .number, .dimension });
    try expectTokens("0 0 10px 0", &.{ .number, .number, .dimension, .number });

    try expectTokens("a b", &.{ .ident, .space, .ident });
    try expectTokens(".a .b", &.{ .dot, .ident, .space, .dot, .ident });
    try expectTokens(" a .b #c *", &.{ .ident, .space, .dot, .ident, .space, .hash, .star });

    // try expectTokens("!important", &.{.important});
    // try expectTokens("! important", &.{ .other, .ident });
    try expectTokens("-webkit-xxx", &.{.ident});
    try expectTokens("--var", &.{.ident});

    try expectTokens(
        "parent .btn { /**/ padding: 10px }",
        &.{ .ident, .space, .dot, .ident, .lcurly, .ident, .colon, .dimension, .rcurly },
    );

    try expectTokens("'foo'", &.{.string});
    try expectTokens("\"foo bar\"", &.{.string});
    try expectTokens("'\\''", &.{.string});
    try expectTokens("prop: url('foo bar')", &.{ .ident, .colon, .function, .string, .rparen });
    try expectTokens("[foo=\"bar\"]", &.{ .lsquare, .ident, .other, .string, .rsquare });

    try expectTokens(
        "@media { a b { left: 10% } }",
        &.{ .other, .ident, .lcurly, .ident, .space, .ident, .lcurly, .ident, .colon, .percentage, .rcurly, .rcurly },
    );

    try expectTokens("/**/ a /**/ b {}", &.{ .ident, .space, .ident, .lcurly, .rcurly });
}
