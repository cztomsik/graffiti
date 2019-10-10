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

const os = require('os')
const fs = require('fs')
const child_process = require('child_process')

const extraArgs = process.argv.slice(2)
const linkerOpts = (os.platform() === 'darwin')
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

fs.copyFileSync(`${targetDir}/${extraArgs.includes('--release') ?'release' :'debug'}/libgraffiti.${libSuffix}`, `${targetDir}/libgraffiti.node`)
