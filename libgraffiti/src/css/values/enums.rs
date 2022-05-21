use std::fmt;

// generate `enum Xxx { A, B, ... }` declarations, including their trait impls
macro_rules! css_enums {
    ($( enum $name:ident { $($variant:ident = $value:literal,)* } )*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum $name { $($variant),* }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_str(match self { $(Self::$variant => $value),* })
                }
            }

            impl std::str::FromStr for $name {
                type Err = &'static str;

                fn from_str(v: &str) -> Result<Self, Self::Err> {
                    Ok(match v {
                        $($value => Self::$variant,)*
                        _ => return Err(stringify!(invalid $name))
                    })
                }
            }
        )*
    };
}

css_enums! {
    enum Align {
        Auto = "auto",
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
        Stretch = "stretch",
        Baseline = "baseline",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
    }

    enum Justify {
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
        SpaceEvenly = "space-evenly",
    }

    enum BorderStyle {
        None = "none",
        Hidden = "hidden",
        Dotted = "dotted",
        Dashed = "dashed",
        Solid = "solid",
        Double = "double",
        Groove = "groove",
        Ridge = "ridge",
        Inset = "inset",
        Outset = "outset",
    }

    enum Display {
        None = "none",
        Block = "block",
        Inline = "inline",
        InlineBlock = "inline-block",
        Flex = "flex",
        Table = "table",
        TableHeaderGroup = "table-header-group",
        TableRowGroup = "table-row-group",
        TableRow = "table-row",
        TableCell = "table-cell",
    }

    enum FlexDirection {
        Column = "column",
        ColumnReverse = "column-reverse",
        Row = "row",
        RowReverse = "row-reverse",
    }

    enum FlexWrap {
        NoWrap = "nowrap",
        Wrap = "wrap",
        WrapReverse = "wrap-reverse",
    }

    enum Overflow {
        Visible = "visible",
        Hidden = "hidden",
        Scroll = "scroll",
        Auto = "auto",
    }

    enum Position {
        Static = "static",
        Relative = "relative",
        Absolute = "absolute",
        Sticky = "sticky",
    }

    enum TextAlign {
        Left = "left",
        Right = "right",
        Center = "center",
        Justify = "justify",
    }

    enum Visibility {
        Visible = "visible",
        Hidden = "hidden",
        Collapse = "collapse",
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::parsing::Parsable;
    use super::*;

    #[test]
    fn parse_align() {
        assert_eq!(Align::parse("auto"), Ok(Align::Auto));
        //assert_eq!(Align::parse("start"), Ok(Align::Start));

        assert_eq!(Align::parse("flex-start"), Ok(Align::FlexStart));
        assert_eq!(Align::parse("center"), Ok(Align::Center));
        //assert_eq!(Align::parse("end"), Ok(Align::End));

        assert_eq!(Align::parse("flex-end"), Ok(Align::FlexEnd));
        assert_eq!(Align::parse("stretch"), Ok(Align::Stretch));
        assert_eq!(Align::parse("baseline"), Ok(Align::Baseline));
        assert_eq!(Align::parse("space-between"), Ok(Align::SpaceBetween));
        assert_eq!(Align::parse("space-around"), Ok(Align::SpaceAround));
        //assert_eq!(Align::parse("space-evenly"), Ok(Align::SpaceEvenly));
    }

    #[test]
    fn parse_justify() {
        //assert_eq!(Justify::parse("start"), Ok(Justify::Start));

        assert_eq!(Justify::parse("flex-start"), Ok(Justify::FlexStart));
        assert_eq!(Justify::parse("center"), Ok(Justify::Center));
        //assert_eq!(Justify::parse("end"), Ok(Justify::End));

        assert_eq!(Justify::parse("flex-end"), Ok(Justify::FlexEnd));
        assert_eq!(Justify::parse("space-between"), Ok(Justify::SpaceBetween));
        assert_eq!(Justify::parse("space-around"), Ok(Justify::SpaceAround));
        assert_eq!(Justify::parse("space-evenly"), Ok(Justify::SpaceEvenly));
    }

    #[test]
    fn parse_border_style() {
        assert_eq!(BorderStyle::parse("none"), Ok(BorderStyle::None));
        assert_eq!(BorderStyle::parse("hidden"), Ok(BorderStyle::Hidden));
        assert_eq!(BorderStyle::parse("dotted"), Ok(BorderStyle::Dotted));
        assert_eq!(BorderStyle::parse("dashed"), Ok(BorderStyle::Dashed));
        assert_eq!(BorderStyle::parse("solid"), Ok(BorderStyle::Solid));
        assert_eq!(BorderStyle::parse("double"), Ok(BorderStyle::Double));
        assert_eq!(BorderStyle::parse("groove"), Ok(BorderStyle::Groove));
        assert_eq!(BorderStyle::parse("ridge"), Ok(BorderStyle::Ridge));
        assert_eq!(BorderStyle::parse("inset"), Ok(BorderStyle::Inset));
        assert_eq!(BorderStyle::parse("outset"), Ok(BorderStyle::Outset));
    }

    #[test]
    fn parse_display() {
        assert_eq!(Display::parse("none"), Ok(Display::None));
        assert_eq!(Display::parse("block"), Ok(Display::Block));
        assert_eq!(Display::parse("inline"), Ok(Display::Inline));
        assert_eq!(Display::parse("inline-block"), Ok(Display::InlineBlock));
        assert_eq!(Display::parse("flex"), Ok(Display::Flex));
        assert_eq!(Display::parse("table"), Ok(Display::Table));
        assert_eq!(Display::parse("table-header-group"), Ok(Display::TableHeaderGroup));
        assert_eq!(Display::parse("table-row-group"), Ok(Display::TableRowGroup));
        assert_eq!(Display::parse("table-row"), Ok(Display::TableRow));
        assert_eq!(Display::parse("table-cell"), Ok(Display::TableCell));
    }

    #[test]
    fn parse_flex_direction() {
        assert_eq!(FlexDirection::parse("row"), Ok(FlexDirection::Row));
        assert_eq!(FlexDirection::parse("column"), Ok(FlexDirection::Column));
        assert_eq!(FlexDirection::parse("row-reverse"), Ok(FlexDirection::RowReverse));
        assert_eq!(FlexDirection::parse("column-reverse"), Ok(FlexDirection::ColumnReverse));
    }

    #[test]
    fn parse_flex_wrap() {
        assert_eq!(FlexWrap::parse("nowrap"), Ok(FlexWrap::NoWrap));
        assert_eq!(FlexWrap::parse("wrap"), Ok(FlexWrap::Wrap));
        assert_eq!(FlexWrap::parse("wrap-reverse"), Ok(FlexWrap::WrapReverse));
    }

    #[test]
    fn parse_overflow() {
        assert_eq!(Overflow::parse("visible"), Ok(Overflow::Visible));
        assert_eq!(Overflow::parse("hidden"), Ok(Overflow::Hidden));
        assert_eq!(Overflow::parse("scroll"), Ok(Overflow::Scroll));
        assert_eq!(Overflow::parse("auto"), Ok(Overflow::Auto));
    }

    #[test]
    fn parse_position() {
        assert_eq!(Position::parse("static"), Ok(Position::Static));
        assert_eq!(Position::parse("relative"), Ok(Position::Relative));
        assert_eq!(Position::parse("absolute"), Ok(Position::Absolute));
        assert_eq!(Position::parse("sticky"), Ok(Position::Sticky));
    }

    #[test]
    fn parse_text_align() {
        assert_eq!(TextAlign::parse("left"), Ok(TextAlign::Left));
        assert_eq!(TextAlign::parse("center"), Ok(TextAlign::Center));
        assert_eq!(TextAlign::parse("right"), Ok(TextAlign::Right));
        assert_eq!(TextAlign::parse("justify"), Ok(TextAlign::Justify));
    }

    #[test]
    fn parse_visibility() {
        assert_eq!(Visibility::parse("visible"), Ok(Visibility::Visible));
        assert_eq!(Visibility::parse("hidden"), Ok(Visibility::Hidden));
        assert_eq!(Visibility::parse("collapse"), Ok(Visibility::Collapse));
    }
}
