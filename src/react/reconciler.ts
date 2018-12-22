import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback,
} from 'scheduler'
import { Surface, TextContainer, TextPart } from '../core'
import initDevtools from './devtools'

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
  createTextInstance: str => new TextPart(str),
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
  commitTextUpdate: (textInstance, oldText, newText) => textInstance.setValue(newText),
  commitMount: NOOP,
  commitUpdate: (instance, payload, type, oldProps, newProps, handle) => instance.update(newProps),
  insertBefore,
  insertInContainerBefore: insertBefore,
  removeChild,
  removeChildFromContainer: removeChild,
  resetTextContent: NOOP,
  hideInstance: NOOP,
  hideTextInstance: NOOP,
  unhideInstance: NOOP,
  unhideTextInstance: NOOP,
})

initDevtools(reconciler)

export function render(vnode, window, cb?) {
  if (window._reactRoot === undefined) {
    window._reactRoot = reconciler.createContainer(window, false, false)
  }

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function createInstance(type, props) {
  const inst = createEmpty(type)

  ;(inst as any).update(props)

  return inst
}

function createEmpty(type) {
  switch (type) {
    case 'host-surface': return new Surface()
    case 'host-text-container': return new TextContainer()
  }

  throw new Error('unknown type')
}

function appendChild(parent, child) {
  parent.appendChild(child)
}

function removeChild(parent, child) {
  parent.removeChild(child)

  // react calls removeChild only for the root of removed subtree
  if (child.yogaNode !== undefined) {
    child.yogaNode.freeRecursive()
  }
}

function insertBefore(parent, child, before) {
  parent.insertBefore(child, before)
}

function resetAfterCommit(window) {
  window.render()
}

declare global {
  namespace JSX {
    interface IntrinsicAttributes {
      children?: any,
      key?: any
    }

    interface IntrinsicElements {
      'host-surface': { brush?, layout?, clip? };
      'host-text-container': { color?, fontSize?, lineHeight? };
    }
  }
}
