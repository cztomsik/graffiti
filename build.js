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

// TODO: release

require('child_process').spawn(
  'cargo',
  [
    'rustc',
    (process.env.NODE_ENV === 'production') ?'--release' :'',
    '--',
    '-Clink-args="-undefined dynamic_lookup"'
  ],
  {
    cwd: `${__dirname}/libgraffiti`,
    stdio: 'inherit',
    shell: true
  }
)
