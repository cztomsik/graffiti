import * as yoga from 'yoga-layout'
import * as React from 'react'

// it probably should be inside View, because Text.style extends View.style
// and it can help with stacking too
export const Text = ({ children = [] }) =>
  <React.Fragment>{children}</React.Fragment>

export const View = ({ style, children }) =>
  <view layout={resolveLayout(style)} {...resolveAppearance(style)}>
    {children}
  </view>

function resolveLayout({
  width = 'auto',
  height = 'auto',
  flex = 0,
  flexDirection = FLEX_DIRECTIONS[0],
  padding = 0,
  margin = 0,
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

    paddingTop, paddingRight, paddingBottom, paddingLeft
  ]
}

function resolveAppearance({
  backgroundColor,

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
    background: backgroundColor && { Rectangle: { color: color(backgroundColor) } },
    border: (borderTopWidth || borderRightWidth || borderBottomWidth || borderLeftWidth || undefined) && {
      Border: {
        widths: [borderTopWidth, borderRightWidth, borderBottomWidth, borderLeftWidth],
        details: {
          Normal: {
            top: { color: color(borderTopColor), style: 'Solid' },
            right: { color: color(borderRightColor), style: 'Solid' },
            bottom: { color: color(borderBottomColor), style: 'Solid' },
            left: { color: color(borderLeftColor), style: 'Solid' },
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
function color(str: string) {
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
