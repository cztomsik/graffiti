// ./bin/gft.js run test.js
// deno install -A --unstable https://deno.land/x/graffiti/bin/gft.js
// deno run -Ar --unstable lib/cli.js run hello

// TODO: docopt?
const USAGE = `Graffiti HTML/CSS engine

Usage:
  gft run <index>

Arguments:
  <index>  HTML file or URL
`

const CWD = globalThis.Deno?.cwd() ?? globalThis.process?.cwd()

cli('process' in globalThis ? process.argv.slice(2) : globalThis['Deno'].args)

async function cli(args) {
  // should work for both node & deno
  const { App, AppWindow } = await import(`${import.meta.url}`.replace('/cli', '/index'))

  if (args.length === 0 || args.includes('--help')) {
    console.log(USAGE)
  } else if (args[0] === 'run') {
    const app = await App.init()
    app.run()

    const w = new AppWindow()

    await w.loadURL(new URL(args[1], new URL(`file://${CWD}/`)))
  } else {
    throw new Error('Unknown cmd')
  }
}
