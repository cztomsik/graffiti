import {
  EnumDeclarationStructure,
  InterfaceDeclarationStructure,
  PropertySignatureStructure,
  StatementedNodeStructure,
  FunctionDeclarationStructure
} from 'ts-morph'
import {
  EnumDesc,
  StructDesc,
  Scalar,
  VarType,
  EntryT,
  EntryType,
  TupleDesc
} from './types'

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
        Tuple: desc => ({
          ...file,
          interfaces: (file.interfaces || []).concat(tupleToInterface(desc)),
          functions: (file.functions || []).concat(tupleToContsructor(desc))
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

// EnumDeclarationStructure;

const structToInterface = ({
  name,
  members
}: StructDesc): InterfaceDeclarationStructure => ({
  name,
  isExported: true,
  properties: members.map(
    ({ name, optional, type }): PropertySignatureStructure => ({
      name,
      hasQuestionToken: optional,
      type: varTypeToString(type)
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
        type: VarType.match(field, {
          Scalar: scalarToString,
          RefToType: s => s
        })
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
    type: varTypeToString(f)
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

const varTypeToString = VarType.match({
  Scalar: scalarToString,
  RefToType: s => s
})
