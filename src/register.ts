import { createWindow } from '.'
import { _requestAnimationFrame } from './core/App'

// setup global env for a single window
// for a lot of apps, this will be enough

global['window'] = createWindow()
global['document'] = window.document

// TODO: leave a comment what is needed for which framework
global['self'] = window
global['navigator'] = window.navigator

// TODO: global is stable but it's not yet clear if it should be shared or per-window
global['requestAnimationFrame'] = _requestAnimationFrame
