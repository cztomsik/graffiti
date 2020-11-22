import { version as VERSION } from '../package.json'
import { ERR, LITTLE_ENDIAN as LE, TODO } from './util'

// TODO: PREBUILT, .dll/so/dylib
const LIB = new URL('../libgraffiti/target/debug/libgraffiti.dylib', import.meta.url).pathname
//const PREBUILT_URL = `https://github.com/cztomsik/graffiti/releases/download/${VERSION}`

// export async fn, nothing should be done at import time (testing)
export const loadNativeApi = async () => {
  if ('Deno' in globalThis) {
    return await loadDenoPlugin()
  }

  if ('process' in globalThis) {
    return await loadNodejsAddon()
  }

  return ERR('unsupported JS engine')
}

export const loadNodejsAddon = async () => {
  // tell dylib to register napi extension
  process.env.GFT_NODEJS = '1'

  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen'](module, LIB)

  return exports as any
}

const loadDenoPlugin = async (Deno = globalThis.Deno) => {
  // TODO: fetch using https://deno.land/x/cache (Plug doesn't really add anything here)

  const rid = Deno.openPlugin(LIB)

  const {
    GFT_TICK,
    GFT_CREATE_WINDOW,
    GFT_TAKE_EVENT,
    GFT_CREATE_TEXT_NODE,
    GFT_SET_TEXT,
    GFT_CREATE_ELEMENT,
    GFT_SET_STYLE,
    GFT_SET_ATTRIBUTE,
    GFT_REMOVE_ATTRIBUTE,
    GFT_INSERT_CHILD,
    GFT_REMOVE_CHILD,
    GFT_FREE_NODE,
  } = Deno.core.ops()

  // string -> utf-8 Uint8Array
  const utf8 = new TextEncoder()

  // const(perf)
  const dispatch = Deno.core.dispatch

  // shared
  const binMsg = new DataView(new ArrayBuffer(4 * 4))

  const decodeEvent = bytes => {
    let bin = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength)

    switch (bin.getUint32(0, LE)) {
      case 0:
        return ['mousemove', bin.getUint32(4, LE), bin.getFloat64(8, LE), bin.getFloat64(16, LE)]
      case 1:
        return ['mousedown', bin.getUint32(4, LE)]
      case 2:
        return ['mouseup', bin.getUint32(4, LE)]
      case 3:
        return ['scroll', bin.getUint32(4, LE), bin.getFloat64(8, LE), bin.getFloat64(16, LE)]
      case 4:
        return ['keydown', bin.getUint32(4, LE)]
      case 5:
        return ['keyup', bin.getUint32(4, LE)]
      case 6:
        return ['keypress', bin.getUint32(4, LE)]
      }

    return ERR('unknown event', bytes)
  }

  return {
    tick: () => dispatch(GFT_TICK),

    createWindow: (title, width, height) => {
      binMsg.setInt32(0, width, LE)
      binMsg.setInt32(4, height, LE)
      return new DataView(dispatch(GFT_CREATE_WINDOW, binMsg, utf8.encode(title)).buffer).getUint32(0, LE)
    },

    takeEvent: (windowId, handler) => {
      Deno.core.setAsyncHandler(GFT_TAKE_EVENT, bytes => handler(decodeEvent(bytes)))

      binMsg.setUint32(0, windowId)
      dispatch(GFT_TAKE_EVENT, binMsg)
    },

    createTextNode: (windowId, text) => {
      binMsg.setUint32(0, windowId)
      return new DataView(dispatch(GFT_CREATE_TEXT_NODE, binMsg, utf8.encode(text)).buffer).getUint32(0, LE)
    },

    setText: (windowId, textNode, text) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, textNode, LE)
      dispatch(GFT_SET_TEXT, binMsg, utf8.encode(text))
    },

    createElement: (windowId, localName) => {
      binMsg.setUint32(0, windowId, LE)
      return new DataView(dispatch(GFT_CREATE_ELEMENT, binMsg, utf8.encode(localName)).buffer).getUint32(0, LE)
    },

    setAttribute: (windowId, el, attName, value) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, el, LE)
      dispatch(GFT_SET_ATTRIBUTE, binMsg, utf8.encode(attName), utf8.encode(value))
    },

    removeAttribute: (windowId, el, attName) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, el, LE)
      dispatch(GFT_REMOVE_ATTRIBUTE, binMsg, utf8.encode(attName))
    },

    setStyle: (windowId, el, prop, value) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, el, LE)
      dispatch(GFT_SET_STYLE, binMsg, utf8.encode(prop), utf8.encode(value))
    },

    insertChild: (windowId, parent, child, index) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, parent, LE)
      binMsg.setUint32(8, child, LE)
      binMsg.setUint32(12, index, LE)
      dispatch(GFT_INSERT_CHILD, binMsg)
    },

    removeChild: (windowId, parent, child) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, parent, LE)
      binMsg.setUint32(8, child, LE)
      dispatch(GFT_REMOVE_CHILD, binMsg)
    },

    freeNode: (windowId, node) => {
      binMsg.setUint32(0, windowId, LE)
      binMsg.setUint32(4, node, LE)
      dispatch(GFT_FREE_NODE, binMsg)
    },
  }
}
