---
nav_order: 0
---

# Things to note

- negative dimensions will block forever (WR will not generate a frame)
- fontSize can now be only one of [10, 12, 14, 16, 20, 24, 34, 40, 48]
- mem usage of production release, including node is ~20M + additional libs you use (but you have to build it yourself with `npx neon build --release` inside your `node_modules/node-webrender` directory because it takes forever)
- `examples/react-calculator.tsx` can then run with ~30M if TS is precompiled first (`npx tsc react-calculator.tsx --jsx react -t es2017 -m commonjs && node react-calculator.js`)
- release build is ~10MB on disk (+ node)

## Out of scope
- **cascading** stylesheets
- accessibility
- RTL and vertical languages
- inline layout for components
- word-break, white-space (pre, pre-line, pre-wrap, nowrap)

## TODO
- any font family/size/weight
- support windows (platform)

and other things related to [current milestone](https://github.com/cztomsik/node-webrender/milestones)
