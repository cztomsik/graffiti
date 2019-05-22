import { TextAlign, Dimension } from '../core/generated'
import { Node } from './Node';
import { Element } from './Element';
import { Text } from './Text';
import { Window } from './Window';

export class Document extends Node {
  _scene = this.defaultView.sceneContext
  _els: Element[] = [this as any]

  documentElement = this.createElement('html')
  head = this.createElement('head')
  body = this.createElement('body')

  constructor(public defaultView: Window) {
    super(null, Node.DOCUMENT_NODE, 0)
    this.documentElement.appendChild(this.body)
    this._scene.appendChild(0, this.documentElement._nativeId)
  }

  get parentNode() {
    return this.defaultView
  }

  createElement(tagName: string) {
    let SpecificElement = Element

    //switch (tagName) {
    //  case 'span':
    //    SpecificElement = HTMLSpanElement
    //}

    const el = new SpecificElement(this, tagName, this._scene.createSurface())
    this._els.push(el)

    return el
  }

  createTextNode(text: string): Text {
    return new Text(text)
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

  _getParent(_nativeId) {
    return this._getEl(this._scene.parents[_nativeId])
  }

  _setParent(_nativeId, _parentId) {
    this._scene.parents[_nativeId] = _parentId
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
