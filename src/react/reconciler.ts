import * as React from 'react'
import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback,
} from 'scheduler'
import initDevtools from './devtools'
import ErrorBoundary from './ErrorBoundary'
import ControlManager, { ControlManagerContext } from './ControlManager'

import { Size, Color, Flex, Image, Border, Text, Flow } from '../core'
import { Msg, mkMsgHandleEvents, mkMsgRender, mkMsgAlloc, mkMsgSetFlow, mkMsgSetImage, mkMsgSetText, mkMsgSetPadding, mkMsgSetMargin, mkMsgSetFlex, mkMsgSetSize, mkMsgSetBackgroundColor, mkMsgAppendChild, mkMsgInsertBefore, mkMsgRemoveChild, mkMsgSetBorder, mkMsgSetBoxShadow, BoxShadow } from '../core/generated'

const ref = require('ref');
const ffi = require('ffi');

// define lib
export const lib = ffi.Library(__dirname + '/../../native-new/target/debug/libnode_webrender', {
  init: ['void', []],
  // pass a buffer (pointer to some memory + its length)
  'send': ['void', [ref.refType(ref.types.void), 'int']]
});

lib.init()
send(mkMsgAlloc())

// necessary for window to stay responsive
setInterval(() => {
  send(mkMsgHandleEvents())
}, 200)

function send(msg: Msg) {
  // prepare buffer with msg
  let buf = Buffer.from(JSON.stringify(msg))

  // send (sync)
  lib.send(buf, buf.length)
}

// temporary helpers
// because root is 0
let __nextId__ = 1

const NOOP = () => undefined
const IDENTITY = v => v

const reconciler = Reconciler({
  getPublicInstance: IDENTITY,
  getRootHostContext: IDENTITY,
  getChildHostContext: IDENTITY,
  prepareForCommit: NOOP,
  resetAfterCommit,
  createInstance,
  appendInitialChild: appendChild,
  finalizeInitialChildren: NOOP,
  prepareUpdate: () => true,
  shouldSetTextContent: () => false,
  shouldDeprioritizeSubtree: () => false,
  createTextInstance: NOOP,
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

  // mutation
  appendChild,
  appendChildToContainer: appendChild,
  commitTextUpdate: NOOP,
  commitMount: NOOP,
  commitUpdate: (surface, payload, type, oldProps, newProps, handle) => update(surface, newProps, oldProps),
  insertBefore,
  insertInContainerBefore: insertBefore,
  removeChild,
  removeChildFromContainer: removeChild,
  resetTextContent: NOOP,
  hideInstance: NOOP,
  hideTextInstance: NOOP,
  unhideInstance: NOOP,
  unhideTextInstance: NOOP
})

initDevtools(reconciler)

export function render(vnode, window, cb?) {
  // add default control manager
  const rootControlManager = new ControlManager()
  vnode = React.createElement(
    ControlManagerContext.Provider,
    { value: rootControlManager },
    vnode
  )
  window.onKeyPress = e => rootControlManager.keyPress(e)

  // add default error boundary
  vnode = React.createElement(ErrorBoundary, null, vnode)

  if (window._reactRoot === undefined) {
    window._reactRoot = reconciler.createContainer(window.id, false, false)
  }

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function createInstance(type, props) {
  send(mkMsgAlloc())
  let id = __nextId__++

  update(id, props, {})

  return id
}

function update(surface, props: HostSurfaceProps, oldProps: HostSurfaceProps) {
  if (props.size !== oldProps.size) {
    send(mkMsgSetSize({ surface, size: props.size }))
  }

  if (props.flex !== oldProps.flex) {
    send(mkMsgSetFlex({ surface, flex: props.flex }))
  }

  if (props.flow !== oldProps.flow) {
    send(mkMsgSetFlow({ surface, flow: props.flow }))
  }

  if (props.padding !== oldProps.padding) {
    send(mkMsgSetPadding({ surface, padding: props.padding }))
  }

  if (props.margin !== oldProps.margin) {
    send(mkMsgSetMargin({ surface, margin: props.margin }))
  }

  if (props.boxShadow !== oldProps.boxShadow) {
    console.log(props.boxShadow)
    send(mkMsgSetBoxShadow({ surface, boxShadow: props.boxShadow }))
  }

  if (props.backgroundColor !== oldProps.backgroundColor) {
    send(mkMsgSetBackgroundColor({ surface, color: props.backgroundColor }))
  }

  if (props.image !== oldProps.image) {
    send(mkMsgSetImage({ surface, image: props.image }))
  }

  if (props.text !== oldProps.text) {
    send(mkMsgSetText({ surface, text: props.text }))
  }

  if (props.border !== oldProps.border) {
    send(mkMsgSetBorder({ surface, border: props.border }))
  }
}

function appendChild(parent, child) {
  send(mkMsgAppendChild({ parent, child }))
}

function removeChild(parent, child) {
  send(mkMsgRemoveChild({ parent, child }))
}

function insertBefore(parent, child, before) {
  send(mkMsgInsertBefore({ parent, child, before }))
}
function resetAfterCommit(window) {
  send(mkMsgRender({ surface: 0 }))
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
      key?: any,
    }

    interface IntrinsicElements {
      'host-surface': HostSurfaceProps
    }
  }
}
