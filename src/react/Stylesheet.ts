import {
  RNStyleSheet,
  ViewStyle,
  TextStyle,
  ImageStyle,
} from './react-native-types'
import {
  Dimensions,
  Dimension,
  Size,
  FlexDirection,
  JustifyContent,
  FlexAlign,
  FlexWrap,
  BorderStyle,
  Vector2f,
  Flex,
  Flow,
  BorderRadius,
  BoxShadow,
  Border,
  Color,
  Overflow,
  Image
} from '../core/generated'
import { parseColor } from '../core/utils'

// needed for Stylesheet.create() autocompletion
type Styles = {
  [key: string]: FlatStyle
}

type FlatStyle = ViewStyle | (TextStyle & ImageStyle)

const create = (obj: Styles) => {
  for (const k in obj) {
    Object.freeze(obj[k])
  }

  return obj
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles) ? Object.assign({}, ...styles.map(flatten)) : styles
}

// any is needed here because TS is too dumb
const absoluteFillObject: any = {
  position: 'absolute',
  left: 0,
  right: 0,
  top: 0,
  bottom: 0
}

const StyleSheet = {
  // note that react-native does not return numbers anymore,
  flatten,
  create,

  setStyleAttributePreprocessor: () => void(0),
  hairlineWidth: 1,
  absoluteFillObject,
  absoluteFill: absoluteFillObject as any
}

export default StyleSheet

export function compileFlatStyle(style: FlatStyle): SurfaceProps {
  const {
    width = 'auto',
    height = 'auto',
    flex = 0,
    flexDirection = 'column',
    padding = 0,
    margin = 0,
    alignContent = 'flex-start',
    alignItems = 'stretch',
    alignSelf = 'auto',
    justifyContent = 'flex-start',
    flexWrap = 'no-wrap',
    // FlexStyle contains 'scroll' too, but ImageStyle does not
    overflow = 'visible',

    shadowColor,
    //shadowOffset,
    //shadowOpacity,
    shadowRadius = 0,
    shadowSpread = 0,
    backgroundColor,
    backgroundImageUrl,
    // TODO: BorderStyle
    borderRadius = 0,
    borderColor = '#000000',
    borderWidth = 0,

    ...rest
  } = style

  const {
    marginHorizontal = margin,
    marginVertical = margin,
    paddingHorizontal = padding,
    paddingVertical = padding,

    borderTopWidth = borderWidth,
    borderRightWidth = borderWidth,
    borderBottomWidth = borderWidth,
    borderLeftWidth = borderWidth,

    borderTopColor = borderColor,
    borderRightColor = borderColor,
    borderBottomColor = borderColor,
    borderLeftColor = borderColor,

    borderTopLeftRadius = borderRadius,
    borderTopRightRadius = borderRadius,
    borderBottomLeftRadius = borderRadius,
    borderBottomRightRadius = borderRadius,

    ...rest2
  } = rest

  const {
    flexGrow = flex,
    flexShrink = flex,
    // should be just 0 but chrome does percents too
    flexBasis = flex ? '0%' : 'auto',

    marginTop = marginVertical,
    marginBottom = marginVertical,
    marginLeft = marginHorizontal,
    marginRight = marginHorizontal,

    paddingTop = paddingVertical,
    paddingBottom = paddingVertical,
    paddingLeft = paddingHorizontal,
    paddingRight = paddingHorizontal
  } = rest2

  return {
    overflow: Overflow[OVERFLOW[overflow]],
    size: Size.mk(parseDimension(width), parseDimension(height)),
    flex: {
      flexGrow,
      flexShrink,
      flexBasis: parseDimension(flexBasis)
    },
    flow: {
      flexDirection: FlexDirection[FLEX_DIRECTION[flexDirection]],
      flexWrap: FlexWrap[FLEX_WRAP[flexWrap]],
      alignContent: FlexAlign[FLEX_ALIGN[alignContent]],
      alignItems: FlexAlign[FLEX_ALIGN[alignItems]],
      alignSelf: FlexAlign[FLEX_ALIGN[alignSelf]],
      justifyContent: JustifyContent[JUSTIFY_CONTENT[justifyContent]]
    },
    padding: Dimensions.mk(
      parseDimension(paddingTop),
      parseDimension(paddingRight),
      parseDimension(paddingBottom),
      parseDimension(paddingLeft)
    ),
    margin: Dimensions.mk(
      parseDimension(marginTop),
      parseDimension(marginRight),
      parseDimension(marginBottom),
      parseDimension(marginLeft)
    ),
    borderRadius: (borderTopLeftRadius || borderTopRightRadius || borderBottomLeftRadius || borderBottomRightRadius) ?[
      borderTopLeftRadius, borderTopRightRadius, borderBottomLeftRadius, borderBottomRightRadius
    ] :undefined,
    boxShadow: shadowColor
      ? {
          blur: shadowRadius,
          spread: shadowSpread,
          color: parseColor(shadowColor),
          offset: Vector2f.mk(0, 0)
        }
      : undefined,
    backgroundColor: backgroundColor ? parseColor(backgroundColor) : undefined,
    image: backgroundImageUrl ? { url: backgroundImageUrl } : undefined,
    //text,
    border:
      borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth
        ? {
            top: {
              width: borderTopWidth,
              color: parseColor(borderTopColor),
              style: BorderStyle.Solid
            },
            right: {
              width: borderRightWidth,
              color: parseColor(borderRightColor),
              style: BorderStyle.Solid
            },
            bottom: {
              width: borderBottomWidth,
              color: parseColor(borderBottomColor),
              style: BorderStyle.Solid
            },
            left: {
              width: borderLeftWidth,
              color: parseColor(borderLeftColor),
              style: BorderStyle.Solid
            }
          }
        : undefined
  }
}

function parseDimension(value?: string | number): Dimension {
  value = '' + value

  if (value.endsWith('%')) {
    return Dimension.Percent(parseFloat(value))
  }

  if (value === 'auto' || value === undefined) {
    return Dimension.Auto
  }

  return Dimension.Point(parseFloat(value))
}

const OVERFLOW = {
  visible: 'Visible',
  hidden: 'Hidden',
  scroll: 'Scroll'
}

const FLEX_DIRECTION = {
  column: 'Column',
  'column-reverse': 'ColumnReverse',
  row: 'Row',
  'row-reverse': 'RowReverse'
}

const FLEX_WRAP = {
  nowrap: 'NoWrap',
  'no-wrap': 'NoWrap',
  wrap: 'Wrap',
  'wrap-reverse': 'WrapReverse'
}

const FLEX_ALIGN = {
  auto: 'Auto',
  'flex-start': 'FlexStart',
  center: 'Center',
  'flex-end': 'FlexEnd',
  stretch: 'Stretch',
  baseline: 'Baseline',
  'space-between': 'SpaceBetween',
  'space-around': 'SpaceAround'
}

const JUSTIFY_CONTENT = {
  'flex-start': 'FlexStart',
  center: 'Center',
  'flex-end': 'FlexEnd',
  'space-between': 'SpaceBetween',
  'space-around': 'SpaceAround',
  'space-evenly': 'SpaceEvenly'
}

export interface SurfaceProps {
  overflow?: Overflow,
  size?: Size
  flex?: Flex
  flow?: Flow
  padding?: any
  margin?: any
  borderRadius?: BorderRadius
  boxShadow?: BoxShadow
  backgroundColor?: Color
  image?: Image
  text?: Text
  border?: Border
  children?: any
  listeners?: any
}
