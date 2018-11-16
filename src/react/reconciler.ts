import * as Reconciler from 'react-reconciler'
import { unstable_now as now, unstable_scheduleCallback as scheduleDeferredCallback, unstable_cancelCallback as cancelDeferredCallback } from 'scheduler'
import * as yoga from 'yoga-layout'

const noop = () => undefined
const identity = v => v

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
  createTextInstance: createText,
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
  commitUpdate,
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
  }

  return reconciler.updateContainer(vnode, window._rootContainer, null, cb)
}

let currentWindow = null

function prepareForCommit(container) {
  currentWindow = container
}

function createInstance(type, props) {
  console.log('create', type, props)

  const node = {
    yogaNode: yoga.Node.create(),
    background: undefined,
    border: undefined,
    children: []
  }

  commitUpdate(node, undefined, type, null, props, null)

  return node
}

function createText(str, container) {
  console.log('createText')
  /*
  const indices = container.getGlyphIndices(str)
  const glyphs = indices.map((glyph_index, i) => [glyph_index, [i * 20, 20]])

  const node = createNode({ layout: ['auto', 'auto', 0, 1], appearance: {
    type: 'Text',
    color: [0, 0, 0, 1],
    font_instance_key: [1, 2],
    glyphs
  } })

  return node*/
}

function appendChild(parent, child) {
  console.log('append', parent, child)

  parent.yogaNode.insertChild(child.yogaNode, parent.children.length)
  parent.children.push(child)
}

function removeChild(parent, child) {
  console.log('remove', parent, child)

  parent.children.splice(parent.children.indexOf(child), 1)
  parent.yogaNode.removeChild(child)
}

function insertBefore(parent, child, before) {
  console.log('insertBefore', parent, child, before)

  const index = parent.children.indexOf(before)

  parent.children.splice(index, 0, child)
  parent.yogaNode.insertChild(child, index)
}

function commitUpdate(node, payload, type, oldProps, { layout, background, border }, handle) {
  try {
    updateYogaNode(node.yogaNode, layout)

    node.background = bucket(background)
    node.border = bucket(border)
  } catch (e) {
    // one's gotta love react - it's soooo easy to debug reconcillers
    console.log(e)
    console.error('err')
    throw e
  }
}

function resetAfterCommit() {
  console.log('reset')

  const rootNode = currentWindow._rootNode

  rootNode.yogaNode.calculateLayout(800, 600)

  const bucketIds = []
  const layouts = []

  writeNode(bucketIds, layouts, rootNode, 0, 0)

  console.log('render', bucketIds, layouts)

  currentWindow.render({ bucket_ids: bucketIds, layouts })
}

function writeNode(bucketIds, layouts, node, x, y) {
  const { left, top, width, height } = node.yogaNode.getComputedLayout()
  const layout = [left + x, top + y, width, height]

  console.log('write', node)

  // TODO: push

  if (node.background !== undefined) {
    bucketIds.push(node.background)
    layouts.push(layout)
  }

  node.children.forEach(c => writeNode(bucketIds, layouts, c, layout[0], layout[1]))

  // TODO: clip

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
