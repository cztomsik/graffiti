import { Window } from '..'

const w = new Window('Hello from JS')
setInterval(() => w.redraw(), 100)

const createElement = tagName => ({
  nodeType: 1,
  tagName,
  hasAttribute: () => false,
  appendChild: () => undefined
})

global.document = {
  createElement,
  createComment: () => null,
  body: null, documentElement: null
}

global.window = {
  ...createElement('window'),
  document,
  navigator: { userAgent: '' }
}

const Vue = require('vue/dist/vue')
require.cache[require.resolve('vue')] = require.cache[require.resolve('vue/dist/vue')]

Object.assign(Vue.config, {
  isReservedTag: () => false
})

Vue.component('rect', {
  props: ['x', 'y', 'w', 'h'],
  template: '<op kind="rect" :xy="[x, y]" :wh="[w, h]" :color="[1, 0, 1]" />'
})

let frame = null

Vue.component('Renderer', {
  render(h) {
    console.log('re-render')
    frame = []
    return h('div', this.$slots.default)
  },

  updated() {
    console.log(JSON.stringify(frame))
    w.sendFrame(JSON.stringify(frame))
  }
})

Vue.component('op', {
  functional: true,
  render: (h, ctx) => !frame.push(ctx.props)
})
