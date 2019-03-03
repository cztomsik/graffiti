import * as React from 'react'
import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback
} from 'scheduler'
import initDevtools from './devtools'
import ErrorBoundary from './ErrorBoundary'
import ControlManager, { ControlManagerContext } from './ControlManager'

import { Msg, mkMsgHandleEvents, mkMsgRender, mkMsgAlloc, Size, Color, Flex, Image, Border, Text, mkMsgSetPadding, mkMsgSetMargin, mkMsgSetFlex, mkDimensionAuto, mkSize, mkDimensionPercent, Flow, mkMsgSetFlow } from '../astToTs/generated/example'
import { mkMsgSetSize, mkMsgSetBackgroundColor, mkMsgAppendChild, mkMsgInsertBefore, mkMsgRemoveChild } from '../astToTs/generated/example'

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
send(mkMsgSetSize({ surface: 0, size: mkSize(mkDimensionPercent(100), mkDimensionAuto()) }))
send(mkMsgSetFlex({ surface: 0, flex: { flexGrow: 1, flexShrink: 1, flexBasis: mkDimensionAuto() } }))

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
let __ROOT_SURFACE__ = 0
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
    window._reactRoot = reconciler.createContainer(__ROOT_SURFACE__, false, false)
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

  if (props.backgroundColor !== oldProps.backgroundColor) {
    send(mkMsgSetBackgroundColor({ surface, color: props.backgroundColor }))
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
