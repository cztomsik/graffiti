import * as React from 'react'
import View from './View';
import { TouchableWithoutFeedbackProps } from '../react-native-types'

const TouchableWithoutFeedback: React.SFC<TouchableWithoutFeedbackProps> = (props) => {
  // TODO: this is wrong (we should clone first child)
  return <View onClick={props.onPress}>{props.children}</View>
}

export default TouchableWithoutFeedback
