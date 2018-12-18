const initDevtools = reconciler => {
  let WebSocket, devtools
  try {
    WebSocket = require('ws')
    global['window'] = global
    devtools = require('react-devtools-core')
  } catch (e) {}

  if (devtools) {
    require('react-devtools-core').connectToDevTools({
      websocket: new WebSocket('ws://localhost:8097')
    })

    reconciler.injectIntoDevTools({
      bundleType: 1,
      version: '0.0.0',
      rendererPackageName: 'node-webrender',
      //findFiberByHostInstance: reconciler.findHostInstance
    })
  }
}

export default initDevtools
