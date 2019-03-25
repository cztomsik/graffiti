import {
  RNStyleSheet,
  ViewStyle,
  TextStyle,
  ImageStyle
} from './react-native-types'
import {
  Rect,
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
  Image
} from '../core/generated'
import { parseColor } from '../core/utils'

// needed for Stylesheet.create() autocompletion
type Styles = {
  [key: string]: FlatStyle
}

type FlatStyle = ViewStyle & TextStyle & ImageStyle

const create = (obj: Styles): Styles => {
  for (const k in obj) {
    compile(obj[k], '' + ++lastId)
  }

  return obj as any
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles) ? Object.assign({}, ...styles.map(flatten)) : styles
}

const cachingFlatten: typeof flatten = styles => {
  const id = []
    .concat(styles)
    .filter(Boolean)
    .map(s => s._id || JSON.stringify(s))
    .join('-')

  let res = CACHE.get(id)

  if (res === undefined) {
    res = flatten(styles)
    compile(res, id)
  }

  return res
}

const compile = (style, id: string) => {
  // could happen if the same anonymous style was passed again but it was evicted from cache few moments ago
  if (style._id !== undefined) {
    CACHE.set(id, style)
    return
  }

  style._surfaceProps = compile2(style)

  Object.defineProperty(style, '_id', {
    // so it does not propagate through Object.assign
    enumerable: false,
    value: id
  })

  CACHE.set(id, Object.freeze(style))
}

const CACHE = new Map<String, any>()
let cleanThreshold = 100
let lastId = 0

// periodically check if cache is not full
// & optionally remove few anonymous styles
const cleanCache = () => {
  if (CACHE.size < cleanThreshold) {
    return
  }

  let removed = 0

  for (const k of CACHE.keys()) {
    // is this anonymous style?
    if (k.indexOf('{') !== -1) {
      CACHE.delete(k)

      // remove at most 10 items
      if (++removed >= 10) {
        break
      }
    }
  }

  // make the cache little bigger
  cleanThreshold += 5
}

setInterval(cleanCache, 5000)

const StyleSheet = {
  compose: (left, right) => (left && right ? [left, right] : left || right),
  // note that react-native does not return numbers anymore,
  flatten: cachingFlatten,
  create
}

export default StyleSheet

function compile2(style: FlatStyle): SurfaceProps {
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
    overflow = 'visible',

    shadowColor,
    //shadowOffset,
    //shadowOpacity,
    shadowRadius = 0,
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
      justifyContent: JustifyContent[JUSTIFY_CONTENT[justifyContent]]
    },
    padding: Rect.mk(
      parseDimension(paddingTop),
      parseDimension(paddingRight),
      parseDimension(paddingBottom),
      parseDimension(paddingLeft)
    ),
    margin: Rect.mk(
      parseDimension(marginTop),
      parseDimension(marginRight),
      parseDimension(marginBottom),
      parseDimension(marginLeft)
    ),
    borderRadius: (borderTopLeftRadius || borderTopRightRadius || borderBottomLeftRadius || borderBottomLeftRadius) ?[
      borderTopLeftRadius, borderTopRightRadius, borderBottomLeftRadius, borderBottomLeftRadius
    ] :undefined,
    boxShadow: shadowColor
      ? {
          blur: shadowRadius,
          spread: 0,
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
