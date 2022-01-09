use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum CssDimension {
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

impl CssDimension {
    pub const ZERO: Self = Self::Px(0.);
}

impl fmt::Display for CssDimension {
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
