// https://drafts.csswg.org/css-syntax/#tokenization

const std = @import("std");

pub const Token = struct {
    tag: Tag,
    start: usize,
    end: usize,

    pub const Tag = enum {
        comment,
        ident,
        function,
        // TODO: at_keyword,
        hash,
        string,
        // TODO: bad_string, url, bad_url,

        // # + - . < @ \ <really anything else?>
        delim,

        // TODO: if there is % or ident after num
        number,
        percentage,
        dimension,

        space,
        // TODO: CDO, CDC,
        colon,
        semi,
        comma,
        lsquare,
        rsquare,
        lparen,
        rparen,
        lcurly,
        rcurly,
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
            '(' => .lparen,
            ')' => .rparen,
            '+' => @panic("TODO"),
            ',' => .comma,
            '-' => @panic("TODO"),
            '.' => @panic("TODO"),
            ':' => .colon,
            ';' => .semi,
            '<' => .delim,
            '@' => .delim,
            '[' => .lsquare,
            '\\' => .delim,
            ']' => .rsquare,
            '{' => .lcurly,
            '}' => .rcurly,
            '0'...'9' => .number,
            'a'...'z', 'A'...'Z', '_' => .ident,
            else => .delim,
        };

        const end = switch (tag) {
            // TODO: find end properly
            .ident => self.input.len - self.pos,
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
    try expectTokens(" ", &.{.space});
    try expectTokens(" \n \t \n ", &.{.space});

    try expectTokens("/* */", &.{.comment});
    try expectTokens(" /**/ /**/ ", &.{ .space, .comment, .space, .comment, .space });

    try expectTokens(";", &.{.semi});
    try expectTokens(";;", &.{ .semi, .semi });
    try expectTokens("; ;", &.{ .semi, .space, .semi });

    try expectTokens("()[]{}", &.{ .lparen, .rparen, .lsquare, .rsquare, .lcurly, .rcurly });

    try expectTokens("block", &.{.ident});
    try expectTokens("10px", &.{.dimension});
    try expectTokens("-10px", &.{.dimension});
    try expectTokens("ident2", &.{.ident});

    try expectTokens("ff0", &.{.ident});
    try expectTokens("00f", &.{.dimension});
    try expectTokens("#00f", &.{.hash});

    try expectTokens("0 10px", &.{ .number, .space, .dimension });
    try expectTokens("0 0 10px 0", &.{ .number, .space, .number, .space, .dimension, .space, .number });

    try expectTokens("a b", &.{ .ident, .space, .ident });
    try expectTokens(".a .b", &.{ .delim, .ident, .space, .delim, .ident });
    try expectTokens(" a .b #c *", &.{ .space, .ident, .space, .delim, .ident, .space, .hash, .space, .delim });

    try expectTokens("!important", &.{ .delim, .ident });
    try expectTokens("! important", &.{ .delim, .space, .ident });

    try expectTokens("-webkit-xxx", &.{.ident});
    try expectTokens("--var", &.{.ident});

    try expectTokens(
        "parent .btn { /**/ padding: 10px }",
        &.{ .ident, .space, .delim, .ident, .space, .lcurly, .space, .comment, .space, .ident, .colon, .space, .dimension, .space, .rcurly },
    );

    try expectTokens("'foo'", &.{.string});
    try expectTokens("\"foo bar\"", &.{.string});
    try expectTokens("'\\''", &.{.string});
    try expectTokens("prop: url('foo bar')", &.{ .ident, .colon, .space, .function, .string, .rparen });

    // assert_eq!(tokenize(b"[foo=\"bar\"]"), vec!["[", "foo", "=", "\"bar\"", "]"]);

    // assert_eq!(
    //     tokenize(b"@media { a b { left: 10% } }"),
    //     vec!["@", "media", "{", "a", " ", "b", "{", "left", ":", "10", "%", "}", "}"]
    // );

    // //assert_eq!(tokenize(b"/**/ a /**/ b {}"), vec!["a", " ", "b", "{", "}"]);
}
