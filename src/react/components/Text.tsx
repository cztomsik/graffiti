import * as React from 'react'
import { StyleSheet } from '..'
import { TextProps } from '../react-native-types'
import { parseColor } from '../../core/utils'

const Text = (props: TextProps & { children? }) => {
  const {
    style = {},
    children
  } = props

  const {
    fontSize,
    color = '#000000',
    lineHeight = 30
  } = StyleSheet.flatten(style)

  return (
    <host-text-container fontSize={fontSize} color={parseColor(color)} lineHeight={lineHeight}>
      {children}
    </host-text-container>
  )
}

export default Text
