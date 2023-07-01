import { createRequire } from 'node:module'
import process from 'node:process'
const require = createRequire(import.meta.url)

// TODO: arch

const targets = {
  darwin: 'macos',
  linux: 'linux',
  win32: 'windows',
}

export const native = require(`../../zig-out/lib/graffiti.${targets[process.platform]}.node`)

/** @type {<T extends new (...args: any[]) => any>(Clz: T, obj: any) => InstanceType<T>} */
export const wrap = (Clz, obj, ...extra) => (Object.setPrototypeOf(obj, Clz.prototype), obj)
