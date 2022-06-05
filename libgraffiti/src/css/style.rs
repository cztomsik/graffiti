use super::parsing::{fail, ident, list, skip, sym, Parsable, ParseError, Parser};
use super::{BorderStyle, Color, Dimension, Overflow, Px, StyleProp};
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
            let shorthand = shorthand_parser() + important();

            for tokens in decls {
                // TODO: shorthands
                if let Ok((prop, important)) = longhand.parse(tokens) {
                    style.add_prop(prop, important);
                }

                if let Ok((props, important)) = shorthand.parse(tokens) {
                    for prop in props {
                        style.add_prop(prop, important);
                    }
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

fn shorthand_parser<'a>() -> Parser<'a, Vec<StyleProp>> {
    use StyleProp::*;

    ident() - sym(":")
        >> |name| match name {
            "background" => background(),
            "flex" => flex(),
            "overflow" => overflow(),
            "outline" => outline(),
            "padding" => rect(PaddingTop, PaddingRight, PaddingBottom, PaddingLeft),
            "margin" => rect(MarginTop, MarginRight, MarginBottom, MarginLeft),
            "border-radius" => rect(
                BorderTopLeftRadius,
                BorderTopRightRadius,
                BorderBottomRightRadius,
                BorderBottomLeftRadius,
            ),
            "border-width" => rect(BorderTopWidth, BorderRightWidth, BorderBottomWidth, BorderLeftWidth),
            "border-style" => rect(BorderTopStyle, BorderRightStyle, BorderBottomStyle, BorderLeftStyle),
            "border-color" => rect(BorderTopColor, BorderRightColor, BorderBottomColor, BorderLeftColor),
            _ => fail("unknown shorthand"),
        }
}

fn rect<'a, T: Parsable + Copy + 'a>(
    top: fn(T) -> StyleProp,
    right: fn(T) -> StyleProp,
    bottom: fn(T) -> StyleProp,
    left: fn(T) -> StyleProp,
) -> Parser<'a, Vec<StyleProp>> {
    list(T::parser(), sym(" ")).convert(move |sides| {
        #[allow(clippy::match_ref_pats)]
        let (a, b, c, d) = match &sides[..] {
            &[a, b, c, d] => (a, b, c, d),
            &[a, b, c] => (a, b, c, b),
            &[a, b] => (a, b, a, b),
            &[a] => (a, a, a, a),
            _ => return Err("expected 1-4 values"),
        };

        Ok(vec![top(a), right(b), bottom(c), left(d)])
    })
}

fn background<'a>() -> Parser<'a, Vec<StyleProp>> {
    (sym("none").map(|_| Color::TRANSPARENT) | Color::parser()).map(|c| vec![StyleProp::BackgroundColor(c)])
}

fn flex<'a>() -> Parser<'a, Vec<StyleProp>> {
    (f32::parser() + (sym(" ") * f32::parser()).opt() + (sym(" ") * Dimension::parser()).opt()).map(
        |((grow, shrink), basis)| {
            vec![
                StyleProp::FlexGrow(grow),
                StyleProp::FlexShrink(shrink.unwrap_or(1.)),
                StyleProp::FlexBasis(basis.unwrap_or(Dimension::ZERO)),
            ]
        },
    )
}

fn overflow<'a>() -> Parser<'a, Vec<StyleProp>> {
    (Overflow::parser() + (sym(" ") * Overflow::parser()).opt())
        .map(|(x, y)| vec![StyleProp::OverflowX(x), StyleProp::OverflowY(y.unwrap_or(x))])
}

fn outline<'a>() -> Parser<'a, Vec<StyleProp>> {
    (Px::parser() + (sym(" ") * BorderStyle::parser()) + (sym(" ") * Color::parser())).map(|((px, style), color)| {
        vec![
            StyleProp::OutlineWidth(px),
            StyleProp::OutlineStyle(style),
            StyleProp::OutlineColor(color),
        ]
    })
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

        assert_eq!(Style::parse("opacity: 0")?.props, &[StyleProp::Opacity(0.)]);

        assert_eq!(
            Style::parse("opacity: 0; opacity: unknown")?.props,
            &[StyleProp::Opacity(0.)]
        );

        assert_eq!(
            Style::parse("opacity: 0; flex-grow: 1")?.props,
            &[StyleProp::Opacity(0.), StyleProp::FlexGrow(1.)]
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
    fn parse_shorthands() -> Result<(), ParseError> {
        use super::super::{Color, Dimension, Overflow};
        use StyleProp::*;

        assert_eq!(
            &Style::parse("overflow: hidden")?.props,
            &[OverflowX(Overflow::Hidden), OverflowY(Overflow::Hidden)],
        );

        assert_eq!(
            &Style::parse("overflow: visible hidden")?.props,
            &[OverflowX(Overflow::Visible), OverflowY(Overflow::Hidden)],
        );

        assert_eq!(
            &Style::parse("flex: 1")?.props,
            &[FlexGrow(1.), FlexShrink(1.), FlexBasis(Dimension::ZERO)],
        );

        assert_eq!(
            &Style::parse("flex: 2 3 10px")?.props,
            &[FlexGrow(2.), FlexShrink(3.), FlexBasis(Dimension::Px(10.))],
        );

        assert_eq!(
            &Style::parse("padding: 0")?.props,
            &[
                PaddingTop(Dimension::ZERO),
                PaddingRight(Dimension::ZERO),
                PaddingBottom(Dimension::ZERO),
                PaddingLeft(Dimension::ZERO),
            ],
        );

        assert_eq!(
            &Style::parse("padding: 10px 20px")?.props,
            &[
                PaddingTop(Dimension::Px(10.)),
                PaddingRight(Dimension::Px(20.)),
                PaddingBottom(Dimension::Px(10.)),
                PaddingLeft(Dimension::Px(20.)),
            ],
        );

        assert_eq!(
            &Style::parse("border-radius: 1px 2px 3px 4px")?.props,
            &[
                BorderTopLeftRadius(Px(1.)),
                BorderTopRightRadius(Px(2.)),
                BorderBottomRightRadius(Px(3.)),
                BorderBottomLeftRadius(Px(4.)),
            ],
        );

        assert_eq!(
            &Style::parse("border-radius: 15px 50px")?.props,
            &[
                BorderTopLeftRadius(Px(15.)),
                BorderTopRightRadius(Px(50.)),
                BorderBottomRightRadius(Px(15.)),
                BorderBottomLeftRadius(Px(50.)),
            ],
        );

        assert_eq!(
            &Style::parse("background: none")?.props,
            &[StyleProp::BackgroundColor(Color::TRANSPARENT)],
        );
        assert_eq!(
            &Style::parse("background: #000")?.props,
            &[StyleProp::BackgroundColor(Color::BLACK)],
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

    #[test]
    #[ignore]
    fn test_size() {
        use super::super::{BoxShadow, Dimension};
        use crate::util::Atom;
        use std::mem::size_of;

        assert_eq!(size_of::<Box<BoxShadow>>(), size_of::<usize>());
        assert_eq!(size_of::<Atom>(), size_of::<usize>());

        assert_eq!(size_of::<Dimension>(), size_of::<(u32, f32)>());

        // TODO: gets broken when Atom<> or Box<> is added
        assert_eq!(size_of::<StyleProp>(), size_of::<(u8, Dimension)>());
    }
}
