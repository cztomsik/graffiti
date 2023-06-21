// minimal subset of DOM and CSSOM to get Preact working
// everything else is supposed to be in polyfills
//
// note that everything is in one file, this is intentional, we want to keep
// things simple & minimal

import { native, wrap } from './native.js'
import { Document } from './Document.js'
import { Window } from './Window.js'

Object.assign(global, native.init())

// TODO: createDocument(), createWindow()
wrap(Document, document)
document.appendChild(document.createElement('html'))
document.documentElement.appendChild(document.createElement('head'))
document.documentElement.appendChild(document.createElement('body'))

wrap(Window, window)
// TODO: should be window
document.documentElement.addEventListener('close', () => process.exit())
