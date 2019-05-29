import { TextAlign, Dimension, StyleProp } from '../core/generated'
import { Node } from './Node';
import { Element } from './Element';
import { Text } from './Text';
import { Window } from './Window';

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

    this.body._updateStyle({ flex: 1 })
    this.documentElement.appendChild(this.body)
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
    return new Text(this, text)
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
