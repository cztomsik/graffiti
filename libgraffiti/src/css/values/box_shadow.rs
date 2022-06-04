use super::super::parsing::{sym, Parsable, Parser};
use super::{Color, Px};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    pub offset: (Px, Px),
    pub blur: Px,
    pub spread: Px,
    pub color: Color,
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.offset.0, self.offset.1, self.blur, self.spread, self.color
        )
    }
}

impl Parsable for BoxShadow {
    fn parser<'a>() -> Parser<'a, Self> {
        let offset = Px::parser() - sym(" ") + Px::parser();
        let blur = Px::parser();
        let spread = (Px::parser() - sym(" ")).opt().map(|d| d.unwrap_or(Px(0.)));
        let shadow = offset - sym(" ") + blur - sym(" ") + spread + Color::parser();

        shadow.map(|(((offset, blur), spread), color)| Self {
            offset,
            blur,
            spread,
            color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_shadow() {
        assert_eq!(
            BoxShadow::parse("1px 1px 1px 1px #000"),
            Ok(BoxShadow {
                offset: (Px(1.), Px(1.)),
                blur: Px(1.),
                spread: Px(1.),
                color: Color::BLACK
            })
        );

        assert_eq!(
            BoxShadow::parse("0 0 10px #000"),
            Ok(BoxShadow {
                offset: (Px(0.), Px(0.)),
                blur: Px(10.),
                spread: Px(0.),
                color: Color::BLACK
            })
        );
    }
}
