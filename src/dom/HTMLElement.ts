import { Element } from './Element'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'
import { TODO } from '../core/utils'

export class HTMLElement extends Element implements globalThis.HTMLElement {
  style

  _init() {
    this.style = new CSSStyleDeclaration(this.ownerDocument._scene, this._nativeId)
  }

  // TODO: display: none
  get offsetParent(): globalThis.Element | null {
    return this.parentElement
  }

  get offsetLeft(): number {
    const [[left]] = this._bounds

    return left
  }

  get offsetTop(): number {
    const [[, top]] = this._bounds

    return top
  }

  get offsetWidth(): number {
    const [[left], [right]] = this._bounds

    return right - left
  }

  get offsetHeight(): number {
    const [[, top], [, bottom]] = this._bounds

    return bottom - top
  }

  click() {
    TODO()
  }

  accessKey
  accessKeyLabel
  autocapitalize
  autofocus
  contentEditable
  dataset
  dir
  draggable
  hidden
  innerText
  inputMode
  isContentEditable
  lang
  spellcheck
  tabIndex
  title
  translate
}
