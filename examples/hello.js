import '../graffiti.js'

document.body.style.cssText = 'background-color: #f00; opacity: 0.5'

document.body.appendChild(document.createTextNode('Hello'))

setInterval(() => document.body.appendChild(document.createTextNode('...')), 1000)
