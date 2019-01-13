import { parseColor } from './utils'
import { WINDOW_HACK } from './Window'
import { RenderOperation, RenderOp, BorderStyle } from './RenderOperation'

// x, y, width, height
export type BridgeRect = [number, number, number, number]

// cannot create outside of ResourceManager.create*
export type BridgeBrush = [number, number] & { 'opaque type': 'of brush' }
export type BridgeClip = [number, number] & { 'opaque type': 'of clip' }

const native = require('../../native')

// TODO: maybe we could somehow find resource duplicates during startup
// there is going to be a lot of similar layouts, brushes, ...

const ResourceManager = {
  createBrush(ops: RenderOperation[]): BridgeBrush {
    return createOpResource(ops) as BridgeBrush
  },

  createClip(ops: RenderOperation[]): BridgeClip {
    return createOpResource(ops) as BridgeClip
  },

  getBrush(flatViewStyle) {
    return resolveViewDefaults(flatViewStyle)
  },

  getLayout(flatFlexStyle) {
    return new native.FlexLayout(JSON.stringify(resolveLayoutDefaults(flatFlexStyle)))
  },

  getClip(flatClipStyle) {
    return resolveClipDefaults(flatClipStyle)
  },

  getGlyphIndicesAndAdvances(fontSize, str) {
    const [
      indicesBuffer,
      advancesBuffer
    ] = WINDOW_HACK.getGlyphIndicesAndAdvances(fontSize, str)

    return [new Uint32Array(indicesBuffer), new Float32Array(advancesBuffer)]
  }
}

const resolveViewDefaults = (style): BridgeBrush | undefined => {
  const {
    backgroundColor = undefined,

    // borderStyle

    // TODO: clip
    borderRadius = 0,

    borderColor,
    borderWidth = 0,

    ...rest
  } = style

  const {
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
    borderTopRightRadius = borderRadius
  } = rest

  const res: RenderOperation[] = []

  if (backgroundColor !== undefined) {
    res.push(RenderOp.Rectangle(parseColor(backgroundColor)))
  }

  if (
    borderColor &&
    (borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth)
  ) {
    res.push(
      RenderOp.Border({
        widths: [
          borderTopWidth,
          borderRightWidth,
          borderBottomWidth,
          borderLeftWidth
        ],

        details: {
          Normal: {
            top: {
              color: parseColor(borderTopColor),
              style: BorderStyle.Solid
            },
            right: {
              color: parseColor(borderRightColor),
              style: BorderStyle.Solid
            },
            bottom: {
              color: parseColor(borderBottomColor),
              style: BorderStyle.Solid
            },
            left: {
              color: parseColor(borderLeftColor),
              style: BorderStyle.Solid
            },
            radius: {
              top_left: [borderTopLeftRadius, borderTopLeftRadius],
              top_right: [borderTopRightRadius, borderTopRightRadius],
              bottom_left: [borderBottomLeftRadius, borderBottomLeftRadius],
              bottom_right: [borderBottomRightRadius, borderBottomRightRadius]
            },
            do_aa: !!(
              borderTopLeftRadius ||
              borderBottomLeftRadius ||
              borderBottomRightRadius ||
              borderTopRightRadius
            )
          }
        }
      })
    )
  }

  return res.length !== 0 ? ResourceManager.createBrush(res) : undefined
}

const resolveLayoutDefaults = layout => {
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
    ...rest
  } = layout

  const {
    marginHorizontal = margin,
    marginVertical = margin,
    paddingHorizontal = padding,
    paddingVertical = padding,
    ...rest2
  } = rest

  const {
    flexGrow = flex,
    flexShrink = flex,
    flexBasis = 'auto',

    marginTop = marginVertical,
    marginBottom = marginVertical,
    marginLeft = marginHorizontal,
    marginRight = marginHorizontal,

    paddingTop = paddingVertical,
    paddingBottom = paddingVertical,
    paddingLeft = paddingHorizontal,
    paddingRight = paddingHorizontal
  } = rest2

  return [
    { Width: StyleUnit(width) },
    { Height: StyleUnit(height) },

    { FlexDirection: { [DIRECTION[flexDirection]]: null } },
    { FlexBasis: StyleUnit(flexBasis) },
    { FlexGrow: flexGrow },
    { FlexShrink: flexShrink },
    { FlexWrap: { [FLEX_WRAP[flexWrap]]: null } },

    { AlignContent: { [ALIGN[alignContent]]: null } },
    { AlignItems: { [ALIGN[alignItems]]: null } },
    { AlignSelf: { [ALIGN[alignSelf]]: null } },

    { JustifyContent: { [JUSTIFY[justifyContent]]: null } },

    { MarginTop: StyleUnit(marginTop) },
    { MarginRight: StyleUnit(marginRight) },
    { MarginBottom: StyleUnit(marginBottom) },
    { MarginLeft: StyleUnit(marginLeft) },
    { PaddingTop: StyleUnit(paddingTop) },
    { PaddingRight: StyleUnit(paddingRight) },
    { PaddingBottom: StyleUnit(paddingBottom) },
    { PaddingLeft: StyleUnit(paddingLeft) },
    { Overflow: { [OVERFLOW[overflow]]: null } }
  ]
}

const StyleUnit = (value) => {
  if (value === 'auto') {
    return { Auto: null }
  }

  if (typeof value === 'number') {
    return { Point: value }
  }

  if (('' + value).endsWith('%')) {
    return { Percent: parseFloat(value) }
  }

  if ( ! value) {
    return { UndefinedValue: null }
  }
}

const resolveClipDefaults = (style): BridgeClip | undefined => {
  const { borderRadius = 0 } = style

  return borderRadius !== 0
    ? ResourceManager.createClip([RenderOp.PushBorderRadiusClip(borderRadius)])
    : undefined
}

const createOpResource = (ops: RenderOperation[]) => {
  return new native.OpResource(JSON.stringify(ops))
}

const DIRECTION = {
  'column': 'Column',
  'column-reverse': 'ColumnReverse',
  'row': 'Row',
  'row-reverse': 'RowReverse'
}

const ALIGN = {
  auto: 'Auto',
  'flex-start': 'FlexStart',
  center: 'Center',
  'flex-end': 'FlexEnd',
  stretch: 'Stretch',
  baseline: 'Baseline',
  'space-between': 'SpaceBetween',
  'space-around': 'SpaceAround'
}

const JUSTIFY = {
  'flex-start': 'FlexStart',
  'center': 'Center',
  'flex-end': 'FlexEnd',
  'space-between': 'SpaceBetween',
  'space-around': 'SpaceAround',
  'space-evenly': 'SpaceEvenly'
}

const FLEX_WRAP = {
  'no-wrap': 'NoWrap',
  'wrap': 'Wrap',
  'wrap-reverse': 'WrapReverse'
}

const OVERFLOW = {
  hidden: 'Hidden',
  scroll: 'Scroll',
  visible: 'Visible'
}

export default ResourceManager
