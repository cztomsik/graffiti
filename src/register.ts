import { getApp } from '../src'

// setup global env for a single window
// for a lot of apps, this will be enough

global['window'] = getApp().createWindow()
global['document'] = window.document

// TODO: leave a comment what is needed for which framework
global['self'] = window
global['navigator'] = window.navigator
