import { TextAlign, Dimension, StyleProp } from '../core/generated'
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

    this.documentElement._updateStyle({ flex: 1 })
    this.body._updateStyle({ flex: 1 })
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
