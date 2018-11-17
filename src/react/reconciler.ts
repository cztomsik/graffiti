import * as Reconciler from 'react-reconciler'
import { unstable_now as now, unstable_scheduleCallback as scheduleDeferredCallback, unstable_cancelCallback as cancelDeferredCallback } from 'scheduler'
import * as yoga from 'yoga-layout'

const noop = () => undefined
const identity = v => v
const LAYOUT_EMPTY = [0, 0, 0, 0]

const reconciler = Reconciler({
  getPublicInstance: identity,
  getRootHostContext: identity,
  getChildHostContext: identity,
  prepareForCommit,
  resetAfterCommit,
  createInstance,
  appendInitialChild: appendChild,
  finalizeInitialChildren: noop,
  prepareUpdate: noop,
  shouldSetTextContent: () => false,
  shouldDeprioritizeSubtree: () => false,
  createTextInstance,
  scheduleDeferredCallback,
  cancelDeferredCallback,
  setTimeout,
  clearTimeout,
  noTimeout: -1,
  now: now,
  isPrimaryRenderer: true,
  supportsMutation: true,
  supportsPersistence: false,
  supportsHydration: false,

  appendChild,
  appendChildToContainer: (ct, child) => {
    ct._rootNode = child
    child.yogaNode.setWidth('100%')
    child.yogaNode.setHeight('100%')
  },
  commitTextUpdate: noop,
  commitMount: noop,
  commitUpdate: (node, payload, type, oldProps, newProps, handle) => updateNode(node, newProps),
  insertBefore,
  insertInContainerBefore: (ct, child, before) => insertBefore(ct._rootNode, child, before),
  removeChild,
  removeChildFromContainer: (ct, child) => removeChild(ct._rootNode, child),
  resetTextContent: noop
})

export function render(vnode, window, cb?) {
  currentWindow = window

  if (window._rootContainer === undefined) {
    window._rootContainer = reconciler.createContainer(window, false, false)

    window.TEXT_STACKING_CONTEXT = bucket({
      PushStackingContext: {
        stacking_context: {
          transform_style: 'Flat',
          mix_blend_mode: 'Normal',
          raster_space: 'Screen'
        }
      }
    })
    window.POP_STACKING_CONTEXT = bucket({ PopStackingContext: null })
  }

  return reconciler.updateContainer(vnode, window._rootContainer, null, cb)
}

let currentWindow = null

function prepareForCommit(container) {
  currentWindow = container
}

function createInstance(type, props) {
  const node = {
    yogaNode: yoga.Node.create(),
    background: undefined,
    border: undefined,
    children: []
  }

  updateNode(node, props)

  return node
}

function createTextInstance(str, container) {
  const indices = container.getGlyphIndices(str)
  const glyphs = indices.map((glyph_index, i) => [glyph_index, [i * 16, 24]])

  const node = createInstance('text', {
    layout: ['auto', 'auto', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    stackingContext: currentWindow.TEXT_STACKING_CONTEXT,
    text: {
      Text: [{
        font_key: [1, 2],
        color: [0, 0, 0, 1]
      }, glyphs]
    }
  })

  node.yogaNode.setMeasureFunc((...args) => {
    console.log('measure', args)

    return { width: (str.length * 16) + 5, height: 30 }
  })

  return node
}

function appendChild(parent, child) {
  parent.yogaNode.insertChild(child.yogaNode, parent.children.length)
  parent.children.push(child)
}

function removeChild(parent, child) {
  parent.children.splice(parent.children.indexOf(child), 1)
  parent.yogaNode.removeChild(child)
}

function insertBefore(parent, child, before) {
  const index = parent.children.indexOf(before)

  parent.children.splice(index, 0, child)
  parent.yogaNode.insertChild(child, index)
}

function updateNode(node, { stackingContext, layout, background, text, border }) {
  updateYogaNode(node.yogaNode, layout)

  node.stackingContext = bucket(stackingContext)
  node.background = bucket(background)
  node.text = bucket(text)
  node.border = bucket(border)
}

function resetAfterCommit() {
  const rootNode = currentWindow._rootNode

  rootNode.yogaNode.calculateLayout(800, 600)

  const bucketIds = []
  const layouts = []

  writeNode(bucketIds, layouts, rootNode, 0, 0)

  console.log('render', bucketIds, layouts)

  // TODO: we convert back and forth from f32 (yoga-cpp, webrender) to f64 (js)
  currentWindow.render({ bucket_ids: bucketIds, layouts })
}

function writeNode(bucketIds, layouts, node, x, y) {
  const { left, top, width, height } = node.yogaNode.getComputedLayout()
  const layout = [left, top, width, height]

  if (node.stackingContext !== undefined) {
    bucketIds.push(node.stackingContext)
    layouts.push([layout[0] + x, layout[1] + y, layout[2], layout[3]])

    // now everything is at [0, 0]
    layout[0] = 0
    layout[1] = 0
  } else {
    layout[0] += x
    layout[1] += y
  }

  if (node.background !== undefined) {
    bucketIds.push(node.background)
    layouts.push(layout)
  }

  if (node.text !== undefined) {
    bucketIds.push(node.text)
    // pozor, pushnout s 0, 0 pozici
    layouts.push(layout)
  }

  node.children.forEach(c => writeNode(bucketIds, layouts, c, layout[0], layout[1]))

  // TODO: clip

  if (node.stackingContext !== undefined) {
    bucketIds.push(currentWindow.POP_STACKING_CONTEXT)
    layouts.push(LAYOUT_EMPTY)
  }

  if (node.border !== undefined) {
    bucketIds.push(node.border)
    layouts.push(layout)
  }
}

// TODO: should be in the same "land" with yoga so there is no hopping back and forth)
// takes array of numbers
function updateYogaNode(n: yoga.YogaNode, values) {
  let i = 0
  const v = values

  n.setWidth(v[i++])
  n.setHeight(v[i++])
  n.setFlexDirection(v[i++])
  n.setFlex(v[i++])
  // n.setFlexGrow(v[i++])
  // n.setFlexShrink(v[i++])

  n.setMargin(yoga.EDGE_TOP, v[i++])
  n.setMargin(yoga.EDGE_RIGHT, v[i++])
  n.setMargin(yoga.EDGE_BOTTOM, v[i++])
  n.setMargin(yoga.EDGE_LEFT, v[i++])

  n.setPadding(yoga.EDGE_TOP, v[i++])
  n.setPadding(yoga.EDGE_RIGHT, v[i++])
  n.setPadding(yoga.EDGE_BOTTOM, v[i++])
  n.setPadding(yoga.EDGE_LEFT, v[i++])
}

// TODO: magic LRU cache?
// for now, it's ok to just create new bucket every time
function bucket(data) {
  return data && currentWindow.createBucket(data)
}

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "node": any;
    }
  }
}
