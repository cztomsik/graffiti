use super::super::parsing::{any, ident, sym, Parsable, Parser};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // just a few for easier testing
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "transparent" => Self::rgba(0, 0, 0, 0),
            // https://drafts.csswg.org/css-color/#named-colors
            "aliceblue" => Self::rgb(240, 248, 255),
            "antiquewhite" => Self::rgb(250, 235, 215),
            "aqua" => Self::rgb(0, 255, 255),
            "aquamarine" => Self::rgb(127, 255, 212),
            "azure" => Self::rgb(240, 255, 255),
            "beige" => Self::rgb(245, 245, 220),
            "bisque" => Self::rgb(255, 228, 196),
            "black" => Self::rgb(0, 0, 0),
            "blanchedalmond" => Self::rgb(255, 235, 205),
            "blue" => Self::rgb(0, 0, 255),
            "blueviolet" => Self::rgb(138, 43, 226),
            "brown" => Self::rgb(165, 42, 42),
            "burlywood" => Self::rgb(222, 184, 135),
            "cadetblue" => Self::rgb(95, 158, 160),
            "chartreuse" => Self::rgb(127, 255, 0),
            "chocolate" => Self::rgb(210, 105, 30),
            "coral" => Self::rgb(255, 127, 80),
            "cornflowerblue" => Self::rgb(100, 149, 237),
            "cornsilk" => Self::rgb(255, 248, 220),
            "crimson" => Self::rgb(220, 20, 60),
            "cyan" => Self::rgb(0, 255, 255),
            "darkblue" => Self::rgb(0, 0, 139),
            "darkcyan" => Self::rgb(0, 139, 139),
            "darkgoldenrod" => Self::rgb(184, 134, 11),
            "darkgray" => Self::rgb(169, 169, 169),
            "darkgreen" => Self::rgb(0, 100, 0),
            "darkgrey" => Self::rgb(169, 169, 169),
            "darkkhaki" => Self::rgb(189, 183, 107),
            "darkmagenta" => Self::rgb(139, 0, 139),
            "darkolivegreen" => Self::rgb(85, 107, 47),
            "darkorange" => Self::rgb(255, 140, 0),
            "darkorchid" => Self::rgb(153, 50, 204),
            "darkred" => Self::rgb(139, 0, 0),
            "darksalmon" => Self::rgb(233, 150, 122),
            "darkseagreen" => Self::rgb(143, 188, 143),
            "darkslateblue" => Self::rgb(72, 61, 139),
            "darkslategray" => Self::rgb(47, 79, 79),
            "darkslategrey" => Self::rgb(47, 79, 79),
            "darkturquoise" => Self::rgb(0, 206, 209),
            "darkviolet" => Self::rgb(148, 0, 211),
            "deeppink" => Self::rgb(255, 20, 147),
            "deepskyblue" => Self::rgb(0, 191, 255),
            "dimgray" => Self::rgb(105, 105, 105),
            "dimgrey" => Self::rgb(105, 105, 105),
            "dodgerblue" => Self::rgb(30, 144, 255),
            "firebrick" => Self::rgb(178, 34, 34),
            "floralwhite" => Self::rgb(255, 250, 240),
            "forestgreen" => Self::rgb(34, 139, 34),
            "fuchsia" => Self::rgb(255, 0, 255),
            "gainsboro" => Self::rgb(220, 220, 220),
            "ghostwhite" => Self::rgb(248, 248, 255),
            "gold" => Self::rgb(255, 215, 0),
            "goldenrod" => Self::rgb(218, 165, 32),
            "gray" => Self::rgb(128, 128, 128),
            "green" => Self::rgb(0, 128, 0),
            "greenyellow" => Self::rgb(173, 255, 47),
            "grey" => Self::rgb(128, 128, 128),
            "honeydew" => Self::rgb(240, 255, 240),
            "hotpink" => Self::rgb(255, 105, 180),
            "indianred" => Self::rgb(205, 92, 92),
            "indigo" => Self::rgb(75, 0, 130),
            "ivory" => Self::rgb(255, 255, 240),
            "khaki" => Self::rgb(240, 230, 140),
            "lavender" => Self::rgb(230, 230, 250),
            "lavenderblush" => Self::rgb(255, 240, 245),
            "lawngreen" => Self::rgb(124, 252, 0),
            "lemonchiffon" => Self::rgb(255, 250, 205),
            "lightblue" => Self::rgb(173, 216, 230),
            "lightcoral" => Self::rgb(240, 128, 128),
            "lightcyan" => Self::rgb(224, 255, 255),
            "lightgoldenrodyellow" => Self::rgb(250, 250, 210),
            "lightgray" => Self::rgb(211, 211, 211),
            "lightgreen" => Self::rgb(144, 238, 144),
            "lightgrey" => Self::rgb(211, 211, 211),
            "lightpink" => Self::rgb(255, 182, 193),
            "lightsalmon" => Self::rgb(255, 160, 122),
            "lightseagreen" => Self::rgb(32, 178, 170),
            "lightskyblue" => Self::rgb(135, 206, 250),
            "lightslategray" => Self::rgb(119, 136, 153),
            "lightslategrey" => Self::rgb(119, 136, 153),
            "lightsteelblue" => Self::rgb(176, 196, 222),
            "lightyellow" => Self::rgb(255, 255, 224),
            "lime" => Self::rgb(0, 255, 0),
            "limegreen" => Self::rgb(50, 205, 50),
            "linen" => Self::rgb(250, 240, 230),
            "magenta" => Self::rgb(255, 0, 255),
            "maroon" => Self::rgb(128, 0, 0),
            "mediumaquamarine" => Self::rgb(102, 205, 170),
            "mediumblue" => Self::rgb(0, 0, 205),
            "mediumorchid" => Self::rgb(186, 85, 211),
            "mediumpurple" => Self::rgb(147, 112, 219),
            "mediumseagreen" => Self::rgb(60, 179, 113),
            "mediumslateblue" => Self::rgb(123, 104, 238),
            "mediumspringgreen" => Self::rgb(0, 250, 154),
            "mediumturquoise" => Self::rgb(72, 209, 204),
            "mediumvioletred" => Self::rgb(199, 21, 133),
            "midnightblue" => Self::rgb(25, 25, 112),
            "mintcream" => Self::rgb(245, 255, 250),
            "mistyrose" => Self::rgb(255, 228, 225),
            "moccasin" => Self::rgb(255, 228, 181),
            "navajowhite" => Self::rgb(255, 222, 173),
            "navy" => Self::rgb(0, 0, 128),
            "oldlace" => Self::rgb(253, 245, 230),
            "olive" => Self::rgb(128, 128, 0),
            "olivedrab" => Self::rgb(107, 142, 35),
            "orange" => Self::rgb(255, 165, 0),
            "orangered" => Self::rgb(255, 69, 0),
            "orchid" => Self::rgb(218, 112, 214),
            "palegoldenrod" => Self::rgb(238, 232, 170),
            "palegreen" => Self::rgb(152, 251, 152),
            "paleturquoise" => Self::rgb(175, 238, 238),
            "palevioletred" => Self::rgb(219, 112, 147),
            "papayawhip" => Self::rgb(255, 239, 213),
            "peachpuff" => Self::rgb(255, 218, 185),
            "peru" => Self::rgb(205, 133, 63),
            "pink" => Self::rgb(255, 192, 203),
            "plum" => Self::rgb(221, 160, 221),
            "powderblue" => Self::rgb(176, 224, 230),
            "purple" => Self::rgb(128, 0, 128),
            "rebeccapurple" => Self::rgb(102, 51, 153),
            "red" => Self::rgb(255, 0, 0),
            "rosybrown" => Self::rgb(188, 143, 143),
            "royalblue" => Self::rgb(65, 105, 225),
            "saddlebrown" => Self::rgb(139, 69, 19),
            "salmon" => Self::rgb(250, 128, 114),
            "sandybrown" => Self::rgb(244, 164, 96),
            "seagreen" => Self::rgb(46, 139, 87),
            "seashell" => Self::rgb(255, 245, 238),
            "sienna" => Self::rgb(160, 82, 45),
            "silver" => Self::rgb(192, 192, 192),
            "skyblue" => Self::rgb(135, 206, 235),
            "slateblue" => Self::rgb(106, 90, 205),
            "slategray" => Self::rgb(112, 128, 144),
            "slategrey" => Self::rgb(112, 128, 144),
            "snow" => Self::rgb(255, 250, 250),
            "springgreen" => Self::rgb(0, 255, 127),
            "steelblue" => Self::rgb(70, 130, 180),
            "tan" => Self::rgb(210, 180, 140),
            "teal" => Self::rgb(0, 128, 128),
            "thistle" => Self::rgb(216, 191, 216),
            "tomato" => Self::rgb(255, 99, 71),
            "turquoise" => Self::rgb(64, 224, 208),
            "violet" => Self::rgb(238, 130, 238),
            "wheat" => Self::rgb(245, 222, 179),
            "white" => Self::rgb(255, 255, 255),
            "whitesmoke" => Self::rgb(245, 245, 245),
            "yellow" => Self::rgb(255, 255, 0),
            "yellowgreen" => Self::rgb(154, 205, 50),
            _ => return None,
        })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b, a } = self;
        write!(f, "rgba({}, {}, {}, {})", r, g, b, a)
    }
}

