import * as React from 'react'
import { StyleSheet } from '..'
import { ViewProps } from '../react-native-types'
import { ResourceManager } from '../..';

const View = (props: ViewProps) => {
  let {
    style = {}
  } = props

  style = StyleSheet.flatten(style)

  const brush = ResourceManager.getBrush(style)
  const layout = ResourceManager.getLayout(style)
  const clip = ResourceManager.getClip(style)

  return (
    <host-surface {...({ brush, layout, clip })}>
      {(props as any).children}
    </host-surface>
  )
}

export default View
