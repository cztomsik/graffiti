import * as React from 'react'
import { StyleSheet, View } from '..'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'

const Text = (props: TextProps & { children? }) => {
  const { style = {}, children } = props

  const flatStyle = StyleSheet.flatten(style)

  // TODO: move defaults to TextContainer
  // TODO: what if color was resource with its own bucket/renderOperation? it could in theory improve CPU-caching
  const { fontSize, color = '#000000', lineHeight = 30 } = flatStyle

  return (
    <View style={flatStyle}>
      <host-text-container
        fontSize={fontSize}
        color={parseColor(color)}
        lineHeight={lineHeight}
      >
        {children}
      </host-text-container>
    </View>
  )
}

export default Text
