---
nav_order: 0
---
# Scope of the project
The point of this project is to provide platform for developing desktop and TV applications in a way which is familiar to web developers but not necessarily the same. We **are not recreating** browser here, some of the features simply don't make sense for such apps and/or would have been very hard to implement.

Moreover, some features (innerHTML, cookies, frames) are in fact **undesired** and leaving them out **avoids some security concerns** you'd otherwise have to audit yourself (notably XSS & CSRF)

Note that just like in electron you can directly communicate with your database without exposing any server locally.

## Notable limitations
The table below provides some info about what's left out completely, what can be done with plain `nodejs` or with some 3rd party module from `npm`. Some of them might be introduced in the future but don't have any false hopes.

Note that you can always use/implement native module for anything.

| General features    |     |
|---------------------|-----|
| accessibility       | N/A |
| RTL, vertical langs | N/A |
| canvas, webgl       | N/A |
| SVG                 | N/A |
| HTML5 els           | N/A |
| style validations   | N/A |

| DOM               |     |
|-------------------|-----|
| Element.innerHTML | N/A |
| shadow DOM        | N/A |
| (i)frames         | N/A |
| referer, cookies  | N/A |
| window.open       | N/A |

| Styles                  |              |
|-------------------------|--------------|
| layout                  | flexbox-only |
| CSSOM                   | minimal      |
| word-break, white-space | N/A          |
| getComputedStyle()      | N/A          |

| Web API               |        |
|-----------------------|--------|
| XMLHttpRequest, fetch | nodejs |
| WebSocket             | nodejs |
| Web/Service Worker    | nodejs |
| crypto                | nodejs |
| IndexDB/WebSQL        | npm    |
| History               | N/A    |
| Video                 | N/A    |
| Audio                 | N/A    |
