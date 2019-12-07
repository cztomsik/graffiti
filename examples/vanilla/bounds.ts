// node -r ts-node/register -r ./src/register examples/vanilla/bounds.ts

const h1 = document.createElement('h1')
h1.style.backgroundColor = '#ccc'
document.body.appendChild(h1)

const loop = () => {
  const n = Date.now() / 1000

  document.body.style.paddingLeft = 100 + (Math.sin(n) * 100) + 'px'
  document.body.style.paddingTop = 100 + (Math.cos(n) * 100) + 'px'

  const { x, y } = h1.getBoundingClientRect()
  h1.textContent = `Pos ${x}, ${y}`

  requestAnimationFrame(loop)
}

loop()
