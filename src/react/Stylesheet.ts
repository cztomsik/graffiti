import {
  RNStyleSheet,
  StyleProp,
  ViewStyle,
  TextStyle
} from './react-native-types'
import { ResourceManager } from '../core'

// needed for Stylesheet.create() autocompletion
type Styles = {
  [key: string]: FlatStyle
}

type FlatStyle = ViewStyle | TextStyle

const create = (obj: Styles): Styles => {
  for (const k in obj) {
    compile(obj[k], '' + ++lastId)
  }

  return obj as any
}

const flatten: typeof RNStyleSheet.flatten = styles => {
  styles = styles || undefined

  return Array.isArray(styles) ? Object.assign({}, ...styles) : styles
}

const cachingFlatten: typeof flatten = styles => {
  const id = []
    .concat(styles)
    .filter(Boolean)
    .map(s => s._id || JSON.stringify(s))
    .join('-')

  let res = CACHE.get(id)

  if (res === undefined) {
    res = flatten(styles)
    compile(res, id)
  }

  return res
}

const compile = (style, id: string) => {
  style._brush = ResourceManager.getBrush(style)
  style._layout = ResourceManager.getLayout(style)
  style._clip = ResourceManager.getClip(style)

  CACHE.set((style._id = id), Object.freeze(style))
}

const CACHE = new Map<String, any>()
let cleanThreshold = 100
let lastId = 0

const cleanCache = () => {
  if (CACHE.size < cleanThreshold) {
    return
  }

  let removed = 0

  for (const k of CACHE.keys()) {
    // is this anonymous style?
    if (k.startsWith('{')) {
      CACHE.delete(k)

      // remove at most 10 items
      if (++removed >= 10) {
        break
      }
    }
  }

  // be more generous next time
  cleanThreshold += 5
}

setInterval(cleanCache, 5000)

const StyleSheet = {
  compose: (left, right) => (left && right ? [left, right] : left || right),
  // note that react-native does not return numbers anymore,
  flatten: cachingFlatten,
  create
}

export default StyleSheet
