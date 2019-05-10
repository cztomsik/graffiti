import { SceneContext } from '../core'
import { TextAlign, Dimension } from '../core/generated'
import { EventTarget } from './events/EventTarget'
import { CSSStyleDeclaration } from './styles/CSSStyleDeclaration';
import { compileFlatStyle } from '../react/Stylesheet';

class Node extends EventTarget {
  readonly childNodes: Node[] = []

  constructor(public readonly ownerDocument: Document, public readonly nodeType, public readonly _nativeId) {
    super()
  }

  appendChild(child: Node) {
    this.insertBefore(child, null)
  }

  insertBefore(child: Node, before: Node | null) {
    const index = before === null ?this.childNodes.length :this.childNodes.indexOf(child)

    // consider if it's worth to throw like browsers do
    if (~index) {
      this.insertAt(child, index)
    }
  }

  insertAt(child: Node, index) {
    child.remove()

    // TODO: native.insertAt? (storage is dense anyway)
    if (index === this.childNodes.length) {
      this.ownerDocument._scene.appendChild(this._nativeId, child._nativeId)
    } else {
      this.ownerDocument._scene.insertBefore(this._nativeId, child._nativeId, this.childNodes[index]._nativeId)
    }

    this.childNodes.splice(index, 0, child)
  }

  remove() {
    const parent = this.parentNode

    if (parent) {
      parent.removeChild(this)
    }
  }

  removeChild(child: Node) {
    const index = this.childNodes.indexOf(child)

    // throw?
    if (~index) {
      this.childNodes.splice(index, 1)
      this.ownerDocument._scene.removeChild(this._nativeId, child._nativeId)
    }
  }

  replaceChild(child: Node, prev: Node) {
    const index = this.childNodes.indexOf(prev)

    if (~index) {
      this.insertAt(child, index)
      this.removeChild(prev)
    }
  }

  // really defined on Node.prototype

  get firstChild() {
    return this.childNodes[0]
  }

  get lastChild() {
    const chs = this.childNodes

    return chs[chs.length]
  }

  get parentNode() {
    return this.parentElement
  }

  get parentElement() {
    return this.ownerDocument._getParent(this._nativeId)
  }

  get nextSibling() {
    const parentChildren = this.parentElement.childNodes

    return parentChildren[parentChildren.indexOf(this) + 1]
  }

  get previousSibling() {
    const parentChildren = this.parentElement.childNodes

    return parentChildren[parentChildren.indexOf(this) - 1]
  }

  // TODO: get/set nodeValue
  get nodeName() {
    const node = this as any

    switch (this.nodeType) {
      case Node.ELEMENT_NODE: return node.tagName
      case Node.DOCUMENT_NODE: return '#document'
      case Node.TEXT_NODE: return '#text'
    }
  }

  static ELEMENT_NODE = 1
  static TEXT_NODE = 3
  static DOCUMENT_NODE = 9
}

export class Document extends Node {
  _els: Element[] = [,]

  documentElement = this.createElement('html')
  head = this.createElement('head')
  body = this.createElement('body')

  constructor(public _scene: SceneContext) {
    super(null, Node.DOCUMENT_NODE, 0)
    this.documentElement.appendChild(this.body)
    this._scene.appendChild(0, this.documentElement._nativeId)
  }

  createElement(tagName: string) {
    let SpecificElement = Element

    switch (tagName) {
      case 'span':
        SpecificElement = SpanElement
    }

    const el = new SpecificElement(this, tagName, this._scene.createSurface())
    this._els.push(el)

    // or not, because it will cause text width bug
    // flex: 1 for now
    //this._scene.setFlex(el._nativeId, {
    //  flexGrow: 1, flexShrink: 1, flexBasis: Dimension.Percent(0)
    //})

    return el
  }

  createTextNode(text: string): TextNode {
    return new TextNode(text)
  }

  _getEl(_nativeId) {
    return this._els[_nativeId]
  }

  _getParent(_nativeId) {
    return this._getEl(this._scene.parents[_nativeId])
  }

  _setParent(_nativeId, _parentId) {
    this._scene.parents[_nativeId] = _parentId
  }
}

class Element extends Node {
  _style = new CSSStyleDeclaration()

  constructor(public ownerDocument, public tagName, public _nativeId) {
    super(ownerDocument, Node.ELEMENT_NODE, _nativeId)
  }

  // bubble events
  _getTheParent() {
    return this.parentElement
  }

  get style() {
    return new Proxy(this._style, {
      set: (target, property, value) => {
        requestAnimationFrame(() => {
          const { flex, padding, backgroundColor } = compileFlatStyle(this._style as any)

          this.ownerDocument._scene.setFlex(this._nativeId, flex)
          this.ownerDocument._scene.setPadding(this._nativeId, padding)
          this.ownerDocument._scene.setBackgroundColor(this._nativeId, backgroundColor)
        })

        return Reflect.set(target, property, value)
      }
    })
  }
}

class SpanElement extends Element {
  // only text nodes
  appendChild(child: /* TextNode */ any) {
    this.removeChild(child)
    this.childNodes.push(child)
    child.parentNode = this
    this.updateText()
  }

  removeChild(child) {
    const index = this.childNodes.indexOf(child)

    if (~index) {
      this.childNodes.splice(index, 1)
    }
  }

  updateText() {
    this.ownerDocument._scene.setText(this._nativeId, {
      color: [0, 0, 0, 255],
      fontSize: 16,
      lineHeight: 30,
      align: TextAlign.Left,
      text: this.childNodes.map(tn => tn['data']).join('')
    })
  }
}

class TextNode {
  _data
  parentNode?

  constructor(data) {
    this._data = data
  }

  get data() {
    return this._data
  }

  set data(text) {
    this._data = text

    if (this.parentNode) {
      this.parentNode.updateText()
    }
  }
}

