import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback
} from 'scheduler'
import initDevtools from './devtools'

import { Size, Color, Flex, Image, Border, Text, Flow, BorderRadius, BoxShadow } from '../core'
import { send } from '../core/nativeApi'
import { SceneContext } from '../core';
import { NOOP, IDENTITY } from '../core/utils'
import { WindowEvent } from '../core/generated';

let ctx: SceneContext = null

const reconciler = createReconciler({
  prepareForCommit,
  createInstance,
  appendInitialChild: appendChild,
  appendChild,
  appendChildToContainer: (window, child) =>
    appendChild(window.rootSurface, child),
  prepareUpdate: (surface, type, oldProps, newProps, window) => window,
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
  let id = ctx.createSurface()

  update(id, props, {})

  return id
}

function update(surface, props: HostSurfaceProps, oldProps: HostSurfaceProps) {
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

  if (props.text !== oldProps.text) {
    ctx.setText(surface, props.text)
  }

  if (props.border !== oldProps.border) {
    ctx.setBorder(surface, props.border)
  }

  if (props.listeners !== oldProps.listeners) {
    for (const type in props.listeners) {
      ctx['events'].setEventListener(surface, type, props.listeners[type] || NOOP)
    }
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
  ctx.flush()
  ctx = null
}

export interface HostSurfaceProps {
  size?: Size
  flex?: Flex
  flow?: Flow
  padding?: any
  margin?: any
  borderRadius?: BorderRadius
  boxShadow?: BoxShadow
  backgroundColor?: Color
  image?: Image
  text?: Text
  border?: Border
  children?: any
  listeners?: any
}

declare global {
  namespace JSX {
    interface IntrinsicAttributes {
      children?: any
      key?: any
    }

    interface IntrinsicElements {
      'host-surface': HostSurfaceProps
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
    scheduleDeferredCallback,
    cancelDeferredCallback,
    schedulePassiveEffects: scheduleDeferredCallback,
    cancelPassiveEffects: cancelDeferredCallback,
    shouldYield,
    scheduleTimeout: setTimeout,
    cancelTimeout: clearTimeout,
    noTimeout: -1,
    now,
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
//   - and we want it to be stateless, so that it can be eventually optimized with prepack


// TODO: multi-window

export class NotSureWhat {
  listeners: EventListeners = {
    onMouseMove: [],
    onMouseDown: [],
    onMouseUp: [],
    onClick: []
  }
  moveTarget = 0
  downTarget = 0

  constructor(private parents) {}

  handleWindowEvent(event: WindowEvent) {
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
    this.listeners[type][id] = listener
  }

  dispatch(listeners, id, event) {
    while (id) {
      listeners[id](event)

      id = this.parents[id]
    }
  }
}


interface EventMap {
  onMouseMove: MouseEvent,
  onMouseDown: MouseEvent,
  onMouseUp: MouseEvent,
  onClick: MouseEvent
}

type Listener<E> = (ev: E) => any

type EventListeners = {
  [K in keyof EventMap]: Listener<EventMap[K]>[]
}
