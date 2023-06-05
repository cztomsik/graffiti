const std = @import("std");
const Parser = @import("parser.zig").Parser;
const Angle = @import("angle.zig").Angle;
const NumberOrPercentage = @import("percentage.zig").NumberOrPercentage;
const expectParse = @import("parser.zig").expectParse;

/// A color in sRGB color space.
pub const Color = struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,

    pub fn rgb(r: u8, g: u8, b: u8) Color {
        return rgba(r, g, b, .{ .value = 1.0 });
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: NumberOrPercentage) Color {
        return .{ .r = r, .g = g, .b = b, .a = f32_to_u8_clamped(a.value * 255.0) };
    }

    pub fn hsl(h: Angle, s: NumberOrPercentage, l: NumberOrPercentage) Color {
        return hsla(h, s, l, .{ .value = 1.0 });
    }

    pub fn hsla(hue: Angle, saturation: NumberOrPercentage, lightness: NumberOrPercentage, alpha: NumberOrPercentage) Color {
        var h = @mod(hue.value(), 1.0);
        if (h < 0.0) h += 1.0;

        var s = std.math.clamp(saturation.value, 0, 1);
        var l = std.math.clamp(lightness.value, 0, 1);

        var a = f32_to_u8_clamped(alpha.value * 255.0);

        if (s == 0) {
            var v = f32_to_u8_clamped(l * 255.0);
            return .{
                .r = v,
                .g = v,
                .b = v,
                .a = a,
            };
        }

        var m2 = if (l <= 0.5) l * (s + 1) else l + s - l * s;
        var m1 = l * 2 - m2;

        return .{
            .r = f32_to_u8_clamped(@round(hue_to_rgb(h + 1.0 / 3.0, m1, m2) * 255.0)),
            .g = f32_to_u8_clamped(@round(hue_to_rgb(h, m1, m2) * 255.0)),
            .b = f32_to_u8_clamped(@round(hue_to_rgb(h - 1.0 / 3.0, m1, m2) * 255.0)),
            .a = a,
        };
    }

    pub fn named(name: []const u8) ?Color {
        inline for (NAMED_COLORS) |tuple| {
            if (std.mem.eql(u8, name, tuple.@"0")) {
                return tuple.@"1";
            }
        }

        return null;
    }

    pub fn format(self: Color, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        try writer.print("rgba({}, {}, {}, {})", .{ self.r, self.g, self.b, self.a });
    }

    pub fn parseWith(parser: *Parser) !Color {
        const tok = try parser.tokenizer.next();

        switch (tok) {
            .ident => if (named(tok.ident)) |c| return c,
            .function => inline for (.{ "rgb", "rgba", "hsl", "hsla" }) |name| {
                if (std.mem.eql(u8, tok.function, name)) {
                    return parser.parseFnCall(@field(Color, name));
                }
            },
            .hash => |s| switch (s.len) {
                8 => return rgba(hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), .{ .value = @intToFloat(f32, hex(s[6..8])) / 255.0 }),
                6 => return rgba(hex(s[0..2]), hex(s[2..4]), hex(s[4..6]), .{ .value = 1.0 }),
                4 => return rgba((hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, .{ .value = @intToFloat(f32, (hex(s[3..4])) * 17) / 255.0 }),
                3 => return rgba((hex(s[0..1])) * 17, (hex(s[1..2])) * 17, (hex(s[2..3])) * 17, .{ .value = 1.0 }),
                else => {},
            },
            else => {},
        }

        return error.InvalidColor;
    }
};

fn hex(s: []const u8) u8 {
    return std.fmt.parseInt(u8, s, 16) catch 0;
}

fn hue_to_rgb(hue: f32, m1: f32, m2: f32) f32 {
    var h = if (hue < 0.0) hue + 1.0 else if (hue > 1.0) hue - 1.0 else hue;

    if (h * 6.0 < 1.0) {
        return m1 + (m2 - m1) * h * 6.0;
    }
    if (h * 2.0 < 1.0) {
        return m2;
    }
    if (h * 3.0 < 2.0) {
        return m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0;
    }
    return m1;
}

fn f32_to_u8_clamped(f: f32) u8 {
    return @floatToInt(u8, std.math.clamp(f, 0, 255));
}

