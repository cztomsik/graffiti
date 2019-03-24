import * as React from 'react'
import StyleSheet from '../Stylesheet';
import { ViewProps } from '../react-native-types'

const View: React.SFC<ViewProps> = (props) => {
  const { _props } = StyleSheet.flatten(
    props.style || {}
  ) as any

  const listeners = {
    onMouseMove: () => {},
    onMouseDown: () => {},
    onMouseUp: () => {},
    onClick: props.onClick
  }

  return <host-surface {..._props} listeners={listeners}>{props.children}</host-surface>
}

export default View
