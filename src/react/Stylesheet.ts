import {
  RNStyleSheet,
  ViewStyle,
  TextStyle,
  ImageStyle
} from './react-native-types'

// needed for Stylesheet.create() autocompletion
type Styles = {
  [key: string]: FlatStyle
}

type FlatStyle = ViewStyle | (TextStyle & ImageStyle)

const create = (obj: Styles) => {
  for (const k in obj) {
    Object.freeze(obj[k])
  }

  return obj
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles)
    ? Object.assign({}, ...styles.map(flatten))
    : styles
}

// any is needed here because TS is too dumb
const absoluteFillObject: any = {
  position: 'absolute',
  left: 0,
  right: 0,
  top: 0,
  bottom: 0
}

const StyleSheet = {
  // note that react-native does not return numbers anymore,
  flatten,
  create,

  setStyleAttributePreprocessor: () => void 0,
  hairlineWidth: 1,
  absoluteFillObject,
  absoluteFill: absoluteFillObject as any
}

export default StyleSheet
