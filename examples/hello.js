import '../graffiti.js'

document.body.style.cssText = 'background-color: #f00; opacity: 0.5'

document.body.appendChild(document.createTextNode('Hello'))

// TODO: if we stop tick()ing after close then this would still keep
//       nodejs alive, which means app is running but we are not consuming events anymore -> crash
//setInterval(() => document.body.appendChild(document.createTextNode('...')), 1000)
