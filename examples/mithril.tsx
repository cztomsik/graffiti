// node -r ts-node/register -r ./src/register.ts examples/mithril.tsx

/* @jsx m */
import * as m from 'mithril'

let count = 0

const App = {
  view: () =>
    <div style={styles.container} onclick={() => count++}>
      Hello {count}
    </div>
}

const styles = {
  container: {
    flex: 1,
    backgroundColor: '#ccc'
  }
}

m.mount(document.body, App)
