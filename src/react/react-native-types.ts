// TODO: just use import * as RN from 'react-native' everywhere and move all extensions to index.ts

// typescript should not require('react-native') at all if only types are used
export {
  StyleProp,
  ViewStyle,
  TextStyle,
  FlexStyle,
  ViewProps,
  SwitchProps,
  TextProps,
  ButtonProps,
  TouchableWithoutFeedbackProps,
  ScrollViewProps,
  StyleSheet as RNStyleSheet,
  FlatListProps,
  TextInputProps,
  ImageProps,
  ImageStyle,
  ImageURISource,
  ImageSourcePropType
} from 'react-native'

declare module 'react-native' {
  interface ViewStyle {
    content?: string
    backgroundImageUrl?: string
    shadowSpread?: number
  }

  interface CommonProps {
    // TODO: only els with tabindex should be focusable
    tabindex?: number
    onFocus?: (ev) => void
    onBlur?: (ev) => void
    onKeyDown?: (ev) => void
    onKeyUp?: (ev) => void
    onKeyPress?: (ev) => void
    onClick?: (ev) => void
    onMouseMove?: (ev) => void
    onMouseOver?: (ev) => void
    onMouseOut?: (ev) => void
  }

  interface ViewProps extends CommonProps {}

  type TextValue = string | number | null | undefined | false

  interface TextProps extends CommonProps {
    children?: TextValue | TextValue[]
  }
}
