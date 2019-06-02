---
nav_order: 96
---
# Design decisions
This is rather internal (for me and contributors) page to document decisions what has already been made
(and why).

## javascript
- community, libs
- dynamic lang
- best tooling I know
- HMR

## webrender
- well-tested (used in firefox) across many platforms & hw combinations
- easier to use when compared to skia
  - scrolling is already provided
  - no need to speculate about layers, etc.
  - just render everything each frame (if there's a need for a frame)
- in future, pathfinder might be also interesting
  - used in webrender for some of the work
  - doesn't handle scrolling but it's much lighter
  - has CPU rasterization built-in
  - WebGL target

## core in rust
- javascript is not a good choice for anything perf sensitive, it's hard & also a moving target
- fast, safe, no dependency hell
- webrender is also written in rust
