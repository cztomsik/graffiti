import { Color } from '.';

export const NOOP = () => undefined
export const IDENTITY = v => v

// TODO: rgb(), rgba()
// https://docs.rs/crate/cssparser/0.25.3/source/src/color.rs
export const parseColor = (str: string): Color => {
  if (str[0] === '#') {
    return parseHash(str.slice(1))
  }

  throw new Error('only colors starting with # are supported')
}

export const parseHash = (str: string): Color => {
  let alpha = 255

  switch (str.length) {
    // rgba
    case 8:
      alpha = parseHex(str.slice(7, 9))

    case 6:
      return [
        parseHex(str.slice(0, 2)),
        parseHex(str.slice(2, 4)),
        parseHex(str.slice(4, 6)),
        alpha
      ]

    // short alpha
    case 4:
      alpha = parseHex(str.slice(3, 4)) * 17

    // short
    case 3:
      return [
        parseHex(str.slice(0, 1)) * 17,
        parseHex(str.slice(1, 2)) * 17,
        parseHex(str.slice(2, 3)) * 17,
        alpha
      ]

    default:
      throw new Error(`invalid color #${str}`)
  }
}

export const parseHex = (str: string) => parseInt(str, 16)

const COLOR_CACHE = new Map<string, Color>()
