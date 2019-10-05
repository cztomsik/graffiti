// MIND that we optimize for the optimistic case (few updates in the whole tree)
//
// 1. most updates should be avoided at the app-level (redux, mobx, ...)
// 2. components can avoid a lot too
// 3. reconciler
//    - quickly detect if the props (and listeners) are the same (inside)
//    - check if the style combination is the same
//    - eventually update the style and/or listeners

import * as Reconciler from 'react-reconciler'
import * as scheduler from 'scheduler'
import initDevtools from './devtools'

import { NOOP, IDENTITY, kebabCase } from '../core/utils'
import { ViewProps, StyleProp } from 'react-native'
import StyleSheet from './Stylesheet';
import { isEqual } from 'lodash'
import ErrorBoundary from './ErrorBoundary';
import { Element } from '../dom/Element';

const styleNameCache = {}

const reconciler = createReconciler({
  createInstance,
  appendInitialChild: appendChild,
  appendChild,
  appendChildToContainer: (window, child) =>
    appendChild(window.document.body, child),
  commitUpdate: (surface, window, type, oldProps, newProps, handle) =>
    update(surface, newProps, oldProps),
  insertBefore,
  insertInContainerBefore: (window, child, before) =>
    insertBefore(window.document.body, child, before),
  removeChild,
  removeChildFromContainer: (window, child) =>
    removeChild(window.document.body, child),
})

export function render(vnode, window, cb?) {
  // TODO: causes some err with react-devtools
  vnode = ErrorBoundary.wrap(vnode)

  if (window._reactRoot === undefined) {
    window._reactRoot = reconciler.createContainer(window, false, false)
  }

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function createInstance(type, props, window) {
  let el = window.document.createElement(type)
  return update(el, props, {}), el
}

function update(surface, props: ViewProps, oldProps: ViewProps) {
  // all of this is actually faster than looking up (missing) properties
  // it's also very common pattern in JS, so it's well optimized in V8
  // (and we are just going through slots, in the physical memory order,
  //  so it shouldn't be slow anyway, js objects are not hash tables)
  //
  // it's also worth pointing out that mostly, props are of the same shape,
  // so there should be very few prop-misses
  //
  // there is usually just a few keys (style + children)

  // update existing props with new values
  for (const k in props) {
    if (k !== 'children') {
      const v = props[k]
      const prev = oldProps[k]

      if (v !== prev) {
        setProp(surface, k, v, prev)
      }
    }
  }

  // remove missing props
  if (oldProps !== undefined) {
    for (const k in oldProps) {
      if (k !== 'children' && !(k in props)) {
        setProp(surface, k, null, undefined)
      }
    }
  }
}

function styleEqual(a: StyleProp<any>, b: StyleProp<any>): boolean {
  // TODO: own, optimized (and readable) version
  // [a] == [a]
  // [a1, [a2]] == [a1, [a2]]
  // [{a: 1}] == [{a: 1}]
  // ...
  return isEqual(a, b)
}

function setProp(el: Element, prop, value, prev) {
  if (prop === 'style') {
    // check if it is equal first (& skip if no update is necessary)
    if (styleEqual(value, prev)) {
      return
    }

    value = StyleSheet.flatten(value || {})
    prev = StyleSheet.flatten(prev || {})

    for (const k in prev) {
      if (!(k in value)) {
        el.style.setProperty(styleName(k), undefined)
      }
    }

    for (const k in value) {
      if (value[k] !== prev[k]) {
        el.style.setProperty(styleName(k), value[k])
      }
    }
  }

  function styleName(propName) {
    return styleNameCache[propName] || (styleNameCache[propName] = kebabCase(propName))
  }

  // listeners
  if (prop[0] === 'o' && prop[1] === 'n') {
    const type = prop.slice(2).toLowerCase()

    if (prev) {
      el.removeEventListener(type, prev)
    }

    if (value) {
      el.addEventListener(type, value)
    }
  }
}

function appendChild(parent, child) {
  parent.appendChild(child)
}

function removeChild(parent, child) {
  parent.removeChild(child)
}

function insertBefore(parent, child, before) {
  parent.insertBefore(child, before)
}

declare global {
  namespace JSX {
    interface IntrinsicAttributes {
      children?: any
      key?: any
    }

    interface IntrinsicElements {
      'View': ViewProps & { _text? }
    }
  }
}

initDevtools(reconciler)

// factory with reasonable defaults so that we dont need to read it all the time
function createReconciler(cfg) {
  return Reconciler({
    getPublicInstance: IDENTITY,
    getRootHostContext: IDENTITY,
    getChildHostContext: IDENTITY,
    prepareUpdate: () => true,
    shouldSetTextContent: () => false,
    shouldDeprioritizeSubtree: () => false,
    createTextInstance: NOOP,
    finalizeInitialChildren: NOOP,
    scheduleDeferredCallback: scheduler.unstable_scheduleCallback,
    cancelDeferredCallback: scheduler.unstable_cancelCallback,
    schedulePassiveEffects: scheduler.unstable_scheduleCallback,
    cancelPassiveEffects: scheduler.unstable_cancelCallback,
    shouldYield: scheduler.unstable_shouldYield,
    scheduleTimeout: setTimeout,
    cancelTimeout: clearTimeout,
    noTimeout: -1,
    now: scheduler.unstable_now,
    isPrimaryRenderer: true,
    supportsMutation: true,
    supportsPersistence: false,
    supportsHydration: false,
    commitTextUpdate: NOOP,
    commitMount: NOOP,
    resetTextContent: NOOP,
    hideInstance: NOOP,
    hideTextInstance: NOOP,
    unhideInstance: NOOP,
    unhideTextInstance: NOOP,
    prepareForCommit: NOOP,
    resetAfterCommit: NOOP,

    ...cfg
  })
}
