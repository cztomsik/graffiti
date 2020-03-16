// node -r ts-node/register -r ./src/register examples/vanilla/anim.ts

const el = document.createElement('div')
el.style.height = '20px'
el.style.backgroundColor = '#f00'

document.body.style.padding = '20px'
document.body.appendChild(el)

let LIMIT = 400
let start, i = 0

const step = () => {
  el.style.width = `${i++}px`

  if (i < LIMIT) {
    requestAnimationFrame(step)
  } else {
    console.log(el.textContent = `Updates per second: ${LIMIT * 1000 / (Date.now() - start)}`)
  }
}

setTimeout(() => {
  start = Date.now()
  requestAnimationFrame(step)
}, 1000)
