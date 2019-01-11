import * as React from 'react'
import { ResourceManager } from '../..'
import { __callbacks } from '../../core/Window'
import { View } from '..'
import { TouchableWithoutFeedbackProps } from '../react-native-types'
import { RenderOp } from '../../core/RenderOperation'
import { BridgeBrush } from '../../core/ResourceManager'

// TODO: we should not need extra yoga nodes but
// we do (HasLayout)
class TouchableWithoutFeedback extends React.Component<
  TouchableWithoutFeedbackProps
> {
  callbackId
  brush: BridgeBrush

  static defaultProps = {
    onPress: () => {}
  }

  constructor(props) {
    super(props)

    this.callbackId = __callbacks.push(() => this.props.onPress(null)) - 1

    this.brush = ResourceManager.createBrush([
      RenderOp.HitTest(this.callbackId)
    ])
  }

  render() {
    const { onPress, children, ...rest } = this.props

    // this is super-hacky but it's fine for now
    const props = View(rest).props

    return (
      <host-surface {...props} brush={this.brush}>
        <host-surface brush={props.brush} layout={TOUCHABLE_INSIDE}>
          {children}
        </host-surface>
      </host-surface>
    )
  }

  componentWillUnmount() {
    // TODO: free callback
  }
}

const TOUCHABLE_INSIDE = ResourceManager.getLayout({ flex: 1 })

export default TouchableWithoutFeedback
