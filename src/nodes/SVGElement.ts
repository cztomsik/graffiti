import { Element } from './Element'
import { HTMLElement } from './HTMLElement'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class SVGElement extends Element implements globalThis.SVGElement {
  style = new CSSStyleDeclaration(this)

  get tagName() {
    return this.localName
  }

  blur = HTMLElement.prototype.blur
  focus = HTMLElement.prototype.focus

  autofocus
  className
  dataset
  correspondingElement
  correspondingUseElement
  ownerSVGElement
  tabIndex
  viewportElement
}
