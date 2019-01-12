---
nav_order: 100
---

# Compilation of tech articles, videos and links

For internal development purposes.

## Webrenderer

Curated content about WR

---

### The whole web at maximum FPS: How WebRender gets rid of jank

Amazing intro to Webrenderer by Lin Clark.

https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/

---

### Entering the Quantum Era—How Firefox got fast again and where it’s going to get faster

How Webrenderer fits into Firefox by Lin Clark

https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/

---

### Nicolas Silva — WebRender: a sneak peek into Firefox Quantum's future (Rust Hungary #2, 2017-10-24)

Useful for understanding WR architecture.

https://youtu.be/oXF-uyPIKcc?t=485

---

### Meeting notes from Mozlando 2018.

https://github.com/servo/webrender/wiki/2018-Mozlando

Quote from WR API roadmap:

> Rewrite WR API to accept 3 trees instead of a single display list:
>
> - spatial tree (reference, scroll and sticky frames) <br>
> - clip tree (tree of clip nodes, with a clip chain defined by a path to the tree root)
> - item tree (with nodes being the stacking contexts only, and leafs being items)

Which can be relevant for the project

### Other projects/repos related to webrender
- [servo](https://github.com/servo/servo)
- [azul](https://github.com/maps4print/azul)
- [limn](https://github.com/christolliday/limn)
- [stylish](https://github.com/Thinkofname/stylish)

## ReactNative

There is an effort within the react native community that aims to achieve several goals:

1. Make react-native core smaller in terms of number of components
2. move to `import {View} from 'react-native'`. Which can potentially be used with other bundlers.
3. Simplify architecture 3 threads -> 2 threads model with sync interactions. aka React Fabric
4. Make other platforms 1st class citizens (windows + macOS + web?). That involves rewriting a lot of native code in c++

A combination of all of these ideas might make react-native a potentially appealing direction to explore once all these pieces are in place. Or alternatively use this project as a basis for a react-native platform implementation.

**NOTE: currently there is no plans to implement react-native backend within this project.**

Relevant links:

---

### Make the RN core lean

https://github.com/react-native-community/discussions-and-proposals/issues/6

---

### Cross plat strategy for react-primitives

discussion about the future of react-primitives

https://github.com/lelandrichardson/react-primitives/issues/54

---

### Supporting Third-party Platforms in RNPM + Metro Bundler

discussion about supporting 3rd party platforms as 1st class

https://github.com/react-native-community/discussions-and-proposals/issues/21

---

### Unify RN platforms issue

discussion about the way to handle 3rd party platforms like windows. Also handling API extensions (like adding the mouse support)

https://github.com/react-native-community/discussions-and-proposals/issues/50
