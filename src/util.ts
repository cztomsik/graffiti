export const isDeno = 'Deno' in globalThis
export const isNodeJS = 'process' in globalThis

export const assert = (value, msg) => value || ERR(msg)

export const NOOP = () => {}
export const ERR = (...msgs): any => {
  throw new Error(msgs.join(' '))
}
export const TODO = () => ERR('TODO')
export const UNSUPPORTED = () => ERR('UNSUPPORTED')

export const last = arr => arr[arr.length - 1]

export const LITTLE_ENDIAN = (() => {
  let b = new ArrayBuffer(2)
  new DataView(b).setInt16(0, 256, true)
  return new Int16Array(b)[0] === 256
})()

export const PLATFORM = globalThis.Deno?.build.os ?? (await import('os')).platform().replace(/win32/, 'windows')

export const Worker =
  globalThis.Worker ??
  class Worker extends (await import('worker_threads')).Worker {
    addEventListener(ev, listener) {
      this.on(ev, data => listener({ data }))
    }
  }

// @ts-expect-error
export const fetch = globalThis.fetch ?? (await import('node-fetch')).default

export const readTextFile =
  globalThis.Deno?.readTextFile ??
  (async path => {
    const fs = await import('fs/promises')
    return fs.readFile(path, 'utf-8')
  })

export async function readURL(url) {
  url = new URL(url)

  if (url.protocol === 'data:') {
    return TODO()
  }

  if (url.protocol === 'file:') {
    let path: string
    if (PLATFORM === 'windows') path = url.href.replace('file:///', '') // ~~Windows dirty fix~~ Maybe better
    else path = url.pathname
    return readTextFile(path)
  }

  if (url.protocol.match(/^https?:$/)) {
    return fetch(url).then(res => res.text())
  }

  return UNSUPPORTED()
}
