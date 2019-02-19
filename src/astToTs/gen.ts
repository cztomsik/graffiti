import { Project, ScriptTarget } from 'ts-morph'
import { makeFileStructure } from './schema/ast2ts'
import { makeRustFile } from './schema/ast2rust'
import { exampleEntries } from './exampleAst'
import * as fs from 'fs'

const testFile = 'src/astToTs/generated/example.ts'
const testRustFile = 'src/astToTs/generated/example.rs'
fs.unlinkSync(testFile)

const project = new Project({
  compilerOptions: {
    target: ScriptTarget.ESNext
  }
})

const sourceFile = project.createSourceFile(
  testFile,
  makeFileStructure(exampleEntries)
)

sourceFile.save()

const rustContent = `
use bincode;
use serde::Deserialize;

${makeRustFile(exampleEntries).join('\n')}
`

fs.writeFileSync(testRustFile, rustContent)

console.log('\n\n', JSON.stringify(exampleEntries))
