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
  ctx._send()
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
  const NOOP = () => undefined
  const IDENTITY = v => v

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
