use super::super::parsing::{sym, Parsable, Parser};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
    Em(f32),
    Rem(f32),
    Vw(f32),
    Vh(f32),
    Vmin,
    Vmax,
}

impl Dimension {
    pub const ZERO: Self = Self::Px(0.);
}

impl Parsable for Dimension {
    fn parser<'a>() -> Parser<'a, Self> {
        let px = (f32::parser() - sym("px")).map(Self::Px);
        let percent = (f32::parser() - sym("%")).map(Self::Percent);
        let auto = sym("auto").map(|_| Self::Auto);
        let zero = sym("0").map(|_| Self::ZERO);
        let em = (f32::parser() - sym("em")).map(Self::Em);
        let rem = (f32::parser() - sym("rem")).map(Self::Rem);
        let vw = (f32::parser() - sym("vw")).map(Self::Vw);
        let vh = (f32::parser() - sym("vh")).map(Self::Vh);
        let vmin = sym("vmin").map(|_| Self::Vmin);
        let vmax = sym("vmax").map(|_| Self::Vmax);

        px | percent | auto | zero | em | rem | vw | vh | vmin | vmax
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Px(v) => write!(f, "{}px", v),
            Self::Percent(v) => write!(f, "{}%", v),
            Self::Em(v) => write!(f, "{}em", v),
            Self::Rem(v) => write!(f, "{}rem", v),
            Self::Vw(v) => write!(f, "{}vw", v),
            Self::Vh(v) => write!(f, "{}vh", v),
            Self::Vmin => write!(f, "vmin"),
            Self::Vmax => write!(f, "vmax"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dimension() {
        assert_eq!(Dimension::parse("auto"), Ok(Dimension::Auto));
        assert_eq!(Dimension::parse("10px"), Ok(Dimension::Px(10.)));
        assert_eq!(Dimension::parse("100%"), Ok(Dimension::Percent(100.)));
        assert_eq!(Dimension::parse("1.2em"), Ok(Dimension::Em(1.2)));
        assert_eq!(Dimension::parse("2.1rem"), Ok(Dimension::Rem(2.1)));
        assert_eq!(Dimension::parse("0"), Ok(Dimension::Px(0.)));
        assert_eq!(Dimension::parse("100vw"), Ok(Dimension::Vw(100.)));
        assert_eq!(Dimension::parse("100vh"), Ok(Dimension::Vh(100.)));
        assert_eq!(Dimension::parse("vmin"), Ok(Dimension::Vmin));
        assert_eq!(Dimension::parse("vmax"), Ok(Dimension::Vmax));
    }
}
