! WIP, do not use

# node-webrender
use [webrender](https://github.com/servo/webrender) from node.js

- have rust installed
- `git clone --recursive`
- go into
- run `npm run build`
- run `npm run example`


## TODO

- [x] call rust from node
- [x] show window
- [x] init webrender
- [x] clear screen
- [x] show rect
- [x] react example
- [x] load font
- [ ] yoga
- [ ] scrolling
- [ ] window resize
- [ ] text
- [ ] word-wrap
- [ ] click
- [ ] input
- [ ] hover
- [ ] more fonts

## Notes
- release build `npx neon build --release` weights ~7MB, dev is ~30MB

## How does it work (internally)
- overview
  - webrender is about drawing as many rects and glyphs as possible (in parallel) using GPU
  - conceptually, every frame has to be drawn from scratch (in correct order), yet it's faster than usual approach
  - drawing UI on GPU is very different from classic approach, we implement few shader programs and then we "just" fill big buffers of floats
  - but everything has to be prepared and well-thought in advance (it's really hard work, not to mention there are bugs in GPU drivers, multiplied by platforms and versions you support, etc.)
  - webrender does this for us and provides an api

- webrender api
  - webrender retains some info about the scene so it can do scrolling/zooming and hitbox testing for us (without rebuilding the scene), so it is not entirely stateless but I wouldn't call it retained-mode either (we don't directly manage any instances of anything)
  - api is useful for font-loading, text measurements, font glyph resolving but also for sending **transactions**
  - nothing is rendered unless transaction is sent (and async **processed**, which is when we swap buffers, etc.)
  - even scrolling/zooming has to be sent in trasaction
  - transaction can also "set" **display list** which is how scene will get rebuild and re-rendered eventually
  - display list is a binary blob of display items and to make one, it's easiest to use a **builder** (provided by webrender) which has methods like push_rect() and similar
  - all of **this is done in rust**, to avoid crossing boundaries (communication with JS should be kept at minimum)

- layout
  - knowing where to put rects and glyphs on the screen is hard, so you want to use some existing layout system (like flexbox)
  - yoga-layout is quite efficient at (re)computing flexbox
  - layout items are usually dependent on each other so it's necessary to recompute the whole layout on **any dimension change** (incl. text changes) but preferrably only once per UI change (defer/batch)

- structure & dimensions
  - so usually, we know what to render (rects, borders, shadows) before we know the **dimensions**
  - dimensions change a lot, more often than the overall **structure** so if we split them, we can quickly generate frames with just minor changes
  - this also makes it possible to use memoization in react and other UI frameworks
