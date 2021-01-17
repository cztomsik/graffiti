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
    GFT_INIT,
    GFT_NEXT_EVENT,
    GFT_CREATE_WINDOW,
    GFT_CREATE_VIEWPORT,
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
  const dispatch = (...args) => {
    const res = Deno.core.dispatch(...args)

    if (res) {
      return new DataView(res.buffer)
    }
  }

  // TODO: avoid allocs, one DataView can point to one, shared ArrayBuffer
  const i32 = v => new Int32Array([v])
  const u32 = v => new Uint32Array([v])

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

  dispatch(GFT_INIT)

  return {
    waitEvents: () => {
      return dispatch(GFT_NEXT_EVENT)
    },

    createWindow: (title, width, height) => {
      return dispatch(GFT_CREATE_WINDOW, utf8.encode(title), i32(width), i32(height))!.getUint32(0, LE)
    },

    nextEvent: win => {
      //handler(decodeEvent(dispatch(GFT_NEXT_EVENT, u32(win))))
    },

    createViewport: win => {
      return dispatch(GFT_CREATE_VIEWPORT, u32(win))!.getUint32(0, LE)
    },

    createTextNode: (win, text) => {
      return dispatch(GFT_CREATE_TEXT_NODE, u32(win), utf8.encode(text))!.getUint32(0, LE)
    },

    setText: (win, node, text) => {
      return dispatch(GFT_SET_TEXT, u32(win), u32(node), utf8.encode(text))
    },

    createElement: (win, localName) => {
      return dispatch(GFT_CREATE_ELEMENT, u32(win), utf8.encode(localName))!.getUint32(0, LE)
    },

    setAttribute: (win, el, attName, value) => {
      return dispatch(GFT_SET_ATTRIBUTE, u32(win), u32(el), utf8.encode(attName), utf8.encode(value))
    },

    removeAttribute: (win, el, attName) => {
      return dispatch(GFT_REMOVE_ATTRIBUTE, u32(win), utf8.encode(attName))
    },

    setStyle: (win, el, prop, value) => {
      return console.log('TODO: setStyle')
    },

    insertChild: (win, parent, child, index) => {
      return dispatch(GFT_INSERT_CHILD, u32(win), u32(parent), u32(child), u32(index))
    },

    removeChild: (win, parent, child) => {
      return dispatch(GFT_REMOVE_CHILD, u32(win), u32(parent), u32(child))
    },

    freeNode: (win, node) => {
      return dispatch(GFT_FREE_NODE, u32(win), u32(node))
    },
  }
}
