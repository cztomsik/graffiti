// it's not possible to pass linker args to rustc from either
// rust code, Cargo.toml nor .cargo/config because of various
// unfortunate bugs & edge-cases
//
// we will need linker args for wasm
//
// so... we build the lib from js
//
// another good thing is that we can tell the output filename
// so the node.js require will be a bit easier
//
// and lastly, it's easier to generate typescript here than in
// proc_macro or build.rs (incr. compilation makes it even harder)

console.warn(`
  Hey, thanks for trying out graffiti!
  Some things are still missing, like precompiled binaries
  so if you're running this for the first time, it might take a while
`)

// imports
const os = require('os')
const fs = require('fs')
const child_process = require('child_process')

// args, flags & consts
const platform = os.platform()
const extraArgs = process.argv.slice(2)
const isRelease = extraArgs.includes('--release')
const libFile = platform === 'win32' ? 'graffiti.dll' : `libgraffiti.${platform === 'darwin' ? 'dylib' : 'so'}`
const targetDir = `${__dirname}/libgraffiti/target`

// parse rust & generate interop:
// - ./libgraffiti/src/interop/generated.rs
// - ./src/core/interop.ts
generateInterop([
  ['commons', 'ElementChild', 'Pos', 'Bounds', 'Color'],
  ['app', 'WindowEvent'],
  ['viewport', 'SceneChange', 'Event'],
  ['box_layout', 'Display', 'Overflow', 'Dimension', 'Align', 'FlexWrap', 'FlexDirection'],
  ['text_layout', 'Text', 'TextAlign'],
  ['render', 'Transform', /*'BorderRadius',*/ 'BoxShadow', /*'Border', 'BorderSide', 'BorderStyle'*/],
  ['interop', 'AppMsg', 'AppResponse'],
])

const { status } = child_process.spawnSync('cargo', ['build', ...extraArgs], {
  cwd: `${__dirname}/libgraffiti`,
  stdio: 'inherit',
  shell: true
})

if (status) {
  process.exit(status)
}

fs.copyFileSync(
  `${targetDir}/${isRelease ? 'release' : 'debug'}/${libFile}`,
  `${targetDir}/libgraffiti.node`
)

function generateInterop(mods) {
  const structs = []
  const enums = []
  const taggedUnions = []

  // parse each module & find respective types
  for (const [mod, ...types] of mods) {
    // read & strip comments (easy to fool but whatever)
    const source = fs.readFileSync(`${__dirname}/libgraffiti/src/${mod}.rs`, 'utf-8').replace(/\/\/.*/gm, '').replace(/\/\*.*?\*\//sg,'')

    for (const t of types) {
      // recursive regex is not supported so we try to match to the next token/EOF which should be enough
      const pattern = new RegExp(`(enum|struct)\\s+${t}\\s*{(.*?)}\\s*([\\w#]|$)`, 's')
      const [, kind, body] = source.match(pattern) || err(`Type ${t} not found in ${mod}`)

      //console.log(mod, t, kind, body)

      if (kind === 'struct') {
        // for structs, we only need field names
        structs.push([t, parseAll(body, /(\w+):/g, m => m[1])])
      } else if (kind === 'enum') {
        if (!body.match(/{/)) {
          // for simple enums we need just name & body which can be pasted as is
          enums.push([t, body])
        } else {
          // tagged union, parse variants, fields are enough
          const variants = parseAll(body, /(\w+)\s*(?:{(.*?)}|,|\s*$)/g, m => [
            m[1],
            parseAll(m[2], /(\w+):/g, m => m[1])
          ])
          taggedUnions.push([t, variants])
        }
      }
    }
  }

  write(`${__dirname}/libgraffiti/src/interop/generated.rs`, rustInterop())
  write(`${__dirname}/src/core/interop.ts`, tsInterop())

  function rustInterop() {
    return `// generated

    \n${mods
      .map(([m, ...types]) => `use crate::${m.replace(/\//g, '::').replace('::mod', '')}::{${types}};`)
      .join('\n')}

    \ninterop! {
      \n${taggedUnions
        .map(
          ([name, variants]) =>
            `  ${name} { \n${variants.map(([v, fields], i) => `    ${i} ${v} { ${fields.join(', ')} }`).join(',\n')} \n  }`
        )
        .join('\n')}

      \n${structs.map(([name, fields]) => `  ${name} [${fields}]`).join('\n')}

      \n${enums.map(([name, body]) => `  ${name}(u8)`).join('\n')}
    \n}
    `
  }

  function tsInterop() {
    return `// generated

    \n${enums.map(([name, body]) => `\nexport enum ${name} { ${body.trimEnd()} \n}`).join('')}

    \n${structs.map(([name, fields]) => `export const ${name} = (${fields}) => [${fields}]`).join('\n')}

    \n${taggedUnions
      .map(
        ([name, variants]) => `export module ${name} {
        \n${variants.map(([v, fields], i) => `    export const ${v} = (${fields}) => [${i}, ${fields}]`).join('\n')}

        \n    export const TAGS = {\n${variants.map(([v, fields], i) => `      ${v}: ${i},`).join('\n')}\n    }\n}
    `
      )
      .join('\n')}
  `
  }

  function err(msg) {
    throw new Error(msg)
  }

  function parseAll(str, pattern, mapFn) {
    let m,
      res = []

    pattern.lastIndex = 0

    while ((m = pattern.exec(str))) {
      res.push(mapFn(m))
    }

    return res
  }

  function write(file, str) {
    const prev = fs.readFileSync(file, 'utf-8')

    // because of incr. compilation
    if (str !== prev) {
      fs.writeFileSync(file, str)
    }
  }
}
