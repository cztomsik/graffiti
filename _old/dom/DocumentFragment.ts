import { Node, Document } from './index'

export class DocumentFragment extends Node implements globalThis.DocumentFragment {
  // fragment can stay JS-only, it can operate on "invisible" native element
  // and if it's private it should be fine because element should not be observable until connected
  // JS-part should work mostly the same and we will get querySelector() for free
  #element = this.ownerDocument.createElement('#document-fragment')

  // @ts-expect-error
  get childNodes() {
    return this.#element.childNodes
  }

  constructor(doc = document as Document) {
    super(doc)
  }

  get nodeType() {
    return Node.DOCUMENT_FRAGMENT_NODE
  }

  get nodeName() {
    return '#document-fragment'
  }

  querySelector(sel) {
    return this.#element.querySelector(sel)
  }

  querySelectorAll(sel) {
    return this.#element.querySelectorAll(sel)
  }
}
