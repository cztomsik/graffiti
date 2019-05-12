export const camelCase = name => name.replace(/\-[a-zA-Z]/g, match => match.slice(1).toUpperCase())

export const mixin = (targetClass, mixinClass) => {
  Object.getOwnPropertyNames(mixinClass.prototype).forEach(name => {
    Object.defineProperty(targetClass.prototype, name, Object.getOwnPropertyDescriptor(mixinClass.prototype, name))
  })
}
