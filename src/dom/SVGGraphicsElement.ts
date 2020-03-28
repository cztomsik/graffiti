import { SVGElement } from './SVGElement'

export abstract class SVGGraphicsElement extends SVGElement implements globalThis.SVGGraphicsElement {
  getBBox
  getCTM
  getScreenCTM
  requiredExtensions
  systemLanguage
  transform
}
