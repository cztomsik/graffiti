// note that using this (low-level) pseudo-DOM API is discouraged
// you should rather use some existing framework (react, angular, vue, ...)

const containerEl = document.createElement('div')
containerEl.style.padding = '20px'
document.body.appendChild(containerEl)

const headingEl = document.createElement('h1')
containerEl.appendChild(headingEl)
headingEl.textContent = 'Hello'

const timeEl = document.createElement('span')
containerEl.appendChild(timeEl)

// update periodically
setInterval(() => timeEl.textContent = new Date().toLocaleTimeString(), 100)
