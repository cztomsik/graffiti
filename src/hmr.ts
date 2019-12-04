// not yet correct, m.require should be overridden too so
// that we can record who depends on what
// and use that for invalidation

// also, this is intentionally different (and simpler) from
// original webpack HMR api

import * as fs from 'fs'
const { Module } = require('module')

const watchers = new Map()
let cbs = []

if (process.env.HOT === '1') {
  console.log('enabling HMR')

  // has to be patched, mapVaues is not enough
  for (const k in require.extensions) {
    const fn = require.extensions[k]

    require.extensions[k] = (mod, file) => {
      if (!file.match(/node_modules/)) {
        const watcher = () => {
          console.log(file, 'changed')
          fs.unwatchFile(file, watcher)
          watchers.delete(file)

          let m = mod
          while (m) {
            console.log('forgetting', m.id)
            require.cache[m.id] = undefined
            m = m.parent
          }

          for (const cb of cbs) {
            cb()
          }
        }

        fs.watchFile(file, { interval: 100 }, watcher)
        watchers.set(file, watcher)
      }

      fn(mod, file)
    }
  }

  Module.prototype['hot'] = {
    onChange: (cb) => {
      cbs.push(cb)
    }
  }
}
