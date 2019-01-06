import * as React from 'react'
import { StyleSheet } from '..'
import { ViewProps } from '../react-native-types'
import { ResourceManager } from '../..'

const View = (props: ViewProps) => {
  let { style = {} } = props

  const { _brush: brush, _layout: layout, _clip: clip } = StyleSheet.flatten(
    style
  ) as any

  return (
    <host-surface {...{ brush, layout, clip }}>
      {(props as any).children}
    </host-surface>
  )
}

export default View
