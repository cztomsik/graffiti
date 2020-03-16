import { Element } from './Element'
import { CSSStyleDeclaration } from '../styles/CSSStyleDeclaration'

export class HTMLElement extends Element {
  style = new CSSStyleDeclaration(this.ownerDocument._scene, this._nativeId)
}
