export const isDeno = 'Deno' in globalThis
export const isNodeJS = 'process' in globalThis

export const PLATFORM = isDeno
  ? globalThis.Deno?.build.os
  : isNodeJS
  ? (await import('os')).platform().replace(/win32/, 'windows')
  : 'unknown'

export const assert = (value, msg) => value || ERR(msg)

export const NOOP = () => {}
export const ERR = (msg: string): any => {
  throw new Error(msg)
}
export const TODO = () => ERR('TODO')
export const UNSUPPORTED = () => ERR('UNSUPPORTED')

// TODO: find a better way, maybe we will have to drop \0 strings
const encoder = new TextEncoder()
export const encode = (input: string) => new Uint8Array([...encoder.encode(input), 0])

export const last = arr => arr[arr.length - 1]

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
