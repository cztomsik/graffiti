import { Node } from './nodes/Node'
import { CSSStyleSheet } from './css/CSSStyleSheet'

export const createAdapter = (nativeApi, windowId, url) => {
  const NATIVE_NODE_ID = Symbol()

  const id = node => node[NATIVE_NODE_ID] || (node[NATIVE_NODE_ID] = createNativeNodeFor(node))

  const createNativeNodeFor = node => {
    if (node.nodeType === Node.ELEMENT_NODE) {
      return nativeApi.createElement(windowId, node.localName)
    }

    // TODO: we could set it when adapter is created
    if (node.nodeType === Node.DOCUMENT_NODE) {
      // pub const
      return 0
    }

    // TODO: comments?
    return nativeApi.createTextNode(windowId, node.data)
  }

  return {
    childInserted: (parent, child, index) => {
      // TODO: fragment notifies too
      if (parent.nodeType === Node.ELEMENT_NODE || parent.nodeType === Node.DOCUMENT_NODE) {
        nativeApi.insertChild(windowId, id(parent), id(child), index)
      }

      // TODO: head only
      if (child.localName === 'style') {
        // TODO: order
        // TODO: text changed
        const sheet = (child.sheet = new CSSStyleSheet(child, null))

        sheet.insertRule(child.textContent)

        // TODO: native style
        //console.log(sheet)
      }

      // TODO: head/body only
      if (child.localName === 'script') {
        const { text, src } = child

        if (src) {
          // TODO: queue (or just chain promises in global var - but use finally)
          console.log('[import]', src)
          const promise = import('' + new URL(src, url))
        } else if (text) {
          console.log('[eval]', text)
          const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor
          new AsyncFunction('__filename', text.replace(/import\s+(".*?")/gi, 'await import(new URL($1, __filename))'))(
            url
          )
        }
      }
    },

    childRemoved: (parent, child) => {
      // TODO: fragment notifies too
      if (parent.nodeType === Node.ELEMENT_NODE || parent.nodeType === Node.DOCUMENT_NODE) {
        nativeApi.removeChild(windowId, id(parent), id(child))
      }
    },

    styleChanged: (el, prop, value) => nativeApi.setStyle(windowId, id(el), prop, value),

    attributeChanged: (el, attName, value) => {
      if (value === null) {
        nativeApi.removeAttribute(windowId, id(el), attName)
      } else {
        nativeApi.setAttribute(windowId, id(el), attName, value)
      }
    },

    dataChanged: (textNode, data) => nativeApi.setText(windowId, id(textNode), data),
  }
}
