import * as yoga from 'yoga-layout'
import * as React from 'react'

import { ViewProps, TextProps } from '@types/react-native'

// TODO: style
const Button = ({ title, onPress = undefined }) =>
  <View style={{ padding: 10, backgroundColor: '#2196F3' }} onPress={onPress}>
    <Text style={{ color: '#ffffff', textAlign: 'center' }}>{title.toUpperCase()}</Text>
  </View>

// TODO: style (extends View.style)
const Text = ({ style = {}, children = undefined }: TextProps & { children? }) => {
  const {
    color = '#000000',
    lineHeight = 30
  } = (style as any)

  return <wr-text color={parseColor(color)} lineHeight={lineHeight}>{children}</wr-text>
}

const View = ({ style = {}, children = undefined, onPress = undefined }: ViewProps) =>
  <wr-view layout={resolveLayout(style)} {...resolveAppearance(style)} onPress={onPress}>
    {children}
  </wr-view>

// otherwise it would be Unknown in devtools
export { Button, Text, View }

function resolveLayout({
  width = 'auto',
  height = 'auto',
  flex = 0,
  flexDirection = FLEX_DIRECTIONS[0],
  padding = 0,
  margin = 0,
  alignContent = 'flex-start',
  alignItems = 'strech',
  alignSelf = 'auto',
  justifyContent = 'flex-start',
  flexWrap = 'no-wrap',
  ...rest
}) {
  const {
    marginTop = margin,
    marginRight = margin,
    marginBottom = margin,
    marginLeft = margin,

    paddingTop = padding,
    paddingRight = padding,
    paddingBottom = padding,
    paddingLeft = padding
  } = rest

  return [
    width,
    height,
    FLEX_DIRECTIONS.indexOf(flexDirection),
    flex,

    marginTop, marginRight, marginBottom, marginLeft,

    paddingTop, paddingRight, paddingBottom, paddingLeft,

    ALIGN.indexOf(alignContent), ALIGN.indexOf(alignItems), ALIGN.indexOf(alignSelf),

    JUSTIFY.indexOf(justifyContent),

    FLEX_WRAP.indexOf(flexWrap)
  ]
}

function resolveAppearance({
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
}) {
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

  return {
    // TODO
    /*stackingContext: ((opacity !== 1.0) || undefined) && {
      PushStackingContext: {
        stacking_context: {
          transform_style: 'Flat',
          mix_blend_mode: 'Normal',
          //clip_node_id: None,
          raster_space: 'Screen'
        }
      }
    },*/
    background: backgroundColor && { Rectangle: { color: parseColor(backgroundColor) } },
    border: (borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth || undefined) && {
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
    }
  }
}

// TODO
function parseColor(str: string) {
  return [
    hex(str.slice(1, 3)) / 255,
    hex(str.slice(3, 5)) / 255,
    hex(str.slice(5, 7)) / 255,
    1
  ]
}

function hex(str) {
  return parseInt(str, 16)
}

const FLEX_DIRECTIONS = ['column', 'column-reverse', 'row', 'row-reverse']
const ALIGN = ['auto', 'flex-start', 'center', 'flex-end', 'strech', 'baseline', 'space-between', 'space-around']
const JUSTIFY = ['flex-start', 'center', 'flex-end', 'space-between', 'space-around', 'space-evenly']
const FLEX_WRAP = ['no-wrap', 'wrap', 'wrap-reverse']