const NAMED_COLORS = .{
    .{ "transparent", Color.rgba(0, 0, 0, .{ .value = 0 }) },
    // https://drafts.csswg.org/css-color/#named-colors
    .{ "aliceblue", Color.rgb(240, 248, 255) },
    .{ "antiquewhite", Color.rgb(250, 235, 215) },
    .{ "aqua", Color.rgb(0, 255, 255) },
    .{ "aquamarine", Color.rgb(127, 255, 212) },
    .{ "azure", Color.rgb(240, 255, 255) },
    .{ "beige", Color.rgb(245, 245, 220) },
    .{ "bisque", Color.rgb(255, 228, 196) },
    .{ "black", Color.rgb(0, 0, 0) },
    .{ "blanchedalmond", Color.rgb(255, 235, 205) },
    .{ "blue", Color.rgb(0, 0, 255) },
    .{ "blueviolet", Color.rgb(138, 43, 226) },
    .{ "brown", Color.rgb(165, 42, 42) },
    .{ "burlywood", Color.rgb(222, 184, 135) },
    .{ "cadetblue", Color.rgb(95, 158, 160) },
    .{ "chartreuse", Color.rgb(127, 255, 0) },
    .{ "chocolate", Color.rgb(210, 105, 30) },
    .{ "coral", Color.rgb(255, 127, 80) },
    .{ "cornflowerblue", Color.rgb(100, 149, 237) },
    .{ "cornsilk", Color.rgb(255, 248, 220) },
    .{ "crimson", Color.rgb(220, 20, 60) },
    .{ "cyan", Color.rgb(0, 255, 255) },
    .{ "darkblue", Color.rgb(0, 0, 139) },
    .{ "darkcyan", Color.rgb(0, 139, 139) },
    .{ "darkgoldenrod", Color.rgb(184, 134, 11) },
    .{ "darkgray", Color.rgb(169, 169, 169) },
    .{ "darkgreen", Color.rgb(0, 100, 0) },
    .{ "darkgrey", Color.rgb(169, 169, 169) },
    .{ "darkkhaki", Color.rgb(189, 183, 107) },
    .{ "darkmagenta", Color.rgb(139, 0, 139) },
    .{ "darkolivegreen", Color.rgb(85, 107, 47) },
    .{ "darkorange", Color.rgb(255, 140, 0) },
    .{ "darkorchid", Color.rgb(153, 50, 204) },
    .{ "darkred", Color.rgb(139, 0, 0) },
    .{ "darksalmon", Color.rgb(233, 150, 122) },
    .{ "darkseagreen", Color.rgb(143, 188, 143) },
    .{ "darkslateblue", Color.rgb(72, 61, 139) },
    .{ "darkslategray", Color.rgb(47, 79, 79) },
    .{ "darkslategrey", Color.rgb(47, 79, 79) },
    .{ "darkturquoise", Color.rgb(0, 206, 209) },
    .{ "darkviolet", Color.rgb(148, 0, 211) },
    .{ "deeppink", Color.rgb(255, 20, 147) },
    .{ "deepskyblue", Color.rgb(0, 191, 255) },
    .{ "dimgray", Color.rgb(105, 105, 105) },
    .{ "dimgrey", Color.rgb(105, 105, 105) },
    .{ "dodgerblue", Color.rgb(30, 144, 255) },
    .{ "firebrick", Color.rgb(178, 34, 34) },
    .{ "floralwhite", Color.rgb(255, 250, 240) },
    .{ "forestgreen", Color.rgb(34, 139, 34) },
    .{ "fuchsia", Color.rgb(255, 0, 255) },
    .{ "gainsboro", Color.rgb(220, 220, 220) },
    .{ "ghostwhite", Color.rgb(248, 248, 255) },
    .{ "gold", Color.rgb(255, 215, 0) },
    .{ "goldenrod", Color.rgb(218, 165, 32) },
    .{ "gray", Color.rgb(128, 128, 128) },
    .{ "green", Color.rgb(0, 128, 0) },
    .{ "greenyellow", Color.rgb(173, 255, 47) },
    .{ "grey", Color.rgb(128, 128, 128) },
    .{ "honeydew", Color.rgb(240, 255, 240) },
    .{ "hotpink", Color.rgb(255, 105, 180) },
    .{ "indianred", Color.rgb(205, 92, 92) },
    .{ "indigo", Color.rgb(75, 0, 130) },
    .{ "ivory", Color.rgb(255, 255, 240) },
    .{ "khaki", Color.rgb(240, 230, 140) },
    .{ "lavender", Color.rgb(230, 230, 250) },
    .{ "lavenderblush", Color.rgb(255, 240, 245) },
    .{ "lawngreen", Color.rgb(124, 252, 0) },
    .{ "lemonchiffon", Color.rgb(255, 250, 205) },
    .{ "lightblue", Color.rgb(173, 216, 230) },
    .{ "lightcoral", Color.rgb(240, 128, 128) },
    .{ "lightcyan", Color.rgb(224, 255, 255) },
    .{ "lightgoldenrodyellow", Color.rgb(250, 250, 210) },
    .{ "lightgray", Color.rgb(211, 211, 211) },
    .{ "lightgreen", Color.rgb(144, 238, 144) },
    .{ "lightgrey", Color.rgb(211, 211, 211) },
    .{ "lightpink", Color.rgb(255, 182, 193) },
    .{ "lightsalmon", Color.rgb(255, 160, 122) },
    .{ "lightseagreen", Color.rgb(32, 178, 170) },
    .{ "lightskyblue", Color.rgb(135, 206, 250) },
    .{ "lightslategray", Color.rgb(119, 136, 153) },
    .{ "lightslategrey", Color.rgb(119, 136, 153) },
    .{ "lightsteelblue", Color.rgb(176, 196, 222) },
    .{ "lightyellow", Color.rgb(255, 255, 224) },
    .{ "lime", Color.rgb(0, 255, 0) },
    .{ "limegreen", Color.rgb(50, 205, 50) },
    .{ "linen", Color.rgb(250, 240, 230) },
    .{ "magenta", Color.rgb(255, 0, 255) },
    .{ "maroon", Color.rgb(128, 0, 0) },
    .{ "mediumaquamarine", Color.rgb(102, 205, 170) },
    .{ "mediumblue", Color.rgb(0, 0, 205) },
    .{ "mediumorchid", Color.rgb(186, 85, 211) },
    .{ "mediumpurple", Color.rgb(147, 112, 219) },
    .{ "mediumseagreen", Color.rgb(60, 179, 113) },
    .{ "mediumslateblue", Color.rgb(123, 104, 238) },
    .{ "mediumspringgreen", Color.rgb(0, 250, 154) },
    .{ "mediumturquoise", Color.rgb(72, 209, 204) },
    .{ "mediumvioletred", Color.rgb(199, 21, 133) },
    .{ "midnightblue", Color.rgb(25, 25, 112) },
    .{ "mintcream", Color.rgb(245, 255, 250) },
    .{ "mistyrose", Color.rgb(255, 228, 225) },
    .{ "moccasin", Color.rgb(255, 228, 181) },
    .{ "navajowhite", Color.rgb(255, 222, 173) },
    .{ "navy", Color.rgb(0, 0, 128) },
    .{ "oldlace", Color.rgb(253, 245, 230) },
    .{ "olive", Color.rgb(128, 128, 0) },
    .{ "olivedrab", Color.rgb(107, 142, 35) },
    .{ "orange", Color.rgb(255, 165, 0) },
    .{ "orangered", Color.rgb(255, 69, 0) },
    .{ "orchid", Color.rgb(218, 112, 214) },
    .{ "palegoldenrod", Color.rgb(238, 232, 170) },
    .{ "palegreen", Color.rgb(152, 251, 152) },
    .{ "paleturquoise", Color.rgb(175, 238, 238) },
    .{ "palevioletred", Color.rgb(219, 112, 147) },
    .{ "papayawhip", Color.rgb(255, 239, 213) },
    .{ "peachpuff", Color.rgb(255, 218, 185) },
    .{ "peru", Color.rgb(205, 133, 63) },
    .{ "pink", Color.rgb(255, 192, 203) },
    .{ "plum", Color.rgb(221, 160, 221) },
    .{ "powderblue", Color.rgb(176, 224, 230) },
    .{ "purple", Color.rgb(128, 0, 128) },
    .{ "rebeccapurple", Color.rgb(102, 51, 153) },
    .{ "red", Color.rgb(255, 0, 0) },
    .{ "rosybrown", Color.rgb(188, 143, 143) },
    .{ "royalblue", Color.rgb(65, 105, 225) },
    .{ "saddlebrown", Color.rgb(139, 69, 19) },
    .{ "salmon", Color.rgb(250, 128, 114) },
    .{ "sandybrown", Color.rgb(244, 164, 96) },
    .{ "seagreen", Color.rgb(46, 139, 87) },
    .{ "seashell", Color.rgb(255, 245, 238) },
    .{ "sienna", Color.rgb(160, 82, 45) },
    .{ "silver", Color.rgb(192, 192, 192) },
    .{ "skyblue", Color.rgb(135, 206, 235) },
    .{ "slateblue", Color.rgb(106, 90, 205) },
    .{ "slategray", Color.rgb(112, 128, 144) },
    .{ "slategrey", Color.rgb(112, 128, 144) },
    .{ "snow", Color.rgb(255, 250, 250) },
    .{ "springgreen", Color.rgb(0, 255, 127) },
    .{ "steelblue", Color.rgb(70, 130, 180) },
    .{ "tan", Color.rgb(210, 180, 140) },
    .{ "teal", Color.rgb(0, 128, 128) },
    .{ "thistle", Color.rgb(216, 191, 216) },
    .{ "tomato", Color.rgb(255, 99, 71) },
    .{ "turquoise", Color.rgb(64, 224, 208) },
    .{ "violet", Color.rgb(238, 130, 238) },
    .{ "wheat", Color.rgb(245, 222, 179) },
    .{ "white", Color.rgb(255, 255, 255) },
    .{ "whitesmoke", Color.rgb(245, 245, 245) },
    .{ "yellow", Color.rgb(255, 255, 0) },
    .{ "yellowgreen", Color.rgb(154, 205, 50) },
};

