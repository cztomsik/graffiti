import * as React from 'react'
import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback
} from 'scheduler'
import initDevtools from './devtools'

import { Size, Color, Flex, Image, Border, Text, Flow } from '../core'
import {
  mkMsgAlloc,
  mkMsgSetFlow,
  mkMsgSetImage,
  mkMsgSetText,
  mkMsgSetPadding,
  mkMsgSetMargin,
  mkMsgSetFlex,
  mkMsgSetSize,
  mkMsgSetBackgroundColor,
  mkMsgSetBorder,
  mkMsgSetBoxShadow,
  BoxShadow
} from '../core/generated'
import { send } from '../core/nativeApi'

// temporary helpers
// because root is 0
let __nextId__ = 1
let tx = null

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

  // initial tx
  prepareForCommit(window)

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function prepareForCommit(window) {
  // prepareForCommit is called before any update but also before initial
  // append. I'd love to do this otherwise but I have no idea what reconciler
  // actually calls and when (and if it's not going to change in next version)
  if (tx === null) {
    tx = window.createTransaction()
  }
}

function createInstance(type, props) {
  send(mkMsgAlloc())
  let id = __nextId__++

  update(id, props, {})

  return id
}

function update(surface, props: HostSurfaceProps, oldProps: HostSurfaceProps) {
  if (props.size !== oldProps.size) {
    tx.sceneMsgs.push(mkMsgSetSize({ surface, size: props.size }))
  }

  if (props.flex !== oldProps.flex) {
    tx.sceneMsgs.push(mkMsgSetFlex({ surface, flex: props.flex }))
  }

  if (props.flow !== oldProps.flow) {
    tx.sceneMsgs.push(mkMsgSetFlow({ surface, flow: props.flow }))
  }

  if (props.padding !== oldProps.padding) {
    tx.sceneMsgs.push(mkMsgSetPadding({ surface, padding: props.padding }))
  }

  if (props.margin !== oldProps.margin) {
    tx.sceneMsgs.push(mkMsgSetMargin({ surface, margin: props.margin }))
  }

  if (props.boxShadow !== oldProps.boxShadow) {
    tx.sceneMsgs.push(
      mkMsgSetBoxShadow({ surface, boxShadow: props.boxShadow })
    )
  }

  if (props.backgroundColor !== oldProps.backgroundColor) {
    tx.sceneMsgs.push(
      mkMsgSetBackgroundColor({ surface, color: props.backgroundColor })
    )
  }

  if (props.image !== oldProps.image) {
    tx.sceneMsgs.push(mkMsgSetImage({ surface, image: props.image }))
  }

  if (props.text !== oldProps.text) {
    tx.sceneMsgs.push(mkMsgSetText({ surface, text: props.text }))
  }

  if (props.border !== oldProps.border) {
    tx.sceneMsgs.push(mkMsgSetBorder({ surface, border: props.border }))
  }
}

function appendChild(parent, child) {
  tx.appendChild(parent, child)
}

function removeChild(parent, child) {
  tx.removeChild(parent, child)
}

function insertBefore(parent, child, before) {
  tx.insertBefore(parent, child, before)
}

function resetAfterCommit(window) {
  tx._send()
  tx = null
}

export interface HostSurfaceProps {
  size?: Size
  flex?: Flex
  flow?: Flow
  padding?: any
  margin?: any
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
