import { getApp } from '..'
import { Window } from '../dom/Window'

// setup global env for a single window
//
// for a lot of apps, this will be enough and
// they dont have to care about App API at all

global['window'] = getApp().createWindow()
global['self'] = window
global['document'] = window.document
global['navigator'] = window.navigator
