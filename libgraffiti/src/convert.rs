// TODO: conversion macro for all enums?

use crate::{
    css::{self, BorderStyle, Display, Overflow, Style, StyleProp},
    layout::{self, LayoutStyle},
    renderer::{Color, ContainerStyle, Outline, Shadow, StrokeStyle},
};

pub fn container_style(style: &css::Style) -> ContainerStyle {
    let mut res = ContainerStyle::default();

    // (transient) initial_values
    let mut hidden = false;
    let mut radii = [0.; 4];
    let mut outline = (3., BorderStyle::None, Color::BLACK);

    for prop in style.props() {
        match prop {
            StyleProp::Transform(t) => todo!(),
            &StyleProp::Opacity(o) => res.opacity = Some(o),
            StyleProp::BorderTopLeftRadius(r) => radii[0] = r.0,
            StyleProp::BorderTopRightRadius(r) => radii[1] = r.0,
            StyleProp::BorderBottomLeftRadius(r) => radii[2] = r.0,
            StyleProp::BorderBottomRightRadius(r) => radii[3] = r.0,
            StyleProp::BoxShadow(s) => {
                res.shadow = Some(Shadow(
                    (s.offset.0 .0, s.offset.1 .0),
                    s.blur.0,
                    s.spread.0,
                    color(s.color),
                ))
            }
            StyleProp::OutlineWidth(w) => outline.0 = w.0,
            &StyleProp::OutlineStyle(s) => outline.1 = s,
            &StyleProp::OutlineColor(c) => outline.2 = color(c),
            StyleProp::OverflowX(v) | StyleProp::OverflowY(v) if matches!(v, Overflow::Hidden | Overflow::Scroll) => {
                res.clip = true
            }
            &StyleProp::BackgroundColor(c) => res.bg_color = Some(color(c)),
            &StyleProp::Display(Display::None) => hidden = true,
            _ => {}
        }
    }

    if radii != [0., 0., 0., 0.] {
        res.border_radii = Some(radii);
    }

    if outline.0 > 0. && outline.1 != BorderStyle::None && outline.2 != Color::TRANSPARENT {
        println!("TODO: outline-style");
        res.outline = Some(Outline(outline.0, StrokeStyle::Solid, outline.2))
    }

    if hidden {
        res.opacity = 0.
    }

    res
}

fn color(color: css::Color) -> Color {
    let css::Color { r, g, b, a } = color;

    Color::from_argb(a, r, g, b)
}

pub fn layout_style(style: &Style) -> LayoutStyle {
    let mut res = LayoutStyle::default();

    for prop in style.props() {
        use StyleProp::*;

        match prop {
            // size
            &Width(v) => res.size.width = dimension(v),
            &Height(v) => res.size.height = dimension(v),
            &MinWidth(v) => res.min_size.width = dimension(v),
            &MinHeight(v) => res.min_size.height = dimension(v),
            &MaxWidth(v) => res.max_size.width = dimension(v),
            &MaxHeight(v) => res.max_size.height = dimension(v),

            // padding
            &PaddingTop(v) => res.padding.top = dimension(v),
            &PaddingRight(v) => res.padding.right = dimension(v),
            &PaddingBottom(v) => res.padding.bottom = dimension(v),
            &PaddingLeft(v) => res.padding.left = dimension(v),

            // margin
            &MarginTop(v) => res.margin.top = dimension(v),
            &MarginRight(v) => res.margin.right = dimension(v),
            &MarginBottom(v) => res.margin.bottom = dimension(v),
            &MarginLeft(v) => res.margin.left = dimension(v),

            // border
            &BorderTopWidth(v) => res.border.top = v.0,
            &BorderRightWidth(v) => res.border.right = v.0,
            &BorderBottomWidth(v) => res.border.bottom = v.0,
            &BorderLeftWidth(v) => res.border.left = v.0,

            // position
            &Position(v) => res.position_type = position(v),
            &Top(v) => res.position.top = dimension(v),
            &Right(v) => res.position.right = dimension(v),
            &Bottom(v) => res.position.bottom = dimension(v),
            &Left(v) => res.position.left = dimension(v),

            // flex
            &FlexDirection(v) => res.flex_direction = flex_direction(v),
            &FlexWrap(v) => res.flex_wrap = flex_wrap(v),
            &FlexGrow(v) => res.flex_grow = v,
            &FlexShrink(v) => res.flex_shrink = v,
            &FlexBasis(v) => res.flex_basis = dimension(v),
            &AlignContent(v) => res.align_content = align(v),
            &AlignItems(v) => res.align_items = align(v),
            &AlignSelf(v) => res.align_self = align(v),
            &JustifyContent(v) => res.justify_content = justify(v),

            // other
            &Display(v) => res.display = display(v),

            _ => {}
        }
    }

    res
}

