// x export fn, nothing should be done at import time (testing)
// x flat-api (rust api is flat too)
// x async (deno plugin download)

import { ERR, TODO } from './util'

// TODO: get from package.json?
const VERSION = '1.0.0-alpha.1'

export const loadNativeApi = async () => {
  if ('Deno' in globalThis) {
    return await loadDenoPlugin()
  }

  return ERR('unsupported JS engine')
}

const loadDenoPlugin = async () => {
  const PLUG_URL = 'https://deno.land/x/plug@0.0.5/mod.ts'
  const { Plug } = await import(PLUG_URL)

  const BUILD_DIR = new URL('../libgraffiti/target/debug', import.meta.url).pathname
  const PREBUILT_URL = `https://github.com/cztomsik/graffiti/releases/download/${VERSION}`

  let [path, cache] = globalThis.Deno.statSync(BUILD_DIR).isDirectory ? [BUILD_DIR, false] : [PREBUILT_URL, true]

  const rid = await Plug.prepare({
    name: 'graffiti',
    urls: {
      darwin: `${path}/libgraffiti.dylib`,
      windows: `${path}/graffiti.dll`,
      linux: `${path}/libgraffiti.so`,
    },
    policy: Plug.CachePolicy[cache ? 'STORE' : 'NONE'],
  })

  // TODO: assert
  // TODO: generate from shared definition? (GFT_${k.toUpperCase()})
  const { GFT_CREATE_WINDOW, GFT_CREATE_NODE, GFT_FREE_NODE, GFT_UPDATE_DOCUMENT, GFT_TICK } = Plug.core.ops()

  const encoder = new TextEncoder()
  const decoder = new TextDecoder()

  // TODO: bin + (de)ser
  const send = (op, msg) => JSON.parse(decoder.decode(Plug.core.dispatch(op, encoder.encode(JSON.stringify(msg)))))

  return {
    tick: () => send(GFT_TICK, null),
    createWindow: (title, width, height) => send(GFT_CREATE_WINDOW, [title, width, height]),
    createNode: (windowId) => send(GFT_CREATE_NODE, [windowId]),
    freeNode: (windowId) => send(GFT_FREE_NODE, [windowId]),
    updateDocument: (windowId, changes) => send(GFT_UPDATE_DOCUMENT, [windowId, changes]),
  }
}

// TODO: could download too (but it's unusual for nodejs modules to download something at runtime)
export const loadNodejsAddon = async () => {
  TODO()

  /*
  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen'](module, `${__dirname}/../libgraffiti/target/libgraffiti.node`)

  return exports as any
  */
}
