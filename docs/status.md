---
nav_order: 0
---

# Project status
We are currently doing major rewrite after which we will mark it as 1.0 release. After that it might be considered good enough for some simple apps.

## Known issues
- negative dimensions will block forever (WR will not generate a frame)
- font size can now be only one of [10, 12, 14, 16, 20, 24, 34, 40, 48]
- only `Arial` font is supported yet (no matter what font family you define)
- only `normal` font weight is supported yet
- windows platform is not supported yet

## Performance
At this point, performance is not a priority nor a current goal.

But when it gets closer to v2, it should be way faster than comparable electron app simply because we don't have to support various web features accumulated over the 30 ys history and because we also make some shortcuts where it makes sense (for desktop apps).
