// note we only implement subset of the DOM API so it's safer
// to use one of the supported frameworks (react, angular, vue, ...)

const div = document.body.appendChild(document.createElement('div'))
const h1 = div.appendChild(document.createElement('h1'))
const span = div.appendChild(document.createElement('span'))

div.style.padding = '20px'
h1.textContent = 'Hello'

// update periodically
setInterval(() => span.textContent = new Date().toLocaleTimeString(), 1000)
