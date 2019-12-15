import { createWindow } from '.'
import { _requestAnimationFrame } from './core/App'

// setup global env for a single window
// for a lot of apps, this will be enough

const window = createWindow()

// TODO: document what is needed for which framework
//
// - window, document: very common
// - self: ?
// - navigator: ?
// - location: mithril, wouter
// - add/removeEventListener: wouter
// - Event/dispatchEvent: wouter
for (const k of ['window', 'self', 'document', 'location', 'navigator', 'history', 'addEventListener', 'removeEventListener', 'dispatchEvent', 'Event']) {
  Object.defineProperty(global, k, {
    // getter for values, delegate for methods
    // (defined on prototype, typeof is not enough)
    //
    // intentionally late-bound
    get: (window.hasOwnProperty(k))
      ?() => window[k]
      :() => (...args) => window[k](...args)
  })
}

// TODO: global is stable but it's not yet clear if it should be shared or per-window
global['requestAnimationFrame'] = _requestAnimationFrame
