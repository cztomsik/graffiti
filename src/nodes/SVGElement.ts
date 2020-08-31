import { Element } from './Element'
import { HTMLElement } from './HTMLElement'
import { CSSStyleDeclaration } from '../css/CSSStyleDeclaration'

export class SVGElement extends Element implements globalThis.SVGElement {
  style = new CSSStyleDeclaration(this, (prop, value) => console.log('TODO: change svg style', prop, value))

  get tagName() {
    return this.localName
  }

  blur = HTMLElement.prototype.blur
  focus = HTMLElement.prototype.focus

  autofocus
  dataset
  correspondingElement
  correspondingUseElement
  ownerSVGElement
  tabIndex
  viewportElement
}
