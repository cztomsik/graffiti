---
nav_order: 1
---

# Getting started

## Requirements
- node.js (preferrably v11)
- rust & cargo from [rustup](https://rustup.rs/)

### Debian/Ubuntu
- `sudo apt install clang xorg-dev`

### OSX/MacOs
- `xcode-select --install`

### Win
- [track here](https://github.com/cztomsik/node-webrender/issues/37), get in touch if you'd like to maintain windows-related part

## Getting started
```bash
# can take few minutes (add --verbose if paranoid)
# (npm is doing build 2 times when installing from github and it takes a while)
npm i github:cztomsik/graffiti
```

main.js
```js
document.body.appendChild(document.createTextNode('Hello'))
```

run with
```bash
node -r graffiti/register main.js
```

![image](https://user-images.githubusercontent.com/3526922/66957171-ff791800-f065-11e9-96c8-aea9eae84482.png)


## Starters
You can also clone one of these repos. Note that you still need to build native extension so the steps above apply too.
- https://github.com/cztomsik/hello-graffiti
- https://github.com/cztomsik/node-webrender-starter (outdated)
- https://github.com/cztomsik/slack-app (outdated)
- https://github.com/cztomsik/brew-cleaner (outdated)
