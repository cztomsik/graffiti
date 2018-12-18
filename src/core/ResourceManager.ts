import { parseColor } from './utils'
import { WINDOW_HACK } from './Window'

// TODO: maybe we could somehow find resource duplicates during startup
// there is going to be a lot of similar layouts, brushes, ...

const ResourceManager = {
  getBrush(flatViewStyle) {
    return resolveViewDefaults(flatViewStyle)
  },

  getLayout(flatFlexStyle) {
    return resolveLayoutDefaults(flatFlexStyle)
  },

  // TODO: not sure if buckets should be public at all
  createBucket(item) {
    return WINDOW_HACK.createBucket(JSON.stringify(item))
  },

  updateBucket(bucketId, item) {
    WINDOW_HACK.updateBucket(bucketId, JSON.stringify(item))
  },

  getGlyphIndicesAndAdvances(str) {
    const [indicesBuffer, advancesBuffer] = WINDOW_HACK.getGlyphIndicesAndAdvances(str)

    return [new Uint32Array(indicesBuffer), new Float32Array(advancesBuffer)]
  }
}

const resolveViewDefaults = (style) => {
  const {
    backgroundColor = undefined,

    // borderStyle

    // borderRadius
    // borderBottomLeftRadius
    // borderBottomRightRadius
    // borderTopLeftRadius
    // borderTopRightRadius

    borderColor = [0, 0, 0, 0], // TODO
    borderWidth = 0,

    ...rest
  } = style

  const {
    borderTopWidth = borderWidth,
    borderRightWidth = borderWidth,
    borderBottomWidth = borderWidth,
    borderLeftWidth = borderWidth
  } = rest

  const {
    borderTopColor = borderColor,
    borderRightColor = borderColor,
    borderBottomColor = borderColor,
    borderLeftColor = borderColor
  } = rest

  const res = []

  if (backgroundColor !== undefined) {
    res.push({
      Rectangle: { color: parseColor(backgroundColor) }
    })
  }

  if (borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth) {
    res.push({
      Border: {
        widths: [borderTopWidth, borderRightWidth, borderBottomWidth, borderLeftWidth],
        details: {
          Normal: {
            top: { color: parseColor(borderTopColor), style: 'Solid' },
            right: { color: parseColor(borderRightColor), style: 'Solid' },
            bottom: { color: parseColor(borderBottomColor), style: 'Solid' },
            left: { color: parseColor(borderLeftColor), style: 'Solid' },
            radius: {
              top_left: [0, 0],
              top_right: [0, 0],
              bottom_left: [0, 0],
              bottom_right: [0, 0]
            },
            do_aa: false // !! borderRadius
          }
        }
      }
    })
  }

  return (
    res.length !== 0
      ?res.map(it => ResourceManager.createBucket(it))
      :undefined
  )
}

const resolveLayoutDefaults = (layout) => {
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
    ...rest
  } = layout

  const {
    flexBasis = flex,
    flexGrow = flex,
    flexShrink = flex,

    marginTop = margin,
    marginRight = margin,
    marginBottom = margin,
    marginLeft = margin,

    paddingTop = padding,
    paddingRight = padding,
    paddingBottom = padding,
    paddingLeft = padding
  } = rest

  return {
    width,
    height,
    alignContent: ALIGN.indexOf(alignContent),
    alignItems: ALIGN.indexOf(alignItems),
    alignSelf: ALIGN.indexOf(alignSelf),
    justifyContent: JUSTIFY.indexOf(justifyContent),
    flexDirection: DIRECTION.indexOf(flexDirection),
    flexBasis,
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
    paddingLeft
  }
}

const DIRECTION = ['column', 'column-reverse', 'row', 'row-reverse']
const ALIGN = ['auto', 'flex-start', 'center', 'flex-end', 'strech', 'baseline', 'space-between', 'space-around']
const JUSTIFY = ['flex-start', 'center', 'flex-end', 'space-between', 'space-around', 'space-evenly']
const FLEX_WRAP = ['no-wrap', 'wrap', 'wrap-reverse']

export default ResourceManager
