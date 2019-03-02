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
      Alias: aliasToAlias,
      Enum: enumToEnum,
      Newtype: newtypeToStruct,
      Tuple: tupleToStruct,
      Struct: structToStruct,
      Union: unionToEnum
    })
  )

const aliasToAlias = ({ name, type }) => `
pub type ${name} = ${typeToString(type)};
`

const enumToEnum = ({ name, variants }: EnumDesc): string => `
#[derive(Deserialize, Debug, Clone)]
pub enum ${name} {
${variants.map(v => `    ${v},`).join('\n')}
}
`

const newtypeToStruct = ({ name, type }: NewTypeDesc): string => `
#[derive(Deserialize, Debug, Clone)]
pub struct ${name}(pub ${typeToString(type)});
`

const tupleToStruct = ({ name, fields }: TupleDesc): string => `
#[derive(Deserialize, Debug, Clone)]
pub struct ${name}(${fields.map(t => `pub ${typeToString(t)}`).join(', ')});
`
const structToStruct = ({ name, members }: StructDesc): string => `
#[derive(Deserialize, Debug, Clone)]
pub struct ${name} {
${Object.keys(members)
  .map(n => {
    const snakeName = camelToSnakeCase(n)
    const field = `pub ${snakeName}: ${typeToString(members[n])}`
    return snakeName === n
      ? `    ${field},`
      : `    #[serde(rename = "${n}")]\n    ${field},\n`
  })
  .join('\n')}
}
`

const unionToEnum = ({ name, variants }: UnionDesc): string => `
#[derive(Deserialize, Debug, Clone)]
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
    `${name} { ${Object.keys(members)
      .map(n => `${camelToSnakeCase(n)}: ${typeToString(members[n])}`)
      .join(', ')} }`
})

const scalarToString = (scalar: Scalar): string => {
  switch (scalar) {
    case Scalar.Bool:
      return 'bool'
    case Scalar.F32:
      return 'f32'
    case Scalar.U8:
      return 'u8'
    case Scalar.U16:
      return 'u16'
    case Scalar.U32:
      return 'u32'
    case Scalar.USIZE:
      return 'usize'
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

const camelToSnakeCase = (str: string) =>
  str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`)
