import {
  RNStyleSheet,
  StyleProp,
  ViewStyle,
  TextStyle,
  ImageStyle
} from './react-native-types'
import { HostSurfaceProps } from './reconciler'
import {
  mkRect,
  mkDimensionAuto,
  Dimension,
  mkDimensionPoint,
  mkDimensionPercent,
  mkSize,
  FlexDirection,
  JustifyContent,
  FlexAlign,
  FlexWrap
} from '../astToTs/generated/example'
import { parseColor } from '../core/utils'

// needed for Stylesheet.create() autocompletion
type Styles = {
  [key: string]: FlatStyle
}

type FlatStyle = ViewStyle | TextStyle | ImageStyle

interface BorderProps {
  borderTopColor?: string
  borderRightColor?: string
  borderBottomColor?: string
  borderLeftColor?: string
}

const create = (obj: Styles): Styles => {
  for (const k in obj) {
    compile(obj[k], '' + ++lastId)
  }

  return obj as any
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles) ? Object.assign({}, ...styles) : styles
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

  style._props = compile2(style)

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

function compile2(style: FlatStyle & BorderProps): HostSurfaceProps {
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

    backgroundColor,
    // TODO: BorderStyle
    borderRadius = 0,
    borderColor,
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

    borderBottomLeftRadius = borderRadius,
    borderBottomRightRadius = borderRadius,
    borderTopLeftRadius = borderRadius,
    borderTopRightRadius = borderRadius,

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
    //borderRadius,
    size: mkSize(parseDimension(width), parseDimension(height)),
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
    padding: mkRect(
      parseDimension(paddingTop),
      parseDimension(paddingBottom),
      parseDimension(paddingRight),
      parseDimension(paddingLeft)
    ),
    margin: mkRect(
      parseDimension(marginTop),
      parseDimension(marginBottom),
      parseDimension(marginRight),
      parseDimension(marginLeft)
    ),
    //boxShadow,
    backgroundColor:
      (backgroundColor || undefined) && parseColor(backgroundColor)
    //image,
    //text,
    //border
  }
}

function parseDimension(value?: string | number): Dimension {
  value = '' + value

  if (value.endsWith('%')) {
    return mkDimensionPercent(parseFloat(value))
  }

  if (value === 'auto' || value === undefined) {
    return mkDimensionAuto()
  }

  return mkDimensionPoint(parseFloat(value))
}

const FLEX_DIRECTION = {
  column: 'Column',
  'column-reverse': 'ColumnReverse',
  row: 'Row',
  'row-reverse': 'RowReverse'
}

const FLEX_WRAP = {
  'nowrap': 'NoWrap',
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
