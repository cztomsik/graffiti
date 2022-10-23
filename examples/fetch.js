import '../graffiti.js'

const url = 'https://jsonplaceholder.typicode.com/todos/1'

document.body.style.display = 'flex'
document.body.style.flexDirection = 'column'
document.body.appendChild(document.createTextNode(`Fetch in 2 seconds`))

setTimeout(async () => {
  console.log(`fetch ${url}`)

  const res = await fetch(url)
  const json = await res.json()

  console.log('ok', json)
  document.body.appendChild(document.createTextNode(JSON.stringify(json)))
}, 2000)
