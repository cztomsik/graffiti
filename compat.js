// Compat build (default) should support the "reasonable subset" of the DOM &
// CSSOM APIs. Roughly speaking, it should be enough for all the common JS
// frameworks and libraries to work.
//
// However, it's not a goal to support every browser feature and you still
// might need to use external polyfills for some of them.

// TODO: go through the previous code and decide what to restore
//       https://github.com/cztomsik/graffiti/tree/79577ee89314ba4d92e098ea7e11872cb5ded99d/src

import 'graffiti/core'

// load all polyfills
import './polyfills/innerHTML.js'
import './polyfills/storage.js'
