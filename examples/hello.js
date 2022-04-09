import { app, AppWindow } from '../lib/index.js'

const win = new AppWindow()
console.log(win)
await win.loadURL(new URL('hello.html', import.meta.url))
console.log('loaded')

console.log(await win.eval('1 + 1'))

console.log(await win.eval('document.documentElement.innerHTML'))

//console.log(await win.eval('err()'))
