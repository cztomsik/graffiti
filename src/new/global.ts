import { getApp } from '../core'
import { Window } from './Window'

// setup global env for a single window
//
// for a lot of apps, this will be enough and
// they dont have to care about App API at all

global['window'] = getApp().createWindow(Window)
global['self'] = window
global['document'] = window.document
