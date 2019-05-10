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

const camelCase = name => name.replace(/\-[a-zA-Z]/g, match => match.slice(1).toUpperCase())
