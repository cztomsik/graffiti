// it's not possible to pass linker args to rustc from either
// rust code, Cargo.toml nor .cargo/config because of various
// unfortunate bugs & edge-cases
//
// we need linker args because of node.js integration
// and we will need it for wasm too
//
// so... we build the lib from js
//
// another good thing is that we can tell the output filename
// so the node.js require will be a bit easier

console.log(`
  Hey, thanks for trying out graffiti!
  Some things are still missing, like precompiled binaries
  so if you're running this for the first time, it might take a while
`)

const os = require('os')
const fs = require('fs')
const child_process = require('child_process')

const extraArgs = process.argv.slice(2)
const isRelease = extraArgs.includes('--release')
const isWasm = extraArgs.includes('--target') && extraArgs.find(opt => opt.match(/wasm/))
const linkerOpts = isWasm
  ?''
  :(os.platform() === 'darwin')
    ?'-Clink-args="-undefined dynamic_lookup"'
    :'-Clink-args="-undefined=dynamic_lookup"'
const libSuffix = (os.platform() === 'darwin') ?'dylib' :'so'
const targetDir = `${__dirname}/libgraffiti/target`

const { status } = child_process.spawnSync(
  'cargo',
  [
    'rustc',
    ...extraArgs,
    '--',
    linkerOpts
  ],
  {
    cwd: `${__dirname}/libgraffiti`,
    stdio: 'inherit',
    shell: true
  }
)

if (status) {
  process.exit(status)
}

fs.copyFileSync(`${targetDir}/${isRelease ?'release' :'debug'}/libgraffiti.${libSuffix}`, `${targetDir}/libgraffiti.node`)
