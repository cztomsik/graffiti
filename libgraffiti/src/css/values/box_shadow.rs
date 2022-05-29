use super::super::parsing::{sym, Parsable, Parser};
use super::{Color, Dimension};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    offset: (Dimension, Dimension),
    blur: Dimension,
    spread: Dimension,
    color: Color,
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}px {}px {}px {}px {}",
            self.offset.0, self.offset.1, self.blur, self.spread, self.color
        )
    }
}

impl Parsable for BoxShadow {
    fn parser<'a>() -> Parser<'a, Self> {
        let offset = Dimension::parser() - sym(" ") + Dimension::parser();
        let blur = Dimension::parser();
        let spread = (Dimension::parser() - sym(" "))
            .opt()
            .map(|d| d.unwrap_or(Dimension::ZERO));
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
    fn parse_transform() {
        assert_eq!(
            BoxShadow::parse("1px 1px 1px 1px #000"),
            Ok(BoxShadow {
                offset: (Dimension::Px(1.), Dimension::Px(1.)),
                blur: Dimension::Px(1.),
                spread: Dimension::Px(1.),
                color: Color::BLACK
            })
        );

        assert_eq!(
            BoxShadow::parse("0 0 10px #000"),
            Ok(BoxShadow {
                offset: (Dimension::ZERO, Dimension::ZERO),
                blur: Dimension::Px(10.),
                spread: Dimension::ZERO,
                color: Color::BLACK
            })
        );
    }
}
