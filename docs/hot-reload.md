---
nav_order: 32
---

# Hot reload

- `npm install hot-module-replacement`
- have your `./main.tsx` file [like this](https://github.com/cztomsik/slack-app/blob/master/src/main.tsx#L13)
- create a `./hmr.js` file [like this](https://github.com/cztomsik/slack-app/blob/master/hmr.js)
- create a script entry in your `package.json` [like this](https://github.com/cztomsik/slack-app/blob/master/package.json#L7)
- run `npm run dev`
- do any changes and it should get reflected instantly
