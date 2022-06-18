import { ERR, isDeno, isNodeJS, PLATFORM, TODO } from './util'

const encoder = new TextEncoder()

export const send: (msg: any) => any = await loadNativeApi()

async function loadNativeApi() {
  const libFile = await resolveLibFile()

  switch (true) {
    case isNodeJS:
      return await loadNodejsAddon(libFile)
    case isDeno:
      return await loadDenoPlugin(libFile)
    default:
      return ERR('unsupported JS engine')
  }
}

async function resolveLibFile() {
  // TODO
  // const PREBUILT_URL = `https://github.com/cztomsik/graffiti/releases/download/${VERSION}`

  const [prefix, suffix] = {
    windows: ['', '.dll'],
    darwin: ['lib', '.dylib'],
    linux: ['lib', '.so'],
  }[PLATFORM]

  const url = new URL(`../libgraffiti/target/debug/${prefix}graffiti${suffix}`, import.meta.url)

  return PLATFORM === 'windows' ? url.href.replace('file:///', '') : url.pathname
}

async function loadNodejsAddon(libFile) {
  return TODO()
}

async function loadDenoPlugin(libFile, Deno = globalThis.Deno) {
  // TODO: fetch using https://deno.land/x/cache (Plug doesn't really add anything here)

  const { gft_send } = Deno.dlopen(libFile, {
    gft_send: { parameters: ['pointer', 'usize'], result: 'pointer' },
  }).symbols

  return msg => {
    // TODO: bincode, reuse buffer
    const buf = encoder.encode(JSON.stringify(msg))
    const res = gft_send(buf, buf.length)
    return res.value ? JSON.parse(new Deno.UnsafePointerView(res).getCString()) : null
  }
}
