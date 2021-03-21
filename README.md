---
title: Home
permalink: /
---

**Work in progress**, here's last version which kinda worked on all platforms, including raspi. https://github.com/cztomsik/graffiti/tree/d60a4b75bf0a9fdb67af8fd449f054a411127f38

Follow me on my [twitter](https://twitter.com/cztomsik) for updates.

[Discord](https://discord.gg/zQwyzFb)

---

# graffiti 

HTML/CSS engine for node.js and deno. No electron, no webkit, no servo, all from scratch.

```
npx gft run https://developit.github.io/hn_minimal/
```

## Features and limitations
- low memory footprint (when compared to electron)
- JS libraries generally work fine (react, vue, svelte, ...)
- `<script>` elements are only evaluated during page-load
- CSS-in-JS should work fine, `@import` is not supported
- flexbox only (block is emulated, inline/float is not supported at all)


![react-calculator](https://github.com/cztomsik/graffiti/blob/master/docs/images/react-calculator.png?raw=true)

![hmr](https://github.com/cztomsik/graffiti/blob/master/docs/images/hmr.gif?raw=true)

![hackable-tv](https://user-images.githubusercontent.com/3526922/74057963-4ad47f00-49e5-11ea-9e0d-b39c98f5fe1b.gif)
