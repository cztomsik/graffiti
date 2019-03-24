import { Color } from '.';

export const NOOP = () => undefined
export const IDENTITY = v => v

export const remove = (arr: any[], item: any) =>
  arr.splice(arr.indexOf(item), 1)

// TODO: rgb(), rgba(), hex short
export const parseColor = (str: string): Color => {
  let res = COLOR_CACHE.get(str)

  if (res === undefined) {
    COLOR_CACHE.set(str, res = [
      parseHex(str.slice(1, 3)),
      parseHex(str.slice(3, 5)),
      parseHex(str.slice(5, 7)),
      255
    ])
  }

  return res
}

export const parseHex = (str: string) => parseInt(str, 16)

const COLOR_CACHE = new Map<string, Color>()
