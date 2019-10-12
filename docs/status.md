---
nav_order: 0
---

# Project status
Finishing major rewrite after which 1.0 will be released.
After that it might be considered good enough for some simple apps.

## Known issues
- only one font (`Roboto`) is supported yet (no matter what font family you define)
  - only some glyphs are provided
- only `normal` font weight is supported yet
- windows platform is not supported yet

## Performance
There's still a lot of low hanging fruit but it's already fast enough to run on raspi 3.

Soon, it should be way faster than comparable
electron/web app simply because we don't have to support various web features
accumulated over the 30 ys history and because we also make some shortcuts
where it makes sense.

That said, the comparison is not fair and some things will take months/ys to get done good enough (accessibility, i18n)
