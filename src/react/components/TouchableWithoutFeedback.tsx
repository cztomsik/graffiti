import * as React from 'react'
import { ResourceManager } from '../..'
import { __callbacks } from '../../core/Window'
import { View } from '..'
import { TouchableWithoutFeedbackProps } from '../react-native-types'

class TouchableWithoutFeedback extends React.Component<TouchableWithoutFeedbackProps> {
  callbackId
  bucketId

  static defaultProps = {
    onPress: () => {}
  }

  constructor(props) {
    super(props)

    this.callbackId = __callbacks.push(() => this.props.onPress(null)) - 1

    this.bucketId = ResourceManager.createBucket({
      HitTest: this.callbackId
    })
  }

  render() {
    const { onPress, ...rest } = this.props

    // this is super-hacky but we need to rethink events anyway so it's fine for now
    const props = View(rest).props
    const brush = [this.bucketId, ...(props.brush || [])]

    return <host-surface {...props} brush={brush} />
  }

  componentWillUnmount() {
    // TODO: free bucket
    // TODO: free callback
  }
}

export default TouchableWithoutFeedback
