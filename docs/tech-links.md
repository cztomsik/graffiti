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
> -  spatial tree (reference, scroll and sticky frames) <br>
> - clip tree (tree of clip nodes, with a clip chain defined by a path to the tree root)
> - item tree (with nodes being the stacking contexts only, and leafs being items)

Which can be relevant for the project
