import { Node } from './Node'
import { Text } from './Text'
import { Comment } from './Comment'
import { DocumentFragment } from './DocumentFragment'

import { HTMLElement } from './HTMLElement'
import { HTMLDivElement } from './HTMLDivElement'
import { HTMLSpanElement } from './HTMLSpanElement'
import { HTMLInputElement } from './HTMLInputElement'
import { HTMLButtonElement } from './HTMLButtonElement'
import { HTMLUnknownElement } from './HTMLUnknownElement'

type IDocument = globalThis.Document

// too many type errs
export class Document extends ((Node as unknown) as typeof globalThis.Document) implements IDocument {
  nodeType = Node.DOCUMENT_NODE
  parentNode = null

  _nativeId = 0
  _scene = this.defaultView.sceneContext
  _els: HTMLElement[] = [Object.assign(new HTMLElement(this), { tagName: 'html', _nativeId: 0 })]

  // title is only defined here, not on the window itself
  title = ''

  documentElement = this._els[0]
  head = this.createElement('head')
  body = this.createElement('body')

  activeElement
  // last time over for enter/leave
  _overElement
  // mousedown origin
  _clickedElement

  constructor(public defaultView) {
    super()

    this.documentElement.parentNode = this
    this.documentElement.appendChild(this.body)
  }

  createElement(tagName: string): HTMLElement {
    let el

    switch (tagName) {
      case 'div':
        el = new HTMLDivElement(this)
        break

      case 'span':
        el = new HTMLSpanElement(this)
        break

      case 'input':
        el = new HTMLInputElement(this)
        break

      case 'button':
        el = new HTMLButtonElement(this)
        break

      default:
        el = new HTMLUnknownElement(this)
    }

    ;(el as any).ownerDocument = this
    el.tagName = tagName
    el._nativeId = this._scene.createElement()
    el._init()

    this._els.push(el)

    // apply default styles
    Object.assign(el.style, defaultStyles[tagName] || {})

    return el
  }

  createTextNode(data: string): Text {
    const t = new Text(this)
    ;(t as any).ownerDocument = this
    t.data = data
    t._nativeId = this._scene.createText()

    return t
  }

  createComment(data: string): Comment {
    const c = new Comment(this)
    ;(c as any).ownerDocument = this
    c.data = data

    // empty text node for now (should be fine)
    c._nativeId = this._scene.createText()

    return c
  }

  createDocumentFragment(): DocumentFragment {
    const f: any = new DocumentFragment(this)
    f.ownerDocument = this

    return f
  }

  getElementById(id): HTMLElement | null {
    // return this.querySelector(`#${id}`)
    return this._els.find(el => el.id === id)
  }

  querySelector(selectors: string) {
    return this.documentElement.querySelector(selectors)
  }

  querySelectorAll(selectors: string) {
    return this.documentElement.querySelectorAll(selectors)
  }

  _getEl(_nativeId) {
    return this._els[_nativeId]
  }

  _getTheParent() {
    return this.defaultView
  }
}

// TODO: share with CSSStyleDeclaration
const EM = 16

// mostly inspired by css reboot
const defaultStyles = {
  body: {
    display: 'block',
    width: '100%',
    minHeight: '100%',
  },

  div: {
    display: 'block',
  },

  h1: {
    display: 'block',
    fontSize: 2.5 * EM,
    lineHeight: 1.2 * 2.5 * EM,
    marginBottom: 0.5 * EM,
  },

  h2: {
    display: 'block',
    fontSize: 2 * EM,
    lineHeight: 1.2 * 2 * EM,
    marginBottom: 0.5 * EM,
  },

  h3: {
    display: 'block',
    fontSize: 1.75 * EM,
    lineHeight: 1.2 * 1.75 * EM,
    marginBottom: 0.5 * EM,
  },

  h4: {
    display: 'block',
    fontSize: 1.5 * EM,
    lineHeight: 1.2 * 1.5 * EM,
    marginBottom: 0.5 * EM,
  },

  h5: {
    display: 'block',
    fontSize: 1.25 * EM,
    lineHeight: 1.2 * 1.25 * EM,
    marginBottom: 0.5 * EM,
  },

  h6: {
    display: 'block',
    fontSize: 1 * EM,
    lineHeight: 1.2 * EM,
    marginBottom: 0.5 * EM,
  },

  button: {
    backgroundColor: '#2196F3',
    paddingHorizontal: 10,
    borderRadius: 2,
    fontSize: 14,
    lineHeight: 32,
    color: '#ffffff',
    textAlign: 'center',
    justifyContent: 'space-around',
  },

  a: {
    color: '#4338ad',
  },

  p: {
    marginBottom: 1 * EM,
  },

  input: {
    lineHeight: 1 * EM,
    padding: 0.5 * EM,
    minHeight: 2 * EM,
  },
}
