import * as Reconciler from 'react-reconciler'
import { unstable_now as now, unstable_scheduleCallback as scheduleDeferredCallback, unstable_cancelCallback as cancelDeferredCallback } from 'scheduler'
import { appendFile } from 'fs';
//import { Window } from '..'

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "display-item": Props;
    }
  }
}

type Props = {
  children?: []
}

const noop = () => undefined

const reconciler = Reconciler({
  getRootHostContext: rootContainerInstance => null,
  getChildHostContext: (parentHostContext, type, rootContainerInstance) => null,

  createInstance: (type, props, rootContainerInstance, hostContext, internalInstanceHandle) => [props],
  getPublicInstance: instance => instance,
  appendInitialChild: (parent, child) => append(parent, child),
  appendChildToContainer: (parent, child) => append(parent, child),
  removeChild: (parent, child) => parent._frame.splice(parent._frame.indexOf(child), 1),
  prepareUpdate: () => true,
  commitUpdate: (instance, payload, type, oldProps, newProps) => {
    console.log(oldProps, newProps)
    instance[0] = newProps
  },

  shouldSetTextContent: (type, props) => (typeof props.children === 'string' || typeof props.children === 'number'),
  shouldDeprioritizeSubtree: (type, props) => false,
  createTextInstance: (text, rootContainerInstance, hostContext, internalInstanceHandle) => 0,

  prepareForCommit: noop,
  resetAfterCommit: container => container._send(),
  finalizeInitialChildren: noop,

  scheduleDeferredCallback,
  cancelDeferredCallback,
  setTimeout: global.setTimeout,
  clearTimeout: global.clearTimeout,
  noTimeout: -1,
  now,

  isPrimaryRenderer: true,
  supportsMutation: true,
  supportsPersistence: false,
  supportsHydration: false
})

export function render(vnode, window, cb?) {
  window._rootContainer = reconciler.createContainer(window, false, false)
  window._frame = []
  window._send = () => window.sendFrame(serialize(flatten(window._frame)))

  return reconciler.updateContainer(vnode, window._rootContainer, null, cb)
}

function append(parent, child) {
  parent._frame.push(child)
  parent._send()
}

function serialize(frame) {
  const json = JSON.stringify(frame)

  console.log('frame', json)

  return json
}

function flatten(arr) {
  const res = []

  for (const childArr of arr) {
    res.push(...childArr)
  }

  return res
}