impl Parsable for Color {
    fn parser<'a>() -> Parser<'a, Self> {
        fn hex_val(byte: u8) -> u8 {
            (byte as char).to_digit(16).unwrap() as u8
        }

        let hex_color = sym("#")
            * any().convert(|hex: &str| {
                let hex = hex.as_bytes();

                Ok(match hex.len() {
                    8 | 6 => {
                        let mut num = u32::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();

                        if hex.len() == 6 {
                            num = num << 8 | 0xFF;
                        }

                        Self {
                            r: ((num >> 24) & 0xFF) as u8,
                            g: ((num >> 16) & 0xFF) as u8,
                            b: ((num >> 8) & 0xFF) as u8,
                            a: (num & 0xFF) as u8,
                        }
                    }

                    4 | 3 => Self {
                        r: hex_val(hex[0]) * 17,
                        g: hex_val(hex[1]) * 17,
                        b: hex_val(hex[2]) * 17,
                        a: hex.get(3).map_or(255, |&v| hex_val(v) * 17),
                    },

                    _ => return Err("invalid hex color"),
                })
            });

        let rgb = sym("rgb")
            * sym("(")
            * (u8::parser() - sym(",") + u8::parser() - sym(",") + u8::parser()).map(|((r, g), b)| Self::rgb(r, g, b))
            - sym(")");

        let rgba = sym("rgba")
            * sym("(")
            * (u8::parser() - sym(",") + u8::parser() - sym(",") + u8::parser() - sym(",") + f32::parser())
                .map(|(((r, g), b), a)| Self::rgba(r, g, b, (255. * a) as _))
            - sym(")");

        let named_color = ident().convert(|name| Self::named(name).ok_or("unknown named color"));

        hex_color | rgb | rgba | named_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(Color::parse("#000000"), Ok(Color::BLACK));
        assert_eq!(Color::parse("#ff0000"), Ok(Color::RED));
        assert_eq!(Color::parse("#00ff00"), Ok(Color::GREEN));
        assert_eq!(Color::parse("#0000ff"), Ok(Color::BLUE));

        assert_eq!(Color::parse("#80808080"), Ok(Color::rgba(128, 128, 128, 128)));
        assert_eq!(Color::parse("#00000080"), Ok(Color::rgba(0, 0, 0, 128)));

        assert_eq!(Color::parse("#000"), Ok(Color::BLACK));
        assert_eq!(Color::parse("#f00"), Ok(Color::RED));
        assert_eq!(Color::parse("#fff"), Ok(Color::WHITE));

        assert_eq!(Color::parse("#0000"), Ok(Color::TRANSPARENT));
        assert_eq!(Color::parse("#f00f"), Ok(Color::RED));

        assert_eq!(Color::parse("rgb(0, 0, 0)"), Ok(Color::BLACK));
        assert_eq!(Color::parse("rgba(0, 0, 0, 0)"), Ok(Color::TRANSPARENT));

        assert_eq!(Color::parse("transparent"), Ok(Color::TRANSPARENT));
        assert_eq!(Color::parse("black"), Ok(Color::BLACK));
    }
}
