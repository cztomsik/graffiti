import * as yoga from 'yoga-layout'
import * as React from 'react'

export const Button = ({ title, style = {} }) =>
  <View style={[styles.Button, style]}>
    <Text>{title}</Text>
  </View>

export const Image = ({ style }) =>
  <View style={[styles.Image, style]}>
    <Text>TODO: Image</Text>
  </View>

export const Text = ({ children = [] }) =>
  <node />

export const View = ({ style = {}, children = [] }) =>
  <node layout={resolveLayout(style)} appearance={resolveAppearance(style)}>
    {children}
  </node>

// TODO: theme
const styles = {
  Button: {
    borderWidth: 1,
    //borderRadius: 5
  },

  Image: {
    backgroundColor: '#f00'
  }
}

function resolveLayout({
  flex = 0,
  flexDirection = FLEX_DIRECTIONS[0]
}) {
  return [
    FLEX_DIRECTIONS.indexOf(flexDirection),
    flex
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

  borderColor = null, // TODO
  borderWidth = 0,

  ...rest
}) {
  const {
    borderBottomWidth = borderWidth,
    borderLeftWidth = borderWidth,
    borderRightWidth = borderWidth,
    borderTopWidth = borderWidth
  } = rest

  const {
    borderBottomColor = borderColor,
    borderLeftColor = borderColor,
    borderRightColor = borderColor,
    borderTopColor = borderColor
  } = rest

  return {
    type: 'Rect',
    color: backgroundColor && color(backgroundColor)
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
