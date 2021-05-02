---
nav_order: 1
---
# Scope of the project
The point of this project is to provide platform for developing desktop and TV applications in a way which is familiar to web developers but not necessarily the same.

The table below provides a bit more info but in general:
  - you are expected to use **modern IDE**, preferrably with type-checking because we are doing very little validation
  - deprecated/exotic features are generally not supported at all
  - if it can be easily replaced with JS it's probably out-of-scope for now
  - popular libraries are expected to run fine eventually (if not already)
  - there's always `WebView` if you need it

## Notable limitations
The table below provides some info about what's left out. Some of those might be available in deno and/or provided by some external modules.

| General features    |           |
|---------------------|-----------|
| accessibility       | no, sorry |
| RTL, vertical langs | N/A       |
| printing, PDF       | N/A       |
| canvas, WebGL       | **TODO**  |
| SVG                 | **TODO**  |

| DOM                         |     |
|-----------------------------|-----|
| innerHTML                   | ✅  |
| innerText                   | N/A |
| custom elements, shadow DOM | N/A |
| (i)frames                   | N/A |

| CSS                     |                              |
|-------------------------|------------------------------|
| values                  | specified-only, no `inherit` |
| colors                  | hex, rgb(a), lowercase names |
| layout                  | `flex`, `block` emulation    |
| transitions             | later                        |
| CSSOM                   | minimal for CSS-in-JS        |
| media queries           | use JS                       |
