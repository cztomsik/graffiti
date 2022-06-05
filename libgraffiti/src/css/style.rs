use super::parsing::{skip, sym, Parsable, ParseError, Parser};
use super::StyleProp;
use smallbitvec::SmallBitVec;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    props: Vec<StyleProp>,
    // TODO: inherited_props?
    important_props: SmallBitVec,
}

impl Style {
    pub const EMPTY: Self = Style {
        props: Vec::new(),
        important_props: SmallBitVec::new(),
    };

    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Parsable::parse(input)
    }

    pub fn props(&self) -> &[StyleProp] {
        &self.props
    }

    pub fn apply(&mut self, other: &Style) {
        for (i, p) in other.props.iter().enumerate() {
            self.add_prop(p.clone(), other.important_props[i]);
        }
    }

    fn add_prop(&mut self, prop: StyleProp, important: bool) {
        let d = discriminant(&prop);

        if let Some(i) = self.props.iter().position(|p| discriminant(p) == d) {
            if important || !self.important_props[i] {
                self.props[i] = prop;
                self.important_props.set(i, important);
            }
        } else {
            self.props.push(prop);
            self.important_props.push(important);
        }
    }
}

impl Parsable for Style {
    fn parser<'a>() -> Parser<'a, Self> {
        // any chunk of tokens before ";" or "}"
        let prop_decl = ((!sym(";") * !sym("}") * skip(1)).repeat(1..)).collect() - sym(";").opt();
        let important = || sym("!important").opt().map(|o| o.is_some());

        prop_decl.repeat(0..).map(move |decls| {
            let mut style = Self::default();
            let longhand = StyleProp::parser() + important();

            for d in decls {
                // TODO: shorthands
                if let Ok((prop, important)) = longhand.parse(d) {
                    style.add_prop(prop, important);
                }
            }

            style
        })
    }
}

// impl From<&str> for Style {
//     fn from(s: &str) -> Self {
//         Self::parse(s).unwrap_or_default()
//     }
// }

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, p) in self.props.iter().enumerate() {
            if i != 0 {
                write!(f, "; ")?;
            }

            write!(f, "{}", p)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallbitvec::sbvec;

    #[test]
    fn parse_style() -> Result<(), ParseError> {
        assert_eq!(Style::parse("")?, Style::EMPTY);
        assert_eq!(Style::parse("unknown-a: 0; unknown-b: 0")?, Style::EMPTY);
        assert_eq!(Style::parse("!important")?, Style::EMPTY);

        assert_eq!(Style::parse("opacity: 0")?.props, vec![StyleProp::Opacity(0.)]);

        assert_eq!(
            Style::parse("opacity: 0; opacity: unknown")?.props,
            vec![StyleProp::Opacity(0.)]
        );

        assert_eq!(
            Style::parse("opacity: 0; flex-grow: 1")?.props,
            vec![StyleProp::Opacity(0.), StyleProp::FlexGrow(1.)]
        );

        assert_eq!(
            Style::parse("opacity: 0 !important")?,
            Style {
                props: vec![StyleProp::Opacity(0.)],
                important_props: sbvec![true]
            }
        );

        Ok(())
    }

    #[test]
    fn css_text() {
        let s = Style::parse("display: block").unwrap();
        assert_eq!(s.to_string(), "display: block");
    }

    #[test]
    fn prop_overriding() {
        let mut s = Style::default();

        s.add_prop(StyleProp::Opacity(0.), false);
        s.add_prop(StyleProp::Opacity(1.), false);

        assert_eq!(s.props, vec![StyleProp::Opacity(1.)]);

        s.add_prop(StyleProp::Opacity(0.), true);
        s.add_prop(StyleProp::Opacity(1.), false);

        assert_eq!(s.props, vec![StyleProp::Opacity(0.)]);
    }

    #[test]
    fn apply() {
        let mut s = Style::default();
        s.apply(&Style::parse("display: block; width: 10px; height: 10px").unwrap());
        s.apply(&Style::parse("display: flex; height: 20px !important").unwrap());
        s.apply(&Style::parse("height: 30px").unwrap());
        assert_eq!(s.to_string(), "display: flex; width: 10px; height: 20px");
    }
}
