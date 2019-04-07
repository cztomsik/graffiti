import {
  schema2rust,
  schema2ts,
  ast2ts,
  schema2serializers
} from 'ts-rust-bridge-codegen'
import * as fs from 'fs'
import { format } from 'prettier'

import { exampleEntries as contract } from './contractAst'

const typescriptDefFile = __dirname + '/../core/generated.ts'
const typescriptSerFile = __dirname + '/../core/serialization.generated.ts'
const rustDefFil = __dirname + '/../../native-new/src/generated.rs'

const rustContent = `
use bincode;
use serde::{Serialize, Deserialize};

${schema2rust(contract).join('\n')}
`

const tsContent = `
${schema2ts(contract).join('\n\n')}
`

const tsSerContent = `
${ast2ts(
  schema2serializers({
    entries: contract,
    typesDeclarationFile: `./generated`,
    pathToBincodeLib: 'ts-rust-bridge-bincode'
  })
).join('\n\n')}
`

const prettierOptions = JSON.parse(
  fs.readFileSync(__dirname + '/../../.prettierrc').toString()
)

const pretty = (content: string) =>
  format(content, {
    ...prettierOptions,
    parser: 'typescript'
  })

fs.writeFileSync(rustDefFil, rustContent)
fs.writeFileSync(typescriptDefFile, pretty(tsContent))
fs.writeFileSync(typescriptSerFile, pretty(tsSerContent))
