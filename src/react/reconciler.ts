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
  createInstance: (type, props) => createNode(props),
  appendInitialChild: appendChild,
  finalizeInitialChildren: noop,
  prepareUpdate: noop,
  shouldSetTextContent: () => false,
  shouldDeprioritizeSubtree: () => false,
  createTextInstance: identity,
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
    child.yogaNode.setWidth(800)
    child.yogaNode.setHeight(800)
  },
  commitTextUpdate: noop,
  commitMount: noop,
  commitUpdate: noop,
  insertBefore,
  insertInContainerBefore: (ct, child, before) => insertBefore(ct._rootNode, child, before),
  removeChild,
  removeChildFromContainer: (ct, child) => removeChild(ct._rootNode, child),
  resetTextContent: noop
})

export function render(vnode, window, cb?) {
  if (window._rootContainer === undefined) {
    window._rootContainer = reconciler.createContainer(window, false, false)
  }

  return reconciler.updateContainer(vnode, window._rootContainer, null, cb)
}

let currentWindow = null

function prepareForCommit(container) {
  currentWindow = container
}

function createNode({ children = [], layout, appearance }) {
  console.log('create', layout, appearance)

  const node = {
    yogaNode: yoga.Node.create(),
    children: []
  }

  update(node, { layout, appearance })

  return node
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

function update(node, { layout, appearance }) {
  updateYogaNode(node.yogaNode, layout)

  node.appearance = appearance
}

function resetAfterCommit() {
  console.log('rerender')

  const rootNode = currentWindow._rootNode

  rootNode.yogaNode.calculateLayout()

  const buf = []

  writeNode(buf, rootNode)

  currentWindow.sendFrame(generateFrame(buf))
}

function writeNode(buf, node) {
  const { left, top, width, height } = node.yogaNode.getComputedLayout()
  const rect = [[left, top], [width, height]]

  buf.push({
    type: node.appearance.type,
    info: { rect, clip_rect: rect, is_backface_visible: true },
    color: node.appearance.color
  })

  node.children.forEach(n => writeNode(buf, n))
}

function generateFrame(items) {
  console.log('items', items)

  const frame = JSON.stringify(items, null, 2)

  console.log('frame', frame)

  return frame
}

// TODO: should be in the same "land" with yoga so there is no hopping back and forth)
// takes array of numbers
function updateYogaNode(n: yoga.YogaNode, values) {
  let i = 0
  const v = values

  n.setFlexDirection(v[i++])
  n.setFlex(v[i++])
  // n.setFlexGrow(v[i++])
  // n.setFlexShrink(v[i++])

  // n.setMargin(yoga.EDGE_LEFT, v[i++])
  // n.setMargin(yoga.EDGE_TOP, v[i++])
  // n.setMargin(yoga.EDGE_RIGHT, v[i++])
  // n.setMargin(yoga.EDGE_BOTTOM, v[i++])

  // n.setPadding(yoga.EDGE_LEFT, v[i++])
  // n.setPadding(yoga.EDGE_TOP, v[i++])
  // n.setPadding(yoga.EDGE_RIGHT, v[i++])
  // n.setPadding(yoga.EDGE_BOTTOM, v[i++])

}

declare global {
  namespace JSX {
    interface IntrinsicElements {
      "node": any;
    }
  }
}
