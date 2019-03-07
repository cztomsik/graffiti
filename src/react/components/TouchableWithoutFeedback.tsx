import * as React from 'react'
import { __callbacks } from '../../core/Window'
import { View } from '..'
import { TouchableWithoutFeedbackProps } from '../react-native-types'

// TODO: we should not need extra yoga nodes but
// we do (HasLayout)
class TouchableWithoutFeedback extends React.Component<
  TouchableWithoutFeedbackProps
> {
  callbackId

  static defaultProps = {
    onPress: () => {}
  }

  constructor(props) {
    super(props)

    this.callbackId = __callbacks.push(() => this.props.onPress(null)) - 1

    //this.brush = ResourceManager.createBrush([
    //  RenderOp.HitTest(this.callbackId)
    //])
  }

  render() {
    const { onPress, children, ...rest } = this.props

    // this is super-hacky but it's fine for now
    const props = View(rest).props

    // TODO
    return (
      <host-surface>
        {children}
      </host-surface>
    )
  }

  componentWillUnmount() {
    // TODO: free callback
  }
}

export default TouchableWithoutFeedback
