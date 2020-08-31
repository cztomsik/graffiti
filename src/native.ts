// x export fn, nothing should be done at import time (testing)
// x flat-api (rust api is flat too)
// x async (deno plugin download)

import { ERR } from './util'

export const loadNativeApi = async () => {
  if ('Deno' in globalThis) {
    return await loadDenoPlugin()
  }

  return ERR('unsupported JS engine')
}

const loadDenoPlugin = async () => {
  const { Plug } = await import('https://deno.land/x/plug@0.0.5/mod.ts')

  // TODO: env/autodetect deno.statSync("../libgraffiti/?").isFile/Directory()
  const path = new URL(`../libgraffiti/target/debug`, import.meta.url).pathname
  //const path = 'https://github.com/cztomsik/graffiti/releases/download/1.0.0'

  const rid = await Plug.prepare({
    name: 'graffiti',
    urls: {
      darwin: `${path}/libgraffiti.dylib`,
      windows: `${path}/graffiti.dll`,
      linux: `${path}/libgraffiti.so`,
    }
  })

  // TODO: assert
  // TODO: generate from shared definition? (GFT_${k.toUpperCase()})
  const { GFT_CREATE_WINDOW, GFT_TICK } = Plug.core.ops();

  const encoder = new TextEncoder()
  const decoder = new TextDecoder()

  // TODO: bin + (de)ser
  const send = (op, msg) => JSON.parse(decoder.decode(Plug.core.dispatch(op, encoder.encode(JSON.stringify(msg)))))

  return {
    tick: () => send(GFT_TICK, null),
    createWindow: (title, width, height) => send(GFT_CREATE_WINDOW, [title, width, height])
  }
}

// TODO: (async) loadNodejsAddon()
// could download too (but it's unusual for nodejs modules to download something at runtime)
