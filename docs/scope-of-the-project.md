---
nav_order: 0
---
# Scope of the project
The point of this project is to provide platform for developing desktop and TV applications in a way which is familiar to web developers but not necessarily the same. We **are not recreating** browser here, some of the features simply don't make sense for such apps and/or would have been very hard to implement.

Note the complexity is **vastly** reduced, people often don't realize how different it is to develop web application vs. desktop one and how much baggage electron app needs to carry with itself (or to put it differently, how big is the cost of being able to turn existing web application into desktop app). To name just a few things you usually don't need:
  - tabs with different apps loaded at the same time & being to (really) quickly switch between them
  - excessive caching (images, fonts, styles, js files) & mem consumption to make that speed possible
  - history tracking, password management, autofill, synchronization, incognito mode
  - theming/user-provided styles, excessive media queries (phones/tablets)
  - user-agent/feature detection & polyfilling/transpiling
  - custom extensions, custom CSS/HTML/JS overrides for given sites, download manager
  - devtools, syntax highlighting, profiling
  - WebGL, canvas, SVG, XML parsing/styling
  - printing, print preview, saving to pdf
  - audio/video, including formats your OS can't play itself
  - url-bar, integration with search engines
  - cookies, local/session storage, indexeddb/websql, crypto, security mitigations
  - flash player
  - flawless i18n & accessibility, speech modulation/recognition

Moreover, some features (innerHTML, cookies, frames) are in fact **undesired** and leaving them out **avoids some security concerns** you'd otherwise have to audit yourself (notably XSS & CSRF). Note that you can directly communicate (require whatever you need) with your database without exposing any API server.

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