fn display(value: css::Display) -> layout::Display {
    match value {
        css::Display::None => layout::Display::None,
        css::Display::Flex => layout::Display::Flex,
        css::Display::Block => layout::Display::Block,
        css::Display::Inline => layout::Display::Inline,
        css::Display::InlineBlock => layout::Display::InlineBlock,
        css::Display::Table => layout::Display::Table,
        css::Display::TableRow => layout::Display::TableRow,
        css::Display::TableCell => layout::Display::TableCell,
        _ => layout::Display::Block,
    }
}

fn flex_direction(value: css::FlexDirection) -> layout::FlexDirection {
    match value {
        css::FlexDirection::Row => layout::FlexDirection::Row,
        css::FlexDirection::Column => layout::FlexDirection::Column,
        css::FlexDirection::RowReverse => todo!(),
        css::FlexDirection::ColumnReverse => todo!(),
    }
}

fn flex_wrap(value: css::FlexWrap) -> layout::FlexWrap {
    match value {
        css::FlexWrap::NoWrap => layout::FlexWrap::NoWrap,
        css::FlexWrap::Wrap => layout::FlexWrap::Wrap,
        css::FlexWrap::WrapReverse => todo!(),
    }
}

fn dimension(value: css::Dimension) -> layout::Dimension {
    match value {
        css::Dimension::Px(v) => layout::Dimension::Px(v),
        css::Dimension::Auto => layout::Dimension::Auto,
        css::Dimension::Percent(v) => layout::Dimension::Percent(v / 100.),
        css::Dimension::Vw(v) => layout::Dimension::Vw(v),
        css::Dimension::Vh(v) => layout::Dimension::Vh(v),
        _ => todo!(),
    }
}

fn align(value: css::Align) -> layout::Align {
    match value {
        css::Align::Auto => layout::Align::Auto,
        css::Align::FlexStart => layout::Align::FlexStart,
        css::Align::Center => layout::Align::Center,
        css::Align::FlexEnd => layout::Align::FlexEnd,
        css::Align::Stretch => layout::Align::Stretch,
        css::Align::Baseline => layout::Align::Baseline,
        css::Align::SpaceBetween => layout::Align::SpaceBetween,
        css::Align::SpaceAround => layout::Align::SpaceAround,
    }
}

fn justify(value: css::Justify) -> layout::Justify {
    match value {
        css::Justify::FlexStart => layout::Justify::FlexStart,
        css::Justify::Center => layout::Justify::Center,
        css::Justify::FlexEnd => layout::Justify::FlexEnd,
        css::Justify::SpaceBetween => layout::Justify::SpaceBetween,
        css::Justify::SpaceAround => layout::Justify::SpaceAround,
        css::Justify::SpaceEvenly => layout::Justify::SpaceEvenly,
    }
}

fn position(value: css::Position) -> layout::Position {
    match value {
        css::Position::Static => layout::Position::Static,
        css::Position::Relative => layout::Position::Relative,
        css::Position::Absolute => layout::Position::Absolute,
        // TODO
        css::Position::Sticky => layout::Position::Static,
    }
}
