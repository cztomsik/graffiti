---
nav_order: 90
---

# Performance

At this point, performance is not a priority nor a current goal.

But when it gets closer to v2, it should be way faster than comparable electron-app simply because we don't have to support various web features accumulated over the 30 ys history.

Specifically, there is no DOM, no stylesheet cascading, no event bubbling and the layout is flexbox-only, so we can get to drawing really quickly. Most of these higher-level concepts are left to libs/frameworks of your choice.
