---
title: Home
nav_order: 1
permalink: /
---

# graffiti

HTML/CSS engine for node.js and deno. No electron, no webkit, no servo, all from scratch.

---

## Current status
- DOM is "done" (in a sense of what I wanted to achieve)
- react/preact/vue is working with this
- nodejs bindings are missing, only deno is working currently
- hit-testing is missing (again)
- rendering is broken
- **layout is WIP** (I am currently replacing yoga with custom impl)
- CSS should be working but it's not currently attached
- incrementality is not done (performance)

---

## Usage

```
# one-shot (takes forever)
npx github:cztomsik/graffiti run <file.html>

# or install locally
npm i github:cztomsik/graffiti
npx graffiti run <file.html>

# or with deno
git clone https://github.com/cztomsik/graffiti
cd graffiti
npm i
deno run -Ar --unstable --no-check lib/cli.js run http://...
```

## Features and limitations
- low memory footprint
- JS libraries generally work fine (react, vue, svelte, ...)
- `<script>` elements are only evaluated during page-load
- CSS-in-JS should work fine, `@import` is not supported
- flexbox only (block is emulated, inline/float is not supported at all)
- no process isolation so it's unsafe to use it as a browser

![react-calculator](https://github.com/cztomsik/graffiti/blob/e7035110f6c6e38fa957871c6df80741690a70b1/docs/images/react-calculator.png?raw=true)

![hmr](https://github.com/cztomsik/graffiti/blob/e7035110f6c6e38fa957871c6df80741690a70b1/docs/images/hmr.gif?raw=true)

![hackable-tv](https://user-images.githubusercontent.com/3526922/74057963-4ad47f00-49e5-11ea-9e0d-b39c98f5fe1b.gif)

## Dev setup
- git clone
- have nodejs 15.x
- have [rust](https://rustup.rs/) in `PATH`
- (linux-only) have X11 headers `apt install xorg-dev`
- `npm i`
- `npm run build && npm run prepare && node lib/cli.js run <html-file>`
