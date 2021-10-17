import { Element, HTMLElement } from './index'

export class SVGElement extends Element implements globalThis.SVGElement {
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
