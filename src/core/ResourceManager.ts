import { parseColor } from './utils'
import { WINDOW_HACK } from './Window'
import { RenderOperation, RenderOp, BorderStyle } from './RenderOperation'

// x, y, width, height
export type BridgeRect = [number, number, number, number]

// that basically means that you cannot create a bucket outside of ResourceManager.createBucket
export type BucketId = number & { 'opaque type': 'of bucket' }

const native = require('../../native')

// TODO: maybe we could somehow find resource duplicates during startup
// there is going to be a lot of similar layouts, brushes, ...

const ResourceManager = {
  getBrush(flatViewStyle) {
    return resolveViewDefaults(flatViewStyle)
  },

  getLayout(flatFlexStyle) {
    return resolveLayoutDefaults(flatFlexStyle)
  },

  getClip(flatClipStyle) {
    return resolveClipDefaults(flatClipStyle)
  },

  // TODO: not sure if buckets should be public at all
  createBucket(item: RenderOperation): BucketId {
    return native.createBucket(JSON.stringify(item))
  },

  updateBucket(bucketId: BucketId, item) {
    native.updateBucket(bucketId, JSON.stringify(item))
  },

  getGlyphIndicesAndAdvances(fontSize, str) {
    const [
      indicesBuffer,
      advancesBuffer
    ] = WINDOW_HACK.getGlyphIndicesAndAdvances(fontSize, str)

    return [new Uint32Array(indicesBuffer), new Float32Array(advancesBuffer)]
  }
}

const resolveViewDefaults = (style): BucketId[] | undefined => {
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
            do_aa: true //!! (borderTopLeftRadius || borderBottomLeftRadius || borderBottomRightRadius || borderTopRightRadius)
          }
        }
      })
    )
  }

  return res.length !== 0
    ? res.map(it => ResourceManager.createBucket(it))
    : undefined
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
    alignItems = 'strech',
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

  return {
    width,
    height,
    alignContent: ALIGN.indexOf(alignContent),
    alignItems: ALIGN.indexOf(alignItems),
    alignSelf: ALIGN.indexOf(alignSelf),
    justifyContent: JUSTIFY.indexOf(justifyContent),
    flexDirection: DIRECTION.indexOf(flexDirection),
    flexBasis: flexBasis === 'auto' ? NaN : flexBasis,
    flexGrow,
    flexShrink,
    flexWrap: FLEX_WRAP.indexOf(flexWrap),
    marginTop,
    marginRight,
    marginBottom,
    marginLeft,
    paddingTop,
    paddingRight,
    paddingBottom,
    paddingLeft,
    overflow: OVERFLOW.indexOf(overflow)
  }
}

const resolveClipDefaults = (style): BucketId[] => {
  const { borderRadius = 0 } = style

  return borderRadius !== 0
    ? [
        ResourceManager.createBucket(
          RenderOp.PushBorderRadiusClip(borderRadius)
        )
      ]
    : undefined
}

const DIRECTION = ['column', 'column-reverse', 'row', 'row-reverse']
const ALIGN = [
  'auto',
  'flex-start',
  'center',
  'flex-end',
  'strech',
  'baseline',
  'space-between',
  'space-around'
]
const JUSTIFY = [
  'flex-start',
  'center',
  'flex-end',
  'space-between',
  'space-around',
  'space-evenly'
]
const FLEX_WRAP = ['no-wrap', 'wrap', 'wrap-reverse']
const OVERFLOW = ['hidden', 'scroll', 'visible']

export default ResourceManager
