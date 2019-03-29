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

import { SceneContext } from '../core'
import { NOOP, IDENTITY } from '../core/utils'
import { WindowEvent } from '../core/generated';
import { ViewProps, StyleProp } from './react-native-types'
import StyleSheet, { SurfaceProps } from './Stylesheet';
import { isEqual } from 'lodash'

let ctx: SceneContext = null

const reconciler = createReconciler({
  prepareForCommit,
  createInstance,
  appendInitialChild: appendChild,
  appendChild,
  appendChildToContainer: (window, child) =>
    appendChild(window.rootSurface, child),
  commitUpdate: (surface, window, type, oldProps, newProps, handle) =>
    update(surface, newProps, oldProps),
  insertBefore,
  insertInContainerBefore: (window, child, before) =>
    insertBefore(window.rootSurface, child, before),
  removeChild,
  removeChildFromContainer: (window, child) =>
    removeChild(window.rootSurface, child),
  resetAfterCommit
})

export function render(vnode, window, cb?) {
  if (window._reactRoot === undefined) {
    window._reactRoot = reconciler.createContainer(window, false, false)

    const ctx = window.getSceneContext()
    const nsw = new NotSureWhat(ctx.parents)
    ctx['events'] = nsw
    ctx['surfaceProps'] = [{}]
    window.handleEvent = (e) => nsw.handleWindowEvent(e)
  }

  // initial update
  prepareForCommit(window)

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function prepareForCommit(window) {
  // prepareForCommit is called before any update but also before initial
  // append. I'd love to do this better but I have no idea what reconciler
  // actually calls and when (and if it's not going to change in next version)
  ctx = window.getSceneContext()
}

function createInstance(type, props, window) {
  // because it might be null if we are creating instance after listener was fired
  ctx = window.getSceneContext()

  let id = ctx.createSurface()

  ctx['events'].alloc()
  ctx['surfaceProps'].push({})
  update(id, props, {})

  return id
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
        // check if style is equal first (& skip if no update is necessary)
        if (k === 'style') {
          if (styleEqual(v, prev)) {
            continue
          }
        }

        setProp(surface, k, v)
      }
    }
  }

  // remove missing props
  if (oldProps !== undefined) {
    for (const k in oldProps) {
      if (k !== 'children' && !(k in props)) {
        setProp(surface, k, null)
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

function setProp(surface, prop, value) {
  if (prop === 'style') {
    //TODO: setStyle(surface, value)
    const { _surfaceProps } = StyleSheet.flatten(value)
    patchStyle(surface, _surfaceProps, ctx['surfaceProps'][surface])
    ctx['surfaceProps'][surface] = _surfaceProps
  }

  if (prop === '_text') {
    ctx.setText(surface, value ?value :undefined)
  }

  // listeners
  if (prop[0] === 'o' && prop[1] === 'n') {
    ctx['events'].setEventListener(surface, prop, value === 'undefined' ?NOOP :value)
  }
}

// TODO: diff style composition first (like we do with props) - most of them are the same and usually shallow
function patchStyle(surface, props: SurfaceProps, oldProps: SurfaceProps) {
  if (props.size !== oldProps.size) {
    ctx.setSize(surface, props.size)
  }

  if (props.flex !== oldProps.flex) {
    ctx.setFlex(surface, props.flex)
  }

  if (props.flow !== oldProps.flow) {
    ctx.setFlow(surface, props.flow)
  }

  if (props.padding !== oldProps.padding) {
    ctx.setPadding(surface, props.padding)
  }

  if (props.margin !== oldProps.margin) {
    ctx.setMargin(surface, props.margin)
  }

  if (props.borderRadius !== oldProps.borderRadius) {
    ctx.setBorderRadius(surface, props.borderRadius)
  }

  if (props.boxShadow !== oldProps.boxShadow) {
    ctx.setBoxShadow(surface, props.boxShadow)
  }

  if (props.backgroundColor !== oldProps.backgroundColor) {
    ctx.setBackgroundColor(surface, props.backgroundColor)
  }

  if (props.image !== oldProps.image) {
    ctx.setImage(surface, props.image)
  }

  if (props.border !== oldProps.border) {
    ctx.setBorder(surface, props.border)
  }
}

function appendChild(parent, child) {
  ctx.appendChild(parent, child)
}

function removeChild(parent, child) {
  ctx.removeChild(parent, child)
}

function insertBefore(parent, child, before) {
  ctx.insertBefore(parent, child, before)
}

function resetAfterCommit(window) {
  ctx = null
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

    ...cfg
  })
}


// events
// it shouldn't be here but I don't want to put it to the core yet and it needs to know
// about hiearchy because of bubbling
//
// there is so much work to be done I don't even know where to start but here
// are some useful notes:
// - reuse existing event types if possible (either RN or DOM), it doesn't have to be 100%
//   the same but it's great not having to re-learn everything
// - bubbling has its issues but any different approach would be very surprising
// - it should be enough to support only one listener for each (surface, type) pair (vs addEventListener)
//   - simpler/faster, edge cases can be handled in user-space (if necessary at all)
// - View shouldn't be responsible for registering events, it doesn't even know about window
//   - and we want it to be stateless
export class NotSureWhat {
  listeners: EventListeners = {
    onMouseMove: [],
    onMouseDown: [],
    onMouseUp: [],
    onClick: []
  }
  moveTarget = 0
  downTarget = 0

  constructor(private parents) {
    // root
    this.alloc()
  }

  alloc() {
    for (const k in this.listeners) {
      this.listeners[k].push(NOOP)
    }
  }

  handleWindowEvent(event: WindowEvent) {
    console.log(event)

    switch (event.tag) {
      case "MouseMove": {
        const target = this.moveTarget = event.value.target
        return this.dispatch(this.listeners.onMouseMove, target, { target })
      }
      case "MouseDown": {
        const target = this.downTarget = this.moveTarget
        return this.dispatch(this.listeners.onMouseDown, target, { target })
      }
      case "MouseUp": {
        const target = this.moveTarget

        this.dispatch(this.listeners.onMouseUp, target, { target })

        if (this.moveTarget === this.downTarget) {
          this.dispatch(this.listeners.onClick, target, { target })
        }

        return
      }
    }
  }

  setEventListener<K extends keyof EventMap>(id, type: K, listener: Listener<EventMap[K]>) {
    if (!(type in this.listeners)) {
      throw new Error(`${type} is not supported`)
    }

    this.listeners[type][id] = listener
  }

  // dispatch event to target and all its parents
  // TODO: stopPropagation()
  dispatch<T>(listeners: Listener<T>[], id, event) {
    while (id) {
      listeners[id](event)

      id = this.parents[id]
    }
  }
}

// events we support
interface EventMap {
  onMouseMove: MouseEvent,
  onMouseDown: MouseEvent,
  onMouseUp: MouseEvent,
  onClick: MouseEvent
}

type Listener<E> = (ev: E) => any

// struct of arrays (listeners for each type)
type EventListeners = {
  [K in keyof EventMap]: Listener<EventMap[K]>[]
}
