import * as React from 'react'
import * as Reconciler from 'react-reconciler'
import {
  unstable_now as now,
  unstable_scheduleCallback as scheduleDeferredCallback,
  unstable_shouldYield as shouldYield,
  unstable_cancelCallback as cancelDeferredCallback
} from 'scheduler'
import { Surface, TextContainer, TextPart, Img } from '../core'
import initDevtools from './devtools'
import ErrorBoundary from './ErrorBoundary'
import ControlManager, { ControlManagerContext } from './ControlManager'
import { BridgeBrush, BridgeClip } from '../core/ResourceManager'
import { N } from '../core/nativeApi'
import { BridgeColor } from '../core/RenderOperation'

const enum ElementType {
  Surface = 'host-surface',
  TextContainer = 'host-text-container',
  Image = 'host-image'
}

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
  commitTextUpdate: (textInstance, oldText, newText) =>
    textInstance.setValue(newText),
  commitMount: NOOP,
  commitUpdate: (instance, payload, type, oldProps, newProps, handle) =>
    instance.update(newProps),
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
    window._reactRoot = reconciler.createContainer(window, false, false)
  }

  return reconciler.updateContainer(vnode, window._reactRoot, null, cb)
}

function createInstance(type, props) {
  const inst = createEmpty(type)
  ;(inst as any).update(props)

  return inst
}

function createEmpty(type: ElementType) {
  switch (type) {
    case ElementType.Surface:
      return new Surface()
    case ElementType.TextContainer:
      return new TextContainer()
    case ElementType.Image:
      return new Img()
  }

  throw new Error('unknown type')
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

function resetAfterCommit(window) {
  window.renderLater()
}

export interface HostSurfaceProps {
  brush?: BridgeBrush
  layout?: N.FlexStyle
  clip?: BridgeClip
}

export interface HostTextContainerProps {
  color?: BridgeColor
  fontSize?: number
  lineHeight?: number
}

export interface HostImageProps {
  imgBrush: N.ResourceHandle
}

declare global {
  namespace JSX {
    interface IntrinsicAttributes {
      children?: any
      key?: any
    }

    interface IntrinsicElements {
      'host-surface': HostSurfaceProps

      'host-text-container': HostTextContainerProps

      'host-image': HostImageProps
    }
  }
}
