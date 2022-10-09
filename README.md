Work in progress, do not use.

# graffiti (prealpha MVP)

## Current scope
I've reduced the scope to something which I can finish myself in a reasonable time and which is also useful to me.
I have some ideas for the future (and lots of deleted code in a git history) but those will have to wait.

- [x] single-window, single-thread, single-script (global `document`), nodejs-only (N-API)
- [ ] subset of DOM needed by major JS frameworks (`preact`-only for now)
- [ ] subset of CSSOM for CSS-in-JS (`goober`-only for now)
- [ ] block/flexbox layout (no floats)
- [x] no index.html, no runtime behavior (no `<script>`, no `<link>`, ...)
- [x] no HMR, no live-reload (but `nodemon` works)
- [ ] publish to npm (until then, you can `npm i github:cztomsik/graffiti`)
- [ ] and even then, [it will be just a toy](https://www.cmyr.net/blog/gui-framework-ingredients.html)

## Goals & philosophy
- simplicity > number of features
- support "reasonable subset" of DOM/CSS so we don't need to learn anything new
- fit nicely into the existing node.js ecosystem (lib is better than framework)
- it has to be fun (for me, sorry)

## Hacking
- you will need Zig (`0.10.0-dev.4247+3234e8de3`)
- and system-installed GLFW3.3 with headers (`brew install glfw`)

```
git clone ...
cd ...
npm i
zig build
node examples/hello.js
```

## License
MIT

## Contributing
[Let's have a chat first](https://discord.gg/zQwyzFb). I will likely reject any unexpected MRs.
