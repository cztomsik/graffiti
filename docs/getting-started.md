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

Low-level api is very simple and follows `serde-json` format for particular `*DisplayItem`s from `webrender::api`. There is no object model, just buckets, representing particular display items, excluding their layout info. Updates are done with `updateBucket(bucket, payload)`. To render, you need to pass buckets along with layout infos. This is mostly because of speed - you can read more on the bottom.

There is also experimental **react binding** which is much more suited for any real UI development. Vue bindings will be added as soon as Vue3 will get published.

```js
const { Window } = require('node-webrender')
const RED = [1, 0, 0, 1]

// create window and bucket with red rectangle
const w = new Window()
const b = w.createBucket({ Rectangle: { color: RED } })

w.render({
  bucket_ids: [b],
  layouts: [[0, 0, 100, 100]]
})
```
