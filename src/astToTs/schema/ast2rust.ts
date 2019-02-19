import {
  EnumDesc,
  StructDesc,
  Scalar,
  Type,
  EntryT,
  EntryType,
  TupleDesc,
  TypeTag,
  Variant,
  UnionDesc,
  NewTypeDesc
} from './ast'

type Entries = string[]

export const makeRustFile = (entries: EntryT[]): Entries =>
  entries.map(
    EntryType.match({
      Enum: enumToEnum,
      Newtype: newtypeToStruct,
      Tuple: tupleToStruct,
      Struct: structToStruct,
      Union: unionToEnum
    })
  )

const enumToEnum = ({ name, variants }: EnumDesc): string => `
#[derive(Deserialize, Debug)]
pub enum ${name} {
${variants.map(v => `    ${v},`).join('\n')}
}
`

const newtypeToStruct = ({ name, type }: NewTypeDesc): string => `
#[derive(Deserialize, Debug)]
pub struct ${name}(${typeToString(type)});
`

const tupleToStruct = ({ name, fields }: TupleDesc): string => `
#[derive(Deserialize, Debug)]
pub struct ${name}(${fields.map(typeToString).join(', ')});
`
const structToStruct = ({ name, members }: StructDesc): string => `
#[derive(Deserialize, Debug)]
pub struct ${name} {
${members.map(m => `    ${m.name}: ${typeToString(m.type)},`).join('\n')}
}
`

const unionToEnum = ({ name, variants }: UnionDesc): string => `
#[derive(Deserialize, Debug)]
#[serde(tag = "tag", content = "value")]
pub enum ${name} {
${variants.map(v => `    ${variantStr(v)},`).join('\n')}
}
`

const variantStr = Variant.match({
  Unit: name => name,
  NewType: ({ name, type }) => `${name}(${typeToString(type)})`,
  Tuple: ({ name, fields }) =>
    `${name}(${fields.map(typeToString).join(', ')})`,
  Struct: ({ name, members }) =>
    `${name} { ${members
      .map(m => `${m.name}: ${typeToString(m.type)}`)
      .join(', ')} }`
})

const scalarToString = (scalar: Scalar): string => {
  switch (scalar) {
    case Scalar.Bool:
      return 'bool'
    case Scalar.F32:
      return 'f32'
    case Scalar.U32:
      return 'u32'
    case Scalar.Str:
      return 'String'
  }
}

const typeToString = (type: Type): string => {
  switch (type.tag) {
    case TypeTag.Option:
      return `Option<${typeToString(type.value)}>`
    case TypeTag.Scalar:
      return scalarToString(type.value)
    case TypeTag.Vec:
      return `Vec<${typeToString(type.value)}>`
    case TypeTag.RefTo:
      return type.value
  }
}
