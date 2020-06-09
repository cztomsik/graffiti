// build native *.node extension

console.warn(`
  Hey, thanks for trying out graffiti!
  Some things are still missing, like precompiled binaries
  so if you're running this for the first time, it might take a while
`)

// args, flags & consts
const platform = require('os').platform()
const extraArgs = process.argv.slice(2)
const isRelease = extraArgs.includes('--release')
const libFile = platform === 'win32' ? 'graffiti.dll' : `libgraffiti.${platform === 'darwin' ? 'dylib' : 'so'}`
const targetDir = `${__dirname}/libgraffiti/target`

// run `cargo build ...`
const { status } = require('child_process').spawnSync('cargo', ['build', ...extraArgs], {
  cwd: `${__dirname}/libgraffiti`,
  stdio: 'inherit',
  shell: true
})

if (status) {
  process.exit(status)
}

// copy result to expected location
require('fs').copyFileSync(
  `${targetDir}/${isRelease ? 'release' : 'debug'}/${libFile}`,
  `${targetDir}/libgraffiti.node`
)
