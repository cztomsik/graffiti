import * as g from '../core/generated'
import { parseColor } from '../core/utils'

// for each StyleProp, there should be one resolver which takes
// style and returns payload to appropriate SetStyleProp
//
// TODO: find out if mapped types could be used for enforcing this

export default {
  Size: ({ width = 'auto', height = 'auto' }) =>
    g.StyleProp.Size(g.Size(parseDimension(width), parseDimension(height))),

  Flex: ({
    flex = 0,
    flexGrow = flex,
    flexShrink = flex,
    // should be just 0 but chrome does percents too
    flexBasis = flex ? '0%' : 'auto'
  }) =>
    g.StyleProp.Flex({
      flexGrow,
      flexShrink,
      flexBasis: parseDimension(flexBasis)
    }),

  Flow: ({
    flexDirection = 'column',
    flexWrap = 'no-wrap',
    alignContent = 'flex-start',
    alignItems = 'stretch',
    alignSelf = 'auto',
    justifyContent = 'flex-start'
  }) =>
    g.StyleProp.Flow({
      flexDirection: FLEX_DIRECTION[flexDirection],
      flexWrap: FLEX_WRAP[flexWrap],
      alignContent: FLEX_ALIGN[alignContent],
      alignItems: FLEX_ALIGN[alignItems],
      alignSelf: FLEX_ALIGN[alignSelf],
      justifyContent: JUSTIFY_CONTENT[justifyContent]
    }),

  Padding: ({
    padding = 0,
    paddingHorizontal = padding,
    paddingVertical = padding,
    paddingTop = paddingVertical,
    paddingBottom = paddingVertical,
    paddingLeft = paddingHorizontal,
    paddingRight = paddingHorizontal
  }) =>
    g.StyleProp.Padding(
      g.Dimensions(
        parseDimension(paddingTop),
        parseDimension(paddingRight),
        parseDimension(paddingBottom),
        parseDimension(paddingLeft)
      )
    ),

  Margin: ({
    margin = 0,
    marginHorizontal = margin,
    marginVertical = margin,
    marginTop = marginVertical,
    marginBottom = marginVertical,
    marginLeft = marginHorizontal,
    marginRight = marginHorizontal
  }) =>
    g.StyleProp.Margin(
      g.Dimensions(
        parseDimension(marginTop),
        parseDimension(marginRight),
        parseDimension(marginBottom),
        parseDimension(marginLeft)
      )
    ),

  BorderRadius: ({
    borderRadius = 0,
    borderTopLeftRadius = borderRadius,
    borderTopRightRadius = borderRadius,
    borderBottomLeftRadius = borderRadius,
    borderBottomRightRadius = borderRadius
  }) =>
    g.StyleProp.BorderRadius(
      borderTopLeftRadius || borderTopRightRadius || borderBottomLeftRadius || borderBottomRightRadius
        ? [borderTopLeftRadius, borderTopRightRadius, borderBottomLeftRadius, borderBottomRightRadius]
        : undefined
    ),

  Border: ({
    // TODO: BorderStyle
    borderWidth = 0,
    borderColor = '#000000',
    borderTopWidth = borderWidth,
    borderRightWidth = borderWidth,
    borderBottomWidth = borderWidth,
    borderLeftWidth = borderWidth,
    borderTopColor = borderColor,
    borderRightColor = borderColor,
    borderBottomColor = borderColor,
    borderLeftColor = borderColor
  }) =>
    g.StyleProp.Border(
      borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth
        ? {
            top: {
              width: borderTopWidth,
              color: parseColor(borderTopColor),
              style: g.BorderStyle.Solid
            },
            right: {
              width: borderRightWidth,
              color: parseColor(borderRightColor),
              style: g.BorderStyle.Solid
            },
            bottom: {
              width: borderBottomWidth,
              color: parseColor(borderBottomColor),
              style: g.BorderStyle.Solid
            },
            left: {
              width: borderLeftWidth,
              color: parseColor(borderLeftColor),
              style: g.BorderStyle.Solid
            }
          }
        : undefined
    ),

  BoxShadow: ({
    shadowColor,
    //shadowOffset,
    //shadowOpacity,
    shadowRadius = 0,
    shadowSpread = 0
  }) =>
    g.StyleProp.BoxShadow(
      shadowColor
        ? {
            blur: shadowRadius,
            spread: shadowSpread,
            color: parseColor(shadowColor),
            offset: g.Vector2f(0, 0)
          }
        : undefined
    ),

  BackgroundColor: ({ backgroundColor }) =>
    g.StyleProp.BackgroundColor(backgroundColor ? parseColor(backgroundColor) : undefined),

  Text: ({ content, fontSize = 16, color = '#000000', lineHeight = 30, textAlign = 'left' }) =>
      typeof content === 'string'
        ? {
            tag: 'Text',
            //color: parseColor(color),
            size: fontSize,
            line_height: lineHeight,
            align: TEXT_ALIGN[textAlign],
            text: content
          }
        : undefined
  ,

  Image: ({ backgroundImageUrl }) => g.StyleProp.Image(backgroundImageUrl ? { url: backgroundImageUrl } : undefined),

  Overflow: ({
    // FlexStyle contains 'scroll' too, but ImageStyle does not
    overflow = 'visible'
  }) => g.StyleProp.Overflow(OVERFLOW[overflow])
}

function parseDimension(value?: string | number): g.Dimension {
  value = '' + value

  if (value.endsWith('%')) {
    return g.Dimension.Percent(parseFloat(value))
  }

  if (value === 'auto' || value === undefined) {
    return g.Dimension.Auto
  }

  return g.Dimension.Point(parseFloat(value))
}

const OVERFLOW = {
  visible: g.Overflow.Visible,
  hidden: g.Overflow.Hidden,
  scroll: g.Overflow.Scroll
}

const FLEX_DIRECTION = {
  column: g.FlexDirection.Column,
  'column-reverse': g.FlexDirection.ColumnReverse,
  row: g.FlexDirection.Row,
  'row-reverse': g.FlexDirection.RowReverse
}

const FLEX_WRAP = {
  nowrap: g.FlexWrap.NoWrap,
  'no-wrap': g.FlexWrap.NoWrap,
  wrap: g.FlexWrap.Wrap,
  'wrap-reverse': g.FlexWrap.WrapReverse
}

const FLEX_ALIGN = {
  auto: g.FlexAlign.Auto,
  'flex-start': g.FlexAlign.FlexStart,
  center: g.FlexAlign.Center,
  'flex-end': g.FlexAlign.FlexEnd,
  stretch: g.FlexAlign.Stretch,
  baseline: g.FlexAlign.Baseline,
  'space-between': g.FlexAlign.SpaceBetween,
  'space-around': g.FlexAlign.SpaceAround
}

const JUSTIFY_CONTENT = {
  'flex-start': g.JustifyContent.FlexStart,
  center: g.JustifyContent.Center,
  'flex-end': g.JustifyContent.FlexEnd,
  'space-between': g.JustifyContent.SpaceBetween,
  'space-around': g.JustifyContent.SpaceAround,
  'space-evenly': g.JustifyContent.SpaceEvenly
}

const TEXT_ALIGN = {
  left: g.TextAlign.Left,
  center: g.TextAlign.Center,
  right: g.TextAlign.Right
}
