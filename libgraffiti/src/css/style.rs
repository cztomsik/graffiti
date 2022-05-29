// TODO: there should be prop.id()
//       so we can first find id for given &str
//       and then just find the prop with simple eq check

use super::parsing::{any, skip, sym, Parsable, ParseError, Parser};
use super::StyleProp;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    props: Vec<StyleProp>,
    // TODO: important: u32 + 1 << prop.id() as u32 to figure out the bit to flip/check
}

impl Style {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Parsable::parse(input)
    }

    /*
    // jsdom squashes longhands into one shorthand (if all are present)
    // but chrome doesn't so I think we don't have to either
    pub fn length(&self) -> usize {
        self.props.len()
    }

    pub fn item(&self, index: usize) -> Option<&str> {
        self.props.get(index).map(StyleProp::css_name)
    }

    pub fn property_value(&self, prop: &str) -> Option<String> {
        if let Some(prop) = self.props.iter().find(|p| p.css_name() == prop) {
            return Some(prop.css_value());
        }

        self.shorthand_value(prop)
    }

    // TODO: priority
    pub fn set_property(&mut self, prop: &str, value: &str) {
        let tokens = super::parser::tokenize(value.as_bytes());

        super::parser::parse_prop_into(prop, &tokens, self);
    }

    pub fn remove_property(&mut self, prop: &str) {
        self.props.retain(|p| p.css_name() == prop);
    }

    */
    pub fn props(&self) -> impl Iterator<Item = &StyleProp> {
        self.props.iter()
    }

    pub(crate) fn add_prop(&mut self, new_prop: StyleProp) {
        let d = discriminant(&new_prop);

        if let Some(existing) = self.props.iter_mut().find(|p| d == discriminant(p)) {
            *existing = new_prop;
        } else {
            self.props.push(new_prop);
        }
    }
}

impl Parsable for Style {
    fn parser<'a>() -> Parser<'a, Self> {
        // any chunk of tokens before ";" or "}"
        let prop_value = (!sym(";") * !sym("}") * skip(1)).repeat(1..).collect();
        let prop = any() - sym(":") + prop_value - sym(";").discard().repeat(0..);

        prop.repeat(0..).map(|props| {
            let mut style = Self::default();

            for (p, v) in props {
                // skip unknown
                parse_prop_into(p, v, &mut style);
            }

            style
        })
    }
}

pub fn parse_prop_into(prop: &str, value: &[&str], style: &mut Style) {
    if let Ok(p) = super::properties::prop_parser(prop).parse(value) {
        style.add_prop(p);
    } /* else if let Ok(props) = shorthand_parser(prop).parse(value) {
          for p in props {
              style.add_prop(p);
          }
      }*/
}

// impl From<&str> for Style {
//     fn from(s: &str) -> Self {
//         Self::parse(s).unwrap_or_default()
//     }
// }

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in &self.props {
            write!(f, "{}:{};", p.css_name(), p.css_value())?;
        }

        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::super::CssDisplay;
    use super::*;

    #[test]
    fn css_text() {
        let s = Style::parse("display:block;").unwrap();
        assert_eq!(s.to_string(), "display:block;");
    }

    #[test]
    fn prop_overriding() {
        let mut s = Style::default();

        s.add_prop(StyleProp::Display(CssDisplay::None));
        s.add_prop(StyleProp::Display(CssDisplay::Block));

        assert!(s.props().eq(&vec![StyleProp::Display(CssDisplay::Block)]));
    }
}
*/
