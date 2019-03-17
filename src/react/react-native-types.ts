// typescript should not require() this dependency at all if only types are used
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

    borderTopColor?: string
    borderRightColor?: string
    borderBottomColor?: string
    borderLeftColor?: string
  }

  type TextValue = string | number | null | undefined | false

  interface TextProps {
    children?: TextValue | TextValue[]
  }
}
