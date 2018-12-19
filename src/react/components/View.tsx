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

  return (
    <host-surface {...({ brush, layout })}>
      {(props as any).children}
    </host-surface>
  )
}

export default View
