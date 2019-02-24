import {
  EnumDeclarationStructure,
  InterfaceDeclarationStructure,
  PropertySignatureStructure,
  StatementedNodeStructure,
  FunctionDeclarationStructure,
  TypeAliasDeclarationStructure,
  CodeBlockWriter,
  ParameterDeclarationStructure
} from 'ts-morph'
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
  VariantT,
  NewTypeDesc
} from './ast'

export const makeFileStructure = (entries: EntryT[]) =>
  entries.reduce<StatementedNodeStructure>(
    (file, entry) =>
      EntryType.match(entry, {
        Struct: (desc): StatementedNodeStructure => ({
          ...file,
          interfaces: (file.interfaces || []).concat(structToInterface(desc))
        }),
        Enum: desc => ({
          ...file,
          enums: (file.enums || []).concat(enumToEnum(desc))
        }),
        Union: desc => ({
          ...file,
          // enums: (file.enums || []).concat(unionToTagEnum(desc)),
          typeAliases: (file.typeAliases || []).concat(
            unionToTaggedUnion(desc)
          ),
          interfaces: (file.interfaces || []).concat(
            unionToPayloadInterfaces(desc)
          ),
          functions: (file.functions || []).concat(unionToContstructors(desc))
        }),
        Tuple: desc => ({
          ...file,
          interfaces: (file.interfaces || []).concat(tupleToInterface(desc)),
          functions: (file.functions || []).concat(tupleToContsructor(desc))
        }),
        Newtype: desc => ({
          ...file,
          typeAliases: (file.typeAliases || []).concat(
            newtypeToTypeAlias(desc)
          ),
          functions: (file.functions || []).concat(newtypeToContsructor(desc))
        })
      }),
    {}
  )

const enumToEnum = ({
  name,
  variants
}: EnumDesc): EnumDeclarationStructure => ({
  name,
  isExported: true,
  members: variants.map(v => ({ name: v }))
})

// const unionToTagEnum = ({
//   name,
//   variants
// }: UnionDesc): EnumDeclarationStructure => ({
//   name: name + 'Tag',
//   isExported: true,
//   members: variants.map(variantName).map(name => ({ name }))
// })

const unionToTaggedUnion = ({
  name,
  variants
}: UnionDesc): TypeAliasDeclarationStructure => ({
  name,
  isExported: true,
  type: (writer: CodeBlockWriter): void => {
    variants.reduce((w, variant) => {
      const valueStr = variantPayload(name, variant)
      return w.writeLine(
        `| { tag: "${variantName(variant)}"${
          valueStr ? `, value: ${valueStr}` : ''
        }}`
      )
    }, writer.newLine())
  }
})

const newtypeToTypeAlias = ({
  name,
  type
}: NewTypeDesc): TypeAliasDeclarationStructure => ({
  name,
  isExported: true,
  type: newtypeToStr(type, name)
})

const newtypeToContsructor = ({
  name,
  type
}: NewTypeDesc): FunctionDeclarationStructure => ({
  name: 'mk' + name,
  isExported: true,
  parameters: [
    {
      name: 'val',
      type: typeToString(type)
    }
  ],
  bodyText: `return val as any`,
  returnType: newtypeToStr(type, name)
})

const unionToPayloadInterfaces = ({
  name: unionName,
  variants
}: UnionDesc): InterfaceDeclarationStructure[] =>
  variants
    .filter(Variant.is.Struct)
    .map(v => v.value)
    .map(({ name, members }) => ({ name: unionName + name, members }))
    .map(structToInterface)

const unionToContstructors = ({
  name: unionName,
  variants
}: UnionDesc): FunctionDeclarationStructure[] =>
  variants.map(v => ({
    name: `mk${unionName}${variantName(v)}`,
    isExported: true,
    parameters: variantToCtorParameters(unionName, v),
    bodyText: `return ${variantToCtorBody(v)};`,
    returnType: unionName
  }))

const variantToCtorParameters = (
  unionName: string,
  v: VariantT
): ParameterDeclarationStructure[] =>
  Variant.match(v, {
    Struct: ({ name }) => [
      { name: 'value', type: structVariantInterfaceName(unionName, name) }
    ],
    Unit: () => [],
    NewType: ({ type }) => [{ name: 'value', type: typeToString(type) }],
    Tuple: ({ fields }) =>
      fields.map((f, i) => ({ name: `p${i}`, type: typeToString(f) }))
  })

const variantToCtorBody = Variant.match({
  Struct: ({ name }) => `{ tag: "${name}", value}`,
  Unit: name => `{ tag: "${name}"}`,
  NewType: ({ name }) => `{ tag: "${name}", value}`,
  Tuple: ({ name, fields }) =>
    `{ tag: "${name}", value: [${fields.map((_, i) => `p${i}`)}]}`
})

const variantPayload = (unionName: string, v: VariantT): string | undefined =>
  Variant.match(v, {
    Struct: ({ name }) => structVariantInterfaceName(unionName, name),
    Unit: () => undefined,
    NewType: ({ type }) => typeToString(type),
    Tuple: ({ fields }) => `[${fields.map(typeToString).join(', ')}]`
  })

const structVariantInterfaceName = (unionName: string, variantName: string) =>
  `${unionName + variantName}`

const structToInterface = ({
  name,
  members
}: StructDesc): InterfaceDeclarationStructure => ({
  name,
  isExported: true,
  properties: Object.keys(members)
    .map(name => ({ name, type: members[name] }))
    .map(
      ({ name, type }): PropertySignatureStructure => ({
        name,
        type: typeToString(type)
      })
    )
})

const tupleToInterface = ({
  name,
  fields
}: TupleDesc): InterfaceDeclarationStructure => ({
  name,
  isExported: true,
  properties: fields
    .map(
      (field, i): PropertySignatureStructure => ({
        name: i.toString(),
        type: typeToString(field)
      })
    )
    .concat({ name: 'length', type: fields.length.toString() })
})

const tupleToContsructor = ({
  name,
  fields
}: TupleDesc): FunctionDeclarationStructure => ({
  name: 'mk' + name,
  isExported: true,
  parameters: fields.map((f, i) => ({
    name: 'p' + i.toString(),
    type: typeToString(f)
  })),
  bodyText: `return [${fields.map((_, i) => 'p' + i.toString()).join(', ')}]`,
  returnType: name
})

const scalarToString = (scalar: Scalar): string => {
  switch (scalar) {
    case Scalar.Bool:
      return 'boolean'
    case Scalar.F32:
      return 'number'
    case Scalar.U32:
      return 'number'
    case Scalar.Str:
      return 'string'
  }
}

const typeToString = (type: Type): string => {
  switch (type.tag) {
    case TypeTag.Option:
      return `(${typeToString(type.value)}) | undefined`
    case TypeTag.Scalar:
      return scalarToString(type.value)
    case TypeTag.Vec:
      return `Array<${typeToString(type.value)}>`
    case TypeTag.RefTo:
      return type.value
  }
}

const variantName = Variant.match({
  Struct: ({ name }) => name,
  Unit: s => s,
  Tuple: ({ name }) => name,
  NewType: ({ name }) => name
})

const newtypeToStr = (type: Type, name: string): string =>
  `${typeToString(type)} & { type: '${name}'}`
