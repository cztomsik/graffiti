// TODO: https://github.com/cztomsik/graffiti/blob/bd1dfe61d3d7b5bfbf9184ecfb9e068dda982a60/src/dom/Document.ts

import { native, wrap } from './native.js'
import { Node } from './Node.js'
import { Element } from './Element.js'
import { Text } from './Text.js'

export class Document extends Node {
  get nodeType() {
    return Node.DOCUMENT_NODE
  }

  get nodeName() {
    return '#document'
  }

  get ownerDocument() {
    return null
  }

  get documentElement() {
    return native.Document_documentElement(this)
  }

  get head() {
    return native.Document_head(this)
  }

  get body() {
    return native.Document_body(this)
  }

  createElement(localName) {
    return wrap(Element, native.Document_createElement(this, localName))
  }

  createTextNode(data) {
    return wrap(Text, native.Document_createTextNode(this, '' + data))
  }

  elementFromPoint(x, y) {
    return native.Document_elementFromPoint(document, x, y)
  }

  get isConnected() {
    return true
  }
}
