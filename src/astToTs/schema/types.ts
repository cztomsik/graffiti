import { unionize, ofType, UnionOf } from 'unionize'

export enum Scalar {
  U32 = 'u32',
  F32 = 'f32',
  Str = 'str',
  Bool = 'bool'
}

export type EnumDesc = {
  name: string
  variants: string[]
}

export type StructDesc = {
  name: string
  members: StructMember[]
}

export type StructMember = {
  name: string
  type: VarT
  optional: boolean
}

export type TupleDesc = {
  name: string
  fields: VarT[]
}

const unionizeProps = { tag: 'tag' as 'tag', value: 'value' as 'value' }

export const EntryType = unionize(
  {
    Struct: ofType<StructDesc>(),
    Enum: ofType<EnumDesc>(),
    Tuple: ofType<TupleDesc>()
  },
  unionizeProps
)

export const VarType = unionize(
  {
    Scalar: ofType<Scalar>(),
    RefToType: ofType<string>() // todo string is enough?
  },
  unionizeProps
)

export type VarT = UnionOf<typeof VarType>
export type EntryT = UnionOf<typeof EntryType>
