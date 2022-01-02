use std::convert::TryFrom;
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

            impl <'a> TryFrom<&'a str> for $name {
                type Error = &'static str;

                fn try_from(v: &str) -> Result<Self, Self::Error> {
                    Ok(match v {
                        $($value => Self::$variant,)*
                        _ => return Err("invalid input")
                    })
                }
            }
        )*
    };
}

css_enums! {
    enum CssAlign {
        Auto = "auto",
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
        Stretch = "stretch",
        Baseline = "baseline",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
    }

    enum CssJustify {
        FlexStart = "flex-start",
        Center = "center",
        FlexEnd = "flex-end",
        SpaceBetween = "space-between",
        SpaceAround = "space-around",
        SpaceEvenly = "space-evenly",
    }

    enum CssBorderStyle {
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

    enum CssDisplay {
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

    enum CssFlexDirection {
        Column = "column",
        ColumnReverse = "column-reverse",
        Row = "row",
        RowReverse = "row-reverse",
    }

    enum CssFlexWrap {
        NoWrap = "nowrap",
        Wrap = "wrap",
        WrapReverse = "wrap-reverse",
    }

    enum CssOverflow {
        Visible = "visible",
        Hidden = "hidden",
        Scroll = "scroll",
        Auto = "auto",
    }

    enum CssPosition {
        Static = "static",
        Relative = "relative",
        Absolute = "absolute",
        Sticky = "sticky",
    }

    enum CssTextAlign {
        Left = "left",
        Right = "right",
        Center = "center",
        Justify = "justify",
    }

    enum CssVisibility {
        Visible = "visible",
        Hidden = "hidden",
        Collapse = "collapse",
    }
}
