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
    backgroundImageUrl?: string
  }

  interface ViewProps {
    onClick?: (ev) => void
    onMouseMove?: (ev) => void
  }

  type TextValue = string | number | null | undefined | false

  interface TextProps {
    children?: TextValue | TextValue[]
  }
}
