import * as React from 'react'
import { Window } from '..'
import { render } from '../src/react'

const color = { r: 1, g: 1, b: 1, a: 1 }
const w = new Window("Example")

const App = () => {
  const [count, setCount] = React.useState(1)

  React.useLayoutEffect(() => setTimeout(() => setCount(count + 1), 1000))

  return (
    <React.Fragment>
      {Array(count).fill().map((_, i) =>
        <Rect key={i} x={count} y={i * 15} w={10} h={10} />
      )}
    </React.Fragment>
  )
}

const Rect = ({ x, y, w, h }) =>
  <display-item type="Rect" info={{ rect: [[x, y], [w, h]], clip_rect: [[x, y], [w, h]], is_backface_visible: true }} color={color} />

render(<App />, w)

// prevent GC
global.window = w

/*import { Window } from '..'

const w = new Window("Hello")

const index = w.getGlyphIndices("H")[0]

w.sendFrame(JSON.stringify([
  { type: 'Rect', info: { rect: [[0, 0], [10, 10]], clip_rect: [[0, 0], [10, 10]], is_backface_visible: true }, color: { r: 1, g: 1, b: 1, a: 1 } },

  { type: 'Text', info: { rect: [[0, 0], [100, 100]], clip_rect: [[0, 0], [100, 100]], is_backface_visible: true }, color: { r: 1, g: 1, b: 1, a: 1 }, font_instance_key: [1, 2], glyphs: [[index, [10, 20]]] }
]))
