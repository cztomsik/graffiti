import { BridgeColor } from './RenderOperation'

export const remove = (arr: any[], item: any) =>
  arr.splice(arr.indexOf(item), 1)

// TODO: rgb(), rgba(), hex short
export const parseColor = (str: string): BridgeColor => {
  let res = COLOR_CACHE.get(str)

  if (res === undefined) {
    COLOR_CACHE.set(str, res = [
      parseHex(str.slice(1, 3)) / 255,
      parseHex(str.slice(3, 5)) / 255,
      parseHex(str.slice(5, 7)) / 255,
      1
    ])
  }

  return res
}

export const parseHex = (str: string) => parseInt(str, 16)

const COLOR_CACHE = new Map<string, BridgeColor>()
