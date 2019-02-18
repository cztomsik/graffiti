export type Union =
  | { tag: UnionTag.VariantUnit }
  | { tag: UnionTag.VariantNewType; value: number }
  | { tag: UnionTag.VariantTuple; value: [Array<boolean>, Struct] }
  | { tag: UnionTag.VariantStruct; value: UnionVariantStruct }

export interface Color {
  0: (number) | undefined
  1: number
  2: number
  length: 3
}

export interface Struct {
  f32: number
  bool: boolean
  ref: Enum
  option: (Array<number>) | undefined
}

export interface UnionVariantStruct {
  optBool: (boolean) | undefined
  color: Color
}

export enum Enum {
  One,
  Two
}

export enum UnionTag {
  VariantUnit,
  VariantNewType,
  VariantTuple,
  VariantStruct
}

export function mkColor(
  p0: (number) | undefined,
  p1: number,
  p2: number
): Color {
  return [p0, p1, p2]
}
