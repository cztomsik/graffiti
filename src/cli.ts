// npm run prepare && ./bin/gft.js run test.js
// deno install -A --unstable https://deno.land/x/graffiti/bin/gft.js
// deno run -Ar --unstable lib/cli.js run hello

const USAGE = `Graffiti GUI toolkit

Usage: gft run <script.js>
`

cli('process' in globalThis ? process.argv.slice(2) : globalThis['Deno'].args)

async function cli(args) {
  // should work for both node & deno
  const gft = await import(`${import.meta.url}`.replace('/cli', '/index'))

  if (args.length === 0 || args.includes('--help')) {
    console.log(USAGE)
  } else if (args[0] === 'run') {
    console.log(args)

    console.log(gft)
  } else {
    throw new Error('unknown')
  }
}
