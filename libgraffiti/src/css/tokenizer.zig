// https://drafts.csswg.org/css-syntax/#tokenization

const std = @import("std");

pub const Token = struct {
    tag: Tag,
    start: usize,
    end: usize,

    pub const Tag = enum {
        ident,
        // TODO: function,
        // TODO: at_keyword,
        hash,
        string,
        // TODO: bad_string,
        // TODO: url,
        // TODO: bad_url,

        // # + - . < @ \ <really anything else?>
        delim,

        // TODO: if there is % or ident after num
        number,
        percentage,
        dimension,

        whitespace,
        // TODO: CDO, CDC,
        colon,
        semicolon,
        comma,
        left_square,
        right_square,
        left_paren,
        right_paren,
        left_curly,
        right_curly,
    };
};

pub const Tokenizer = struct {
    input: []const u8,
    pos: usize,

    const Self = @This();

    pub fn init(input: []const u8) Self {
        return .{ .input = input, .pos = 0 };
    }

    pub fn next(self: *Self) Token {
        const start = self.pos;
        const ch = self.peek(0);

        // https://drafts.csswg.org/css-syntax/#consume-token
        const tag: Token.Tag = switch (ch) {
            '\'', '"' => .string,
            '#' => .hash, // TODO: hash_num/hash_id
            '(' => .left_paren,
            ')' => .right_paren,
            '+' => @panic("TODO"),
            ',' => .comma,
            '-' => @panic("TODO"),
            '.' => @panic("TODO"),
            ':' => .colon,
            ';' => .semicolon,
            '<' => .delim,
            '@' => .delim,
            '[' => .left_square,
            '\\' => .delim,
            ']' => .right_square,
            '{' => .left_curly,
            '}' => .right_curly,
            '0'...'9' => .number,
            'a'...'z', 'A'...'Z', '_' => .ident,
            else => .delim,
        };

        const end = switch (tag) {
            .ident => @panic("TODO"),
            else => start + 1,
        };

        self.pos = end;

        return .{ .tag = tag, .start = start, .end = end };
    }

    fn peek(self: *Self, n: usize) u8 {
        return self.input[self.pos + n];
    }
};

fn expectTokens(input: []const u8, tokens: []const Token.Tag) !void {
    var tokenizer = Tokenizer.init(input);

    for (tokens) |tag| {
        try std.testing.expectEqual(tag, tokenizer.next().tag);
    }
}

test {
    try expectTokens("", &.{});
    try expectTokens(";", &.{.semicolon});
    try expectTokens("()", &.{ .left_paren, .right_paren });
    try expectTokens("[]", &.{ .left_square, .right_square });
    try expectTokens("{}", &.{ .left_curly, .right_curly });
}
