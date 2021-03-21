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

cli('process' in globalThis ? process.argv.slice(2) : globalThis['Deno'].args)

async function cli(args) {
  // should work for both node & deno
  const { App, AppWindow } = await import(`${import.meta.url}`.replace('/cli', '/index'))

  if (args.length === 0 || args.includes('--help')) {
    console.log(USAGE)
  } else if (args[0] === 'run') {
    const app = await App.init()
    const w = new AppWindow()

    await w.loadURL(args[1])

    setInterval(async () => console.log(await w.eval(`JSON.stringify(document.body, ['nodeName', 'data', 'childNodes'], 2)`)), 2000)

    app.run()
  } else {
    throw new Error('Unknown cmd')
  }
}
