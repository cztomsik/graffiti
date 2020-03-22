export const NOOP = () => undefined
export const IDENTITY = v => v
export const TODO = () => ERR('TODO')
export const UNSUPPORTED = () => ERR('UNSUPPORTED')
export const UNREACHABLE = () => ERR('UNREACHABLE')
export const ERR = (...msgs) => {
  throw new Error(msgs.join(' '))
}

export const EMPTY_OBJ = Object.freeze({})

export const last = arr => arr[arr.length - 1]

export const camelCase = name => name.replace(/\-[a-zA-Z]/g, match => match.slice(1).toUpperCase())
export const kebabCase = name => name.replace(/[A-Z]/g, match => '-' + match.toLowerCase())
export const pascalCase = name => ((name = camelCase(name)), name[0].toUpperCase() + name.slice(1))

export const mixin = (targetClass, mixinClass) => {
  Object.getOwnPropertyNames(mixinClass.prototype).forEach(name => {
    Object.defineProperty(targetClass.prototype, name, Object.getOwnPropertyDescriptor(mixinClass.prototype, name))
  })
}
