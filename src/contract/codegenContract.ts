import { schema2rust, schema2ts } from 'ts-rust-bridge-codegen'
import * as fs from 'fs'
import { format } from 'prettier'

import { exampleEntries } from './contractAst'

const testFile = __dirname + '/../core/generated.ts'
const testRustFile = __dirname + '/../../native-new/src/generated.rs'

const rustContent = `
use bincode;
use serde::Deserialize;

${schema2rust(exampleEntries).join('\n')}
`

const tsContent = `
${schema2ts(exampleEntries).join('\n\n')}
`

const prettierOptions = JSON.parse(
  fs.readFileSync(__dirname + '/../../.prettierrc').toString()
)

const prettyTsContent = format(tsContent, {
  ...prettierOptions,
  parser: 'typescript'
})

fs.writeFileSync(testRustFile, rustContent)
fs.writeFileSync(testFile, prettyTsContent)
