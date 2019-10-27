import { Node } from './Node'
import { Element } from './Element'
import { Text } from './Text'
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

    this.documentElement.style['flexDirection'] = 'column'
    this.documentElement.appendChild(this.body)

    this.parentNode = this.defaultView as unknown as Node
  }

  createElement(tagName: string) {
    const el = new Element(this, tagName, this._scene.createSurface())
    this._els.push(el)

    // apply default styles
    Object.assign(el.style, defaultStyles[tagName] || {})

    return el
  }

  createTextNode(text: string): Text {
    return new Text(this, text)
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

  _getEl(_surface) {
    return this._els[_surface]
  }

  // react-dom listens on the top level
  addEventListener(type, l) {
    this.documentElement.addEventListener(type, l)
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

  body: {
    flexDirection: 'column',
    flex: 1,
  },

  div: {
    display: 'block',
  },

  ul: {
    display: 'block'
  },

  li: {
    display: 'block'
  }
}
