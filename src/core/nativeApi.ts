import * as os from 'os';

// require() would make ncc bundle some unnecessary build artifacts
process['dlopen'](module, `${__dirname}/../../libgraffiti/target/libgraffiti.node`)

export const send = (msg) => {
   console.log('send', require('util').inspect(msg, { depth: 4 }))

  // send (sync)
  exports['nativeSend'](msg)

  // TODO: res
  // nodejs extension can throw too so maybe everything could be done there
  return { events: [] }

  /*
  const res = JSON.parse(resBuf.toString('utf-8'))

  if (res.error) {
    throw new Error(res.error)
  }

  return res
  */
}

export const ApiMsg = {
  CreateWindow: (width, height) => [0, width, height],
  GetEvents: (poll) => [1, poll]
}

export const Dimension = {
  UNDEFINED: [0],
  AUTO: [1],
  Points: (points) => [2, points],
  Percent: (percent) => [3, percent],
}
