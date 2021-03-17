import { version as VERSION } from '../package.json'
import { ERR, TODO, UNSUPPORTED } from './util'

export let native: any = new Proxy(
  {},
  {
    get() {
      throw new Error('not loaded, init first')
    },
  }
)

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
    // @ts-expect-error
    const { default: fetch } = await import('node-fetch')
    globalThis.fetch = fetch
  }

  // tell dylib to register napi extension
  process.env.GFT_NODEJS = '1'

  // require() would make ncc bundle some unnecessary build artifacts
  process['dlopen'](module, LIB)

  native = {
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

  native = Object.fromEntries(
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
}
