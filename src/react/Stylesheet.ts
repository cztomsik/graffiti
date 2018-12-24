import { RNStyleSheet } from './react-native-types'

// should auto-complete rules inside Stylesheet.create()
// type Styles = {
//   [key: string]: StyleProp<ViewStyle | TextStyle>;
// };

const create: typeof RNStyleSheet.create = obj => {
  for (const k in obj) {
    Object.freeze(obj[k])
  }

  return obj as any
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles) ? Object.assign({}, ...styles) : styles
}

const StyleSheet = {
  compose: (left, right) => (left && right ? [left, right] : left || right),
  // note that react-native does not return numbers anymore,
  flatten,
  create
}

export default StyleSheet
