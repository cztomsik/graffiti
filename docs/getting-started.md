---
nav_order: 1
---

# Getting started

## Requirements
- node.js 10.14.1
- there are no prebuilt binaries yet so you might need additional tooling in order to build native extension (see below)
- install rustc 1.31.1 & cargo 1.31.0 [with rustup](https://rustup.rs/)
  - check you have `rustfmt` installed (or do `rustup component add rustfmt`)

### Debian/Ubuntu
`sudo apt install g++ cmake pkg-config python libfreetype6 libfreetype6-dev expat libexpat-dev`

### OSX/MacOs
`xcode-select --install`
`brew install cmake pango`

### Win
- [track here](https://github.com/cztomsik/node-webrender/issues/37) but it's a bit like never-ending story so any help would be very welcome.

## Starters
Easiest way to start is to clone one of these repos. Note that you will need to build native extension too so the steps above still apply.
- https://github.com/cztomsik/node-webrender-starter (empty)
- https://github.com/cztomsik/slack-app
- https://github.com/cztomsik/brew-cleaner

## Getting started
```bash
npm i node-webrender
```

Note that, there is also experimental **[react binding](./react.md)** which is much more suited for any real UI development. Vue bindings will be added as soon as Vue3 will get published.

Low-level api is mostly about `ResourceManager` and `Surface` classes. Surface is a container with optional `brush`, `clip` and `layout`.

```js
import { Window, ResourceManager, Surface } from '../src'

const w = new Window("Hello")

const brush = ResourceManager.getBrush({
  backgroundColor: '#ff0000'
})

const layout = ResourceManager.getLayout({
  flex: 1,
  margin: 20
})

const rect = new Surface()
rect.update({ brush, layout })

w.appendChild(rect)
w.render()
```
