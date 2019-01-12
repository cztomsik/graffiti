---
nav_order: 1
---

# Getting started

## Requirements
- node.js 10.14.1
- [rustc 1.31.1 & cargo 1.31.0](https://rustup.rs/)
- (debian/ubuntu) `sudo apt install g++ cmake pkg-config python libfreetype6 libfreetype6-dev expat libexpat-dev`
- (osx) should work without any deps (but if you need something it should be just one `brew install` away)

## Samples
Easiest way to start is to just clone of these apps:
- https://github.com/cztomsik/slack-app
- https://github.com/cztomsik/brew-cleaner

## Getting started
```bash
npm i node-webrender
```

Note that, there is also experimental **react binding** which is much more suited for any real UI development. Vue bindings will be added as soon as Vue3 will get published.

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
