// minimal subset of DOM and CSSOM to get Preact working
// everything else is supposed to be in polyfills
//
// note that everything is in one file, this is intentional, we want to keep
// things simple & minimal

import { native, wrap } from './native.js'
import { Document } from './Document.js'
import { Window } from './Window.js'

// TODO: createDocument(), createWindow()
const { document, window } = native.init()

wrap(Document, document)
document.appendChild(document.createElement('html'))
document.documentElement.appendChild(document.createElement('head'))
document.documentElement.appendChild(document.createElement('body'))
document.defaultView = window

wrap(Window, window)
window.document = document
// TODO: should be window
document.documentElement.addEventListener('close', () => process.exit())

// monkey-patch global scope
// TODO: https://github.com/cztomsik/graffiti/blob/83695bf44c64ce7a6dd433b84b56e2da331304ac/src/window/Window.ts
Object.setPrototypeOf(
  globalThis,
  new Proxy(window, {
    get: (target, key) => Reflect.get(target, key),
    set: (target, key, value) => Reflect.set(target, key, value),
  })
)
