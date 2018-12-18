import * as React from 'react'
import { TouchableWithoutFeedbackProps } from '../react-native-types'

const TouchableWithoutFeedback = (props: TouchableWithoutFeedbackProps) =>
  (props as any).children
  //<hosted-hit-test {...props} />

export default TouchableWithoutFeedback
