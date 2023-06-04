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
document.documentElement = document.createElement('html')
document.appendChild(document.documentElement)
document.head = document.createElement('head')
document.documentElement.appendChild(document.head)
document.body = document.createElement('body')
document.documentElement.appendChild(document.body)

wrap(Window, window)
// TODO: should be window
document.documentElement.addEventListener('close', () => process.exit())
