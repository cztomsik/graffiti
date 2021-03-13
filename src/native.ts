import { version as VERSION } from '../package.json'
import { ERR, LITTLE_ENDIAN as LE, TODO, UNSUPPORTED } from './util'

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
  if (!globalThis.fetch) {
    globalThis.fetch = await import('node-fetch')
  }

  // tell dylib to register napi extension
  process.env.GFT_NODEJS = '1'

  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen'](module, LIB)

  return {
    ...exports,

    // could be shared, not sure yet
    async readURL(url) {
      url = new URL(url)

      if (url.protocol === 'data:') {
        return TODO()
      }

      if (url.protocol === 'file:') {
        let fs = await import('fs/promises')
        return fs.readFile(url.pathname, 'utf-8')
      }

      if (url.protocol.match(/^https?:$/)) {
        return fetch(url).then(res => res.text())
      }

      return UNSUPPORTED()
    },
  }
}

const loadDenoPlugin = async (Deno = globalThis.Deno) => {
  // TODO: fetch using https://deno.land/x/cache (Plug doesn't really add anything here)

  const rid = Deno.openPlugin(LIB)

  const encoder = new TextEncoder()
  const decoder = new TextDecoder()

  return Object.fromEntries(
    Object.entries(Deno.core.ops())
      .filter(([k, v]) => k.startsWith('GFT_'))
      .map(([k, v]) => {
        return [
          k.slice(4),
          (...args) => {
            const res = Deno.core.dispatch(v, encoder.encode(JSON.stringify(args)))

            if (res) {
              return JSON.parse(decoder.decode(res))
            }
          },
        ]
      })
  )

  /*

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

  */
}
