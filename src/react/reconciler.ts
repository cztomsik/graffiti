import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback,
} from 'scheduler'
import { View, Text, TextNode } from './shared'

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
  prepareUpdate: (instance, type, oldProps, newProps, window) => window,
  shouldSetTextContent: () => false,
  shouldDeprioritizeSubtree: () => false,
  createTextInstance: str => new TextNode(str),
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
  appendChildToContainer,
  commitTextUpdate: (textInstance, oldText, newText) => textInstance.setValue(newText),
  commitMount: NOOP,
  commitUpdate: (instance, payload, type, oldProps, newProps, handle) => instance.update(newProps, payload),
  insertBefore,
  insertInContainerBefore,
  removeChild,
  removeChildFromContainer,
  resetTextContent: NOOP,
  hideInstance: NOOP,
  hideTextInstance: NOOP,
  unhideInstance: NOOP,
  unhideTextInstance: NOOP,
})

export function render(vnode, window, cb?) {
  if (window._reactRoot === undefined) {
    window._reactRoot = reconciler.createContainer(window, false, false)
  }

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function createInstance(type, props, window) {
  const inst = createEmpty(type, window)

  ;(inst as any).update(props, window)

  return inst
}

function createEmpty(type, window) {
  switch (type) {
    case 'wr-text': return new Text(window)
    case 'wr-view': return new View()
  }

  throw new Error('unknown type')
}

function appendChildToContainer(window, child) {
  window._root = child
  child.yogaNode.setWidth('100%')
  child.yogaNode.setHeight('100%')
}

function removeChildFromContainer(window, child) {
  assert(window._root === child)
  window._root = undefined
}

function insertInContainerBefore(window, child, before) {
  throw new Error('unsupported')
}

function appendChild(parent, child) {
  parent.appendChild(child)
}

function removeChild(parent, child) {
  parent.removeChild(child)

  // react calls removeChild only for the root of removed subtree
  child.yogaNode.freeRecursive()
}

function insertBefore(parent, child, before) {
  parent.insertBefore(child, before)
}

function resetAfterCommit(window) {
  const root = window._root

  if (root === undefined) {
    // TODO: do something about it
    console.error('no root found, this is likely because react caught some err but insists on rendering, if nothing helps, try to debug with "break on all exceptions"')
    return
  }

  root.yogaNode.calculateLayout(window.width, window.height)

  const bucketIds = []
  const layouts = []

  root.write(bucketIds, layouts, 0, 0)

  // TODO: we convert back and forth from f32 (yoga-cpp, webrender) to f64 (js)
  window.render({ bucket_ids: bucketIds, layouts })
}

// TODO: types
declare global {
  namespace JSX {
    interface IntrinsicElements {
      "wr-view": any;
      "wr-text": any;
    }
  }
}
