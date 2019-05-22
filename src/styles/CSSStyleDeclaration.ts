import { StylePropertyMap, CSSStyleValue } from './CSS'
import { UNSUPPORTED, kebabCase } from '../core/utils'

export class CSSStyleDeclaration {
  constructor(private styleMap: StylePropertyMap) {
    return new Proxy(this, {
      get(target, prop) {
        return target.getPropertyValue(kebabCase(prop))
      },

      set(target, prop, value) {
        target.setProperty(kebabCase(prop), value)
        return true
      },

      deleteProperty(target, prop) {
        target.removeProperty(kebabCase(prop))
        return true
      }
    })
  }

  getPropertyValue(property: string) {
    return UNSUPPORTED()
    //return '' + this.styleMap.get(property)
  }

  setProperty(property: string, value: string, _priority: string = '') {
    if (value === undefined) {
      return
    }

    if (!value) {
      return this.removeProperty(property)
    }

    this.styleMap.set(property, CSSStyleValue.parse(property, value))
  }

  removeProperty(property: string) {
    this.styleMap.delete(property)
  }
}
