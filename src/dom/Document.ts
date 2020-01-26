import { Node } from './Node'
import { Element } from './Element'
import { Text } from './Text'
import { Comment } from './Comment'
import { Window } from './Window'
import { DocumentFragment } from './DocumentFragment'

export class Document extends Node {
  _scene = this.defaultView.sceneContext
  _els: Element[] = [new Element(this, 'html', 0)]

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

  constructor(public defaultView: Window) {
    super(null, Node.DOCUMENT_NODE, 0)

    this.documentElement.style.setProperty('min-height', '100%')
    this.documentElement.parentNode = this
    this.documentElement.appendChild(this.body)

    this.parentNode = this.defaultView as unknown as Node
  }

  createElement(tagName: string) {
    const el = new Element(this, tagName, this._scene.createElement())
    this._els.push(el)

    // apply default styles
    Object.assign(el.style, defaultStyles[tagName] || {})

    // TODO: consider instantiating some subclass so
    // that it's for example possible to open links in native browser, etc.

    return el
  }

  createTextNode(text: string): Text {
    return new Text(this, text, this._scene.createText())
  }

  createComment(data: string): Comment {
    // empty text node for now (should be fine)
    return new Comment(this, data, this._scene.createText())
  }

  createDocumentFragment() {
    return new DocumentFragment(this, Node.DOCUMENT_FRAGMENT_NODE, undefined)
  }

  getElementById(id) {
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
  h1: {
    fontSize: 2.5 * EM,
    marginBottom: 0.5 * EM,
  },

  h2: {
    fontSize: 2 * EM,
    marginBottom: 0.5 * EM,
  },

  h3: {
    fontSize: 1.75 * EM,
    marginBottom: 0.5 * EM,
  },

  h4: {
    fontSize: 1.5 * EM,
    marginBottom: 0.5 * EM,
  },

  h5: {
    fontSize: 1.25 * EM,
    marginBottom: 0.5 * EM,
  },

  h6: {
    fontSize: 1 * EM,
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
}
