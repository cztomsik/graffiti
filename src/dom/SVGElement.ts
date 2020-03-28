import { Element } from './Element'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class SVGElement extends Element implements globalThis.SVGElement {
  style = new CSSStyleDeclaration(this)

  autofocus
  className
  dataset
  correspondingElement
  correspondingUseElement
  ownerSVGElement
  tabIndex
  viewportElement
}
