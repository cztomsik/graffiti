import { StyleProp, ViewStyle, TextStyle } from './react-native-types'

// should auto-complete rules inside Stylesheet.create()
type Styles = {
  [key: string]: StyleProp<ViewStyle | TextStyle>
}

const StyleSheet = {
  // note that react-native does not return numbers anymore
  create(obj: Styles): Styles {
    for (const k in obj) {
      Object.freeze(obj[k])
    }

    return obj
  },

  compose: (left, right) =>
    (left && right)
      ?[left, right]
      :left || right,

  flatten(styles) {
    styles = styles || undefined

    return (
      Array.isArray(styles)
        ?Object.assign({}, ...styles)
        :styles
    )
  }
}

export default StyleSheet
