import { Project, ScriptTarget } from 'ts-morph'
import { makeFileStructure } from './schema/transformers'
import {
  EntryT,
  EntryType,
  Type,
  Scalar,
  T,
  Variant as V
} from './schema/types'
import * as fs from 'fs'

const testFile = 'src/astToTs/generated/bla.ts'
fs.unlinkSync(testFile)

const project = new Project({
  compilerOptions: {
    target: ScriptTarget.ESNext
  }
})

const { Tuple, Enum, Struct, Union } = EntryType

const entries: EntryT[] = [
  Enum({ name: 'Enum', variants: ['One, Two'] }),
  Tuple({
    name: 'Color',
    fields: [
      T.Option(T.Scalar(Scalar.F32)),
      T.Scalar(Scalar.F32),
      T.Scalar(Scalar.F32)
    ]
  }),
  Struct({
    name: 'Struct',
    members: [
      { name: 'f32', type: T.Scalar(Scalar.F32) },
      { name: 'bool', type: T.Scalar(Scalar.Bool) },
      { name: 'ref', type: T.RefTo('Enum') },
      { name: 'option', type: T.Option(T.Vec(T.Scalar(Scalar.F32))) }
    ]
  }),
  Union({
    name: 'Union',
    variants: [
      V.Unit('VariantUnit'),
      V.NewType({ name: 'VariantNewType', type: T.Scalar(Scalar.F32) }),
      V.Tuple({
        name: 'VariantTuple',
        fields: [T.Vec(T.Scalar(Scalar.Bool)), T.RefTo('Struct')]
      }),
      V.Struct({
        name: 'VariantStruct',
        members: [
          { name: 'optBool', type: T.Option(T.Scalar(Scalar.Bool)) },
          { name: 'color', type: T.RefTo('Color') }
        ]
      })
    ]
  })
]

const sourceFile = project.createSourceFile(
  testFile,
  makeFileStructure(entries)
)

sourceFile.save()

console.log(JSON.stringify(entries))
