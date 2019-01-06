import { BridgeColor } from './RenderOperation'

export const remove = (arr: any[], item: any) =>
  arr.splice(arr.indexOf(item), 1)

// TODO: rgb(), rgba(), hex short
export const parseColor = (str: string): BridgeColor => {
  return [
    parseHex(str.slice(1, 3)) / 255,
    parseHex(str.slice(3, 5)) / 255,
    parseHex(str.slice(5, 7)) / 255,
    1
  ]
}

export const parseHex = (str: string) => parseInt(str, 16)
