import { unionize, ofType as of, UnionOf } from 'unionize'

export const enum Scalar {
  U32 = 'u32',
  F32 = 'f32',
  Str = 'str',
  Bool = 'bool'
}

export type EnumDesc = {
  name: string
  variants: string[]
}

export type UnionDesc = {
  name: string
  variants: VariantT[]
}

export type StructDesc = {
  name: string
  members: { [prop: string]: Type }
}

export type TupleDesc = {
  name: string
  fields: Type[]
}

export type NewTypeDesc = {
  name: string
  type: Type
}

const withTagAndValueProps = { tag: 'tag' as 'tag', value: 'value' as 'value' }

export const EntryType = unionize(
  {
    Struct: of<StructDesc>(),
    Enum: of<EnumDesc>(),
    Tuple: of<TupleDesc>(),
    Newtype: of<NewTypeDesc>(),
    Union: of<UnionDesc>()
  },
  withTagAndValueProps
)

export const Variant = unionize(
  {
    Unit: of<string>(),
    Tuple: of<TupleDesc>(),
    NewType: of<NewTypeDesc>(),
    Struct: of<StructDesc>()
  },
  withTagAndValueProps
)

export const enum TypeTag {
  Scalar = 'Scalar',
  Vec = 'Vec',
  Option = 'Option',
  RefTo = 'RefTo'
}

export type Type =
  | { tag: TypeTag.Scalar; value: Scalar }
  | { tag: TypeTag.Vec; value: Type }
  | { tag: TypeTag.Option; value: Type }
  | { tag: TypeTag.RefTo; value: string }

export const T = {
  Scalar: (value: Scalar): Type => ({ tag: TypeTag.Scalar, value }),
  Vec: (value: Type): Type => ({ tag: TypeTag.Vec, value }),
  Option: (value: Type): Type => ({ tag: TypeTag.Option, value }),
  RefTo: (value: string): Type => ({ tag: TypeTag.RefTo, value })
}

export type EntryT = UnionOf<typeof EntryType>
export type VariantT = UnionOf<typeof Variant>
