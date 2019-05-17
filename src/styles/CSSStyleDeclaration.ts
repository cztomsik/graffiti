import { camelCase, mixin } from "../utils";

export class CSSStyleDeclaration {
  getPropertyValue(name) {
    return this[camelCase(name)]
  }

  setProperty(name, value, _priority) {
    this[camelCase(name)] = value
  }

  removeProperty(name) {
    delete this[camelCase(name)]
  }
}

/*
  WIP

  for now, we only define setters and see how far it will get us

  shorthands seem to only work one-way in chrome
    - style.margin = '1em'
    - style.marginLeft = '2em'
    - style.margin === '1em'

  sometimes they even seemingly unset the property (but it remains effective)
    - style.border = '1px solid'
    - style.borderLeft = '2px solid'
    - style.border === ''
*/
class Shorthands {
  set padding(p) {
    this['paddingTop'] = this['paddingRight'] = this['paddingBottom'] = this['paddingLeft'] = p
  }

  set margin(p) {
    this['marginTop'] = this['marginRight'] = this['marginBottom'] = this['marginLeft'] = p
  }
}

mixin(CSSStyleDeclaration, Shorthands)