test "Color.parse()" {
    try expectParse(Color, "#000000", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "#ff0000", .{ .r = 0xFF, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "#00ff00", .{ .r = 0x00, .g = 0xFF, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "#0000ff", .{ .r = 0x00, .g = 0x00, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#000", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "#f00", .{ .r = 0xFF, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "#fff", .{ .r = 0xFF, .g = 0xFF, .b = 0xFF, .a = 0xFF });

    try expectParse(Color, "#0000", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0x00 });
    try expectParse(Color, "#f00f", .{ .r = 0xFF, .g = 0x00, .b = 0x00, .a = 0xFF });

    try expectParse(Color, "rgb(0, 0, 0)", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "rgba(0, 0, 0, 0)", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0x00 });
    try expectParse(Color, "rgba(0, 0, 0, 0%)", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0x00 });
    try expectParse(Color, "rgba(255, 128, 0, 1)", .{ .r = 0xFF, .g = 0x80, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "rgba(255, 128, 0, 100%)", .{ .r = 0xFF, .g = 0x80, .b = 0x00, .a = 0xFF });

    try expectParse(Color, "hsl(0, 0%, 0%)", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "hsla(0deg, 0%, 100%, 0%)", .{ .r = 0xFF, .g = 0xFF, .b = 0xFF, .a = 0x00 });
    try expectParse(Color, "hsla(120grad, 100%, 50%, 1)", .{ .r = 0x33, .g = 0xFF, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "hsla(2.0944rad, 100%, 50%, 0%)", .{ .r = 0x00, .g = 0xFF, .b = 0x00, .a = 0x00 });
    try expectParse(Color, "hsla(0.1667turn, 100%, 50%, 100%)", .{ .r = 0xFF, .g = 0xFF, .b = 0x00, .a = 0xFF });
    try expectParse(Color, "hsla(45deg, 100%, 50%, 50%)", .{ .r = 0xFF, .g = 0xBF, .b = 0x00, .a = 0x7F });

    try expectParse(Color, "transparent", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0x00 });
    try expectParse(Color, "black", .{ .r = 0x00, .g = 0x00, .b = 0x00, .a = 0xFF });

    try expectParse(Color, "xxx", error.InvalidColor);
}
