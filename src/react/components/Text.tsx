import * as React from 'react'
import { StyleSheet, View } from '..'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'

const Text: React.SFC<TextProps> = ({ style = {}, children }) => {
  const flatStyle = StyleSheet.flatten(style)

  const { fontSize, color = '#000000', lineHeight = 30 } = flatStyle

  return (
    <View style={flatStyle}>
      <host-surface
        size={[{ tag: 'Point', value: 20 }, { tag: 'Point', value: 20 }]}
        text={{ fontSize, color: parseColor(color), lineHeight, text: [].concat(children).join('') }}
      />
    </View>
  )
}

export default Text
