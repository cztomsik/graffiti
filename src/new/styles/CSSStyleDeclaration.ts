import { camelCase } from "../utils";

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
