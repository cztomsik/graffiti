import * as React from 'react'
import { TouchableWithoutFeedbackProps } from 'react-native'
import { View } from '..';

export const TouchableWithoutFeedback: React.SFC<TouchableWithoutFeedbackProps> = (props) => {
  // TODO: this is wrong (we should clone first child)
  return <View onClick={props.onPress}>{props.children}</View>
}
