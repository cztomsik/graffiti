import { Project, ScriptTarget } from 'ts-morph'
import { makeFileStructure } from './schema/transformers'
import { EntryT, EntryType, VarType, Scalar } from './schema/types'
import * as fs from 'fs'

const testFile = 'src/astToTs/generated/bla.ts'
fs.unlinkSync(testFile)

const project = new Project({
  compilerOptions: {
    target: ScriptTarget.ESNext
  }
})

const { Tuple, Enum, Struct } = EntryType

const entries: EntryT[] = [
  Enum({ name: 'MyEnum', variants: ['One, Two'] }),
  Enum({ name: 'MyEnum2', variants: ['Tree, Four'] }),
  Tuple({
    name: 'Color',
    fields: [
      VarType.Scalar(Scalar.F32),
      VarType.Scalar(Scalar.F32),
      VarType.Scalar(Scalar.F32)
    ]
  }),
  Struct({
    name: 'MyStruct',
    members: [
      { name: 'F32', type: VarType.Scalar(Scalar.F32), optional: false },
      { name: 'Bool', type: VarType.Scalar(Scalar.Bool), optional: true },
      { name: 'RefToEnum', type: VarType.RefToType('MyEnum'), optional: false }
    ]
  })
]

const sourceFile = project.createSourceFile(
  testFile,
  makeFileStructure(entries)
)

sourceFile.save()

console.log(JSON.stringify(entries))
