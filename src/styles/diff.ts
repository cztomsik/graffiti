import { StyleProp } from '../core/generated'
import resolvers from './resolvers'

// prop miss can be expensive so instead of blindly resolving the whole style we first
// find out what's different, compute "damage" and then return StyleProp(s) to be updated
//
// memory-wise it should be fine
// even for 1k of nodes, it shouldn't take more than let's say 100kB
//
// BTW: resolving everything would also generate a lot of new objects which couldn't
// be compared by reference

export function diffStyle(newStyle, prevStyle): StyleProp[] {
  // TODO: measure, try bitflags
  const damage = new Set()

  // remove missing props
  for (const k in prevStyle) {
    if (!(k in newStyle)) {
      mark(k)
    }
  }

  for (const k in newStyle) {
    if (newStyle[k] !== prevStyle[k]) {
      mark(k)
    }
  }

  return Array.from(damage).map(type => resolvers[type](newStyle))

  function mark(prop) {
    damage.add(propDamage(prop))
  }
}

// TODO: refactor; babel-macro?
function propDamage(prop) {
  switch (prop) {
    case 'width':
    case 'height':
      return 'Size'

    case 'flex':
    case 'flexGrow':
    case 'flexShrink':
    case 'flexBasis':
      return 'Flex'

    case 'flexDirection':
    case 'flexWrap':
    case 'alignContent':
    case 'alignItems':
    case 'alignSelf':
    case 'justifyContent':
      return 'Flow'

    case 'content':
    case 'color':
    case 'fontSize':
    case 'lineHeight':
    case 'textAlign':
      return 'Text'
  }

  if (prop.startsWith('padding')) {
    return 'Padding'
  }

  if (prop.startsWith('margin')) {
    return 'Margin'
  }

  if (prop.endsWith('Radius')) {
    return 'BorderRadius'
  }

  if (prop.startsWith('border')) {
    return 'Border'
  }

  if (prop.startsWith('shadow')) {
    return 'BoxShadow'
  }

  if (prop === 'backgroundColor') {
    return 'BackgroundColor'
  }

  if (prop === 'backgroundImageUrl') {
    return 'Image'
  }

  if (prop === 'overflow') {
    return 'Overflow'
  }

  console.log('no damage', prop)
}
